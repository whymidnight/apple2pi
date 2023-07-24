#![no_std]
#![no_main]

extern crate alloc;

use a2pi_rs::drivers::no_std::kb::input::A2PI_DESCRIPTOR;
use core::borrow::Borrow;
use core::{cell::RefCell, convert::Infallible};
use critical_section::Mutex;

use usb_device::class_prelude::*;
use usb_device::prelude::*;

use a2pi_rs::drivers::no_std::kb::driver::KbDriver;
use a2pi_rs::drivers::shared::kb::KeyboardDriver;
use cortex_m::prelude::_embedded_hal_timer_CountDown;
use cortex_m::singleton;
use embedded_alloc::Heap;
use fugit::{ExtU32, HertzU32, RateExtU32};
use hal::clocks::Clock;
use hal::dma::DMAExt;
use hal::uart::{DataBits, StopBits, UartConfig};
use hal::usb::UsbBus;
use panic_probe as _;
use usb_device::class::UsbClass;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use usbd_hid::hid_class::{
    HIDClass, HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig,
};

use rp2040_hal as hal;
use rp2040_hal::pac;
use rp2040_hal::pac::interrupt;

#[cfg(feature = "probe")]
use defmt_rtt as _;
#[cfg(feature = "serial")]
use defmt_serial as _;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[global_allocator]
static HEAP: Heap = Heap::empty();

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

static mut USB_BUS: Option<usb_device::bus::UsbBusAllocator<UsbBus>> = None;
static mut USB_DEVICE: Option<UsbDevice<'static, UsbBus>> = None;
static mut USB_HID: Option<HIDClass<'static, UsbBus>> = None;
static KEYBOARD_REPORT: Mutex<RefCell<KeyboardReport>> = Mutex::new(RefCell::new(KeyboardReport {
    modifier: 0,
    reserved: 0,
    leds: 0,
    keycodes: [0u8; 6],
}));

static mut KEY_PRESS_EVENT: [u8; 3] = [0x0; 3];

#[rp2040_hal::entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 100 * 1000;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    //
    // -- BEGIN PRELUDE --
    //
    let mut a2pi = KbDriver::init();

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    /*
    let ppb = &mut pac.PPB;
    unsafe {
        // Copy the vector table that cortex_m_rt produced into the RAM vector table
        RAM_VTABLE.init(ppb);
        RAM_VTABLE.register_handler(pac::Interrupt::USBCTRL_IRQ as usize, timer_irq0_replacement);
    }
    */

    let probe_uart = hal::uart::UartPeripheral::new(
        pac.UART0,
        (
            // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
            pins.gpio0.into_mode::<hal::gpio::FunctionUart>(),
            // UART RX (characters received by RP2040) on pin 2 (GPIO1)
            pins.gpio1.into_mode::<hal::gpio::FunctionUart>(),
        ),
        &mut pac.RESETS,
    )
    .enable(
        UartConfig::new(115200.Hz(), DataBits::Eight, None, StopBits::One),
        clocks.peripheral_clock.freq(),
    )
    .unwrap();
    #[cfg(feature = "serial")]
    defmt_serial::defmt_serial(probe_uart);
    defmt::info!("booting!!");
    //
    // -- END PRELUDE --
    //
    //
    //
    // -- BEGIN device INIT --
    //
    let hal_usb_bus = hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    );

    hal_usb_bus.remote_wakeup();
    let usb_bus = UsbBusAllocator::new(hal_usb_bus);
    unsafe {
        USB_BUS = Some(usb_bus);
    }
    let hid_endpoint = HIDClass::new_with_settings(
        unsafe { USB_BUS.as_ref().unwrap() },
        A2PI_DESCRIPTOR,
        1,
        HidClassSettings {
            subclass: HidSubClass::NoSubClass,
            protocol: HidProtocol::Keyboard,
            config: ProtocolModeConfig::ForceReport,
            locale: HidCountryCode::German,
        },
    );

    unsafe {
        USB_HID = Some(hid_endpoint);
    }

    let usb_device = UsbDeviceBuilder::new(
        unsafe { USB_BUS.as_ref().unwrap() },
        UsbVidPid(0x16c0, 0x27db),
    )
    .manufacturer("whymidnight")
    .product("Apple IIe Keyboard")
    // .serial_number("00123")
    .supports_remote_wakeup(true)
    .build();

    unsafe {
        USB_DEVICE = Some(usb_device);
    }

    let uart = hal::uart::UartPeripheral::new(
        pac.UART1,
        (
            // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
            pins.gpio4.into_mode::<hal::gpio::FunctionUart>(),
            // UART RX (characters received by RP2040) on pin 2 (GPIO1)
            pins.gpio5.into_mode::<hal::gpio::FunctionUart>(),
        ),
        &mut pac.RESETS,
    )
    .enable(
        UartConfig::new(
            HertzU32::from_raw(115200),
            DataBits::Eight,
            None,
            StopBits::One,
        ),
        clocks.peripheral_clock.freq(),
    )
    .unwrap();

    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };

    //
    // -- END device INIT --
    //
    //
    //
    // -- BEGIN dma INIT --
    //
    // eyes heavy, brain more fried than a chicken wang, 20 hours chewing glass, my moms def raised a slow dev /s... should wait ~2 seconds before syncing with the the most personal computer ever made

    let sync = singleton!(: [u8; 1] = [0x80]).unwrap();
    defmt::info!("sent resync");
    uart.write_full_blocking(sync);

    delay.delay_ms(100);
    let handshake = singleton!(: [u8; 1] = [0x81]).unwrap();
    uart.write_full_blocking(handshake);
    defmt::info!("sent handshake");
    //
    // -- END dma INIT --
    //
    //
    //
    // -- BEGIN MAIN --
    //
    let mut tick_count_down = timer.count_down();
    tick_count_down.start(1.millis());

    let (mut rx, tx) = uart.split();
    let mut dma = pac.DMA.split(&mut pac.RESETS);

    let mut press = false;
    loop {
        let mut rx_buf = unsafe { &mut KEY_PRESS_EVENT };
        let mut lch0 = dma.ch0;
        let mut lrx = rx;
        let mut lrx_buf = rx_buf;
        let key_press_event = hal::dma::single_buffer::Config::new(lch0, lrx, lrx_buf).start();
        while !key_press_event.is_done() {
            //
            // @required:
            //   `keyboard.tick()` is required every ms so whilst waiting for `key_press_event`
            //   to complete, we must keep ticking.
            //
            /*
            if tick_count_down.wait().is_ok() {}
             */
        }
        let (l_ch0, l_rx, l_rx_buf) = key_press_event.wait();
        lrx = l_rx;
        lch0 = l_ch0;
        lrx_buf = l_rx_buf;

        let payload = *lrx_buf;
        defmt::info!("[KEY_PRESS]: {=[u8]:#x} ::::: {:?}", payload, payload);

        if payload.iter().all(|&b| b == 0x80) {
            tx.write_full_blocking(&[0x81]);
            defmt::info!("resyncd");
        } else {
            let reports = a2pi.process_key_event(payload);
            if let Some(keyboard_reports) = reports {
                let num_reports = keyboard_reports.len();
                for (idx, &report) in keyboard_reports.iter().enumerate() {
                    critical_section::with(|cs| {
                        defmt::info!("writing report...");
                        KEYBOARD_REPORT.replace(cs, report);
                    });
                    // unless a case exists where there are multiple reports to be
                    // emitted, do not incur a delay.
                    if idx != num_reports {
                        defmt::info!("delaying next report...");
                        delay.delay_ms(10);
                    }
                }
            }

            /*
            match press {
                // key up
                true => {
                    critical_section::with(|cs| {
                        KEYBOARD_REPORT.replace(
                            cs,
                            KeyboardReport {
                                modifier: 0,
                                reserved: 0,
                                leds: 0,
                                keycodes: [0u8; 6],
                            },
                        );
                    });

                    press = false;
                }
                // key down
                false => {
                    critical_section::with(|cs| {
                        KEYBOARD_REPORT.replace(
                            cs,
                            KeyboardReport {
                                modifier: 0,
                                reserved: 0,
                                leds: 0,
                                keycodes: [0x04, 0x0, 0x0, 0x0, 0x0, 0x0],
                            },
                        );
                    });
                    press = true;
                }
            }
            */
        }

        rx = lrx;
        dma.ch0 = lch0;
        rx_buf = lrx_buf;
    }
    //
    // -- END MAIN --
    //
}

// @sauce: https://github.com/bschwind/key-ripper/blob/576e5d1b99436b6539302fa41861851fd24ff004/firmware/src/main.rs#L217
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();

    if usb_dev.poll(&mut [usb_hid]) {
        usb_hid.poll();
    }

    let report = critical_section::with(|cs| *KEYBOARD_REPORT.borrow_ref(cs));
    if let Err(_err) = usb_hid.push_input(&report) {
        let _no_op = 0;
    }

    // macOS doesn't like it when you don't pull this, apparently.
    // TODO: maybe even parse something here
    usb_hid.pull_raw_output(&mut [0; 64]).ok();

    // Wake the host if a key is pressed and the device supports
    // remote wakeup.
    if !report_is_empty(&report)
        && usb_dev.state() == UsbDeviceState::Suspend
        && usb_dev.remote_wakeup_enabled()
    {
        usb_dev.bus().remote_wakeup();
    }
}

fn report_is_empty(report: &KeyboardReport) -> bool {
    report.modifier != 0 || report.keycodes.iter().any(|key| *key != 0x0u8)
}

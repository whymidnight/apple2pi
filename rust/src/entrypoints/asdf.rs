#![no_std]
#![no_main]

extern crate alloc;

use hal::clocks::Clock;
use hal::vector_table::VectorTable;
use panic_probe as _;

use rp2040_hal as hal;
use rp2040_hal::pac;
use rp2040_hal::pac::interrupt;

use defmt_serial as _;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

static mut RAM_VTABLE: VectorTable = VectorTable::new();

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[interrupt)]
extern "C" fn timer_irq0_replacement() -> ! {}

#[rp2040_hal::entry]
fn main() -> ! {
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
    let ppb = &mut pac.PPB;
    unsafe {
        // Copy the vector table that cortex_m_rt produced into the RAM vector table
        RAM_VTABLE.init(ppb);
        RAM_VTABLE.register_handler(pac::Interrupt::USBCTRL_IRQ as usize, timer_irq0_replacement);
    }

    //
    // -- END PRELUDE --
    //
    //
    //
    // -- BEGIN device INIT --
    //
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };
    unsafe {
        critical_section::with(|_| {
            RAM_VTABLE.activate(ppb);
        });
    }
    loop {
        delay.delay_ms(10);
    }
    //
    // -- END MAIN --
    //
}

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // Grab the global objects. This is OK as we only access them under interrupt.
    critical_section::with(|cs| {});
}

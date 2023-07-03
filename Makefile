PACKAGE=a2pi-rs
VERSION=0.2.3
DESTDIR=/usr/local
SHAREDIR=$(DESTDIR)/share/a2pi
SBIN=./target/release/a2pi-rs

a2pi-rs:
	cargo build --release --bin a2pi-rs

clean:
	-rm -rf ./target

install:
	sudo systemctl stop a2pi | true
	mkdir -p $(SHAREDIR)
	cp $(SBIN) $(SHAREDIR)
	cp -R ./share/* $(SHAREDIR)
	cp -R ./rust/a2pi_keymaps $(SHAREDIR)
	sudo systemctl enable $(SHAREDIR)/a2pi.service
	sudo systemctl daemon-reload

start:
	sudo systemctl start a2pi

restart:
	sudo systemctl restart a2pi

reload:
	sudo systemctl daemon-reload


PACKAGE=a2pi-rs
VERSION=0.2.3
DESTDIR=/usr/local
HAREDIR=$(DESTDIR)/share/a2pi)
SBIN=./target/release/a2pi-rs

a2pi-rs:
	cargo build --release --bin a2pi-rs

clean:
	-rm -rf ./target

install:
	sudo systemctl stop a2pi | true
	-mkdir -p $(SHAREDIR)
	cp $(SBIN) $(SHAREDIR)
	cp -R ./share/* $(SHAREDIR)
	sudo systemctl enable --system $(SHAREDIR)/a2pi.service
	sudo systemctl daemon-reload

start:
	sudo systemctl start $(SHAREDIR)/a2pi.service

restart:
	sudo systemctl restart $(SHAREDIR)/a2pi.service

reload:
	sudo systemctl daemon-reload


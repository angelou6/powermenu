.PHONY: build install local_install

BIN = powermenu
FILES = src/main.vala src/config.vala
PACKAGES = --pkg gtk4 --pkg posix --pkg gtk4-layer-shell-0
PREFIX ?= /usr/local

build_debug:
	valac $(PACKAGES) $(FILES) --output BIN

build:
	valac -X -O2 -X -DNDEBUG -X -s --disable-assert $(PACKAGES) $(FILES) --output powermenu

install: build
	install -Dm755 powermenu $(PREFIX)/bin/powermenu

clean:
	rm $(OUT)

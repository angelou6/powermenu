.PHONY: build install local_install

OUT = powermenu
FILES = src/main.vala src/config.vala
PACKAGES = --pkg gtk4 --pkg posix --pkg gtk4-layer-shell-0


build_debug:
	valac $(PACKAGES) $(FILES) --output powermenu

build:
	valac -X -O2 -X -DNDEBUG -X -s --disable-assert $(PACKAGES) $(FILES) --output powermenu

install: build
	install -Dm755 powermenu /usr/local/bin/powermenu

local_install: build
	install -Dm755 powermenu ~/.local/bin/powermenu

clean:
	rm $(OUT)

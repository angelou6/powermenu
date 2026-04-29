# Powermenu

## How to compile

Install the dependencies.

Arch Linux
```sh
sudo pacman -S gtk4-layer-shell vala base-devel gtk4
```

To install on /usr/local/bin/
```sh
sudo make install
```

or to install on ~/.local/bin/
```sh
make local_install
```

## help
```
Usage:
  powermenu [OPTION…] A simple powermenu

Help Options:
  -h, --help            Show help options

Application Options:
  -c, --config=PATH     Location of config.ini
  -s, --styles=PATH     Location of style.css

```

# Powermenu
![powermenu image](https://raw.githubusercontent.com/angelou6/powermenu/refs/heads/master/example-config/readme_image.png)

## How to compile

Arch Linux
```sh
sudo pacman -S gtk4-layer-shell vala base-devel gtk4
```

`gtk4-layer-shell` is a dependency for the application.

## Instalation

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

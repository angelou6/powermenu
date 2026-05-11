# Powermenu

![powermenu image](https://raw.githubusercontent.com/angelou6/powermenu/refs/heads/master/example-config/readme_image.png)

## How to compile

Arch Linux

```sh
sudo pacman -S gtk4-layer-shell vala base-devel gtk4
```

`gtk4-layer-shell` is a dependency for powermenu.

## Instalation

To install on /usr/local/bin/

```sh
sudo make install
```

or to install on ~/.local/bin/

```sh
make local_install
```

## Configuration

The configuration needs to be located in `.config/powermenu/`.

You can find an example config in [/example-config](/example-config)

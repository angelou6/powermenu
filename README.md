# Powermenu

![powermenu image](/readme_image.png)

## Instalation

First install the dependencies.

**Arch Linux:**

```sh
sudo pacman -S gtk4-layer-shell vala base-devel gtk4
```

`gtk4-layer-shell` is a dependency for powermenu.

To install on /usr/local/bin/

```sh
sudo make install
```

or to install on ~/.local/bin/

```sh
make install PREFIX=~/.local
```

## Configuration

The configuration needs to be located in `~/.config/powermenu/`.

You can find an example config in [/example-config](example-config)

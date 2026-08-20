# Powermenu

![powermenu image](/readme_image.png)

## Instalation

First install the build dependencies.

**Arch Linux:**

```sh
sudo pacman -S gtk4-layer-shell base-devel gtk4 libadwaita meson desktop-file-utils gcc
```

**OpenSUSE Tumbleweed:**

```sh
sudo zypper in gtk4-devel libadwaita-devel meson vala gtk4-layer-shell-devel desktop-file-utils
```

`gtk4-layer-shell` is necesary for powermenu to work.

## Configuration

The configuration needs to be located in `~/.config/powermenu/`.

You can find an example config in [/example-config](example-config)

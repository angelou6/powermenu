# Powermenu

![powermenu image](/readme_image.png)

## Instalation

`gtk4-layer-shell` is a necessary dependency for powermenu.

### Requirements

**Arch Linux:**

```sh
sudo pacman -S gtk4-layer-shell base-devel gtk4 libadwaita meson desktop-file-utils gcc
```

**OpenSUSE Tumbleweed:**

```sh
sudo zypper in gtk4-devel libadwaita-devel meson gtk4-layer-shell-devel desktop-file-utils
```

## Configuration

You can find an example config in [/example-config](example-config)
The configuration needs to be located in `~/.config/powermenu/`.

The path for configuration and css can also be changed using the following flags:

```
-c, --config              Path to config file
-s, --css                 Path to css file
-v, --vertical            Powermenu vertical mode
```

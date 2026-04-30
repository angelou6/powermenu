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

`config.ini`
```ini
icon=⏻
command=poweroff

[reboot]
icon=↻
command=reboot

[suspend]
icon=Z
command=systemctl suspend

[logout]
icon=⍈
command=hyprctl dispatch exit 0
```

You can style powermenu using css.

`style.css`
```css
:root {
  --bg: #1d2021;
  --bg2: #3c3836;
  --fg: #ebdbb2;
  --cyan: #8ec07c;
  --red: #cc241d;
  --green: #98971a;
  --blue: #458588;
}

window {
  /* border-radius: 10px; */
  background: rgba(0,0,0, 0.0);
}

button {
  margin: 10px;
  border: 1px solid var(--cyan);
  background: var(--bg);
  font-size: 2em;
  font-family: "Ubuntu Nerd Font Propo Med";
}

.shutdown { color: var(--red); }
.reboot   { color: var(--fg); }
.suspend  { color: var(--green); }
.logout   { color: var(--blue); }

button:hover, button:focus {
  background-image: none;
  background-color: var(--bg2);
  color: var(--fg);
}
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

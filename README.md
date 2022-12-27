# AFP
A fast sysfetch program inspired by [Archey3](https://github.com/lclarkmichalek/archey3) and [Archey4](https://github.com/HorlogeSkynet/archey4) writen in rust with 28 dependencies.

![Screenshot](https://github.com/Slimemaster0/afp/blob/main/screenshots/1.png)

## Faq (Probably)
- Q: What does **AFP** stand for?
- A: Another Fetch Program

## Configuration
AFP is configured in json.
The configuration is placed in 1 of 2 places.
1. /etc/afp/config.json
2. Depending on if $XDG_CONFIG_HOME is set of not. ( Usually $XDG_CONFIG_HOME isn't set or is set to $HOME/.config )
  Yes -> $XDG_CONFIG_HOME/afp/config.json
  No  -> $HOME/.config/afp/config.json

If no config file is found, then AFP will use the default configuration.

The default config:
```json
{
    "logo": "auto",
    "items": [
        "user host",
        "session type",
        "distro",
        "kernel",
        "device",
        "vendor",
        "ram",
        "editor",
        "shell",
        "cpu",
        "de"
    ]
}
```

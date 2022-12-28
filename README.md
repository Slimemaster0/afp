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
        {
            "mod_type": "builtin mod",
            "mod_name": "user host",
            "mod_title": ""
        },
        {
            "mod_type": "env_var",
            "mod_name": "XDG_SESSION_TYPE",
            "mod_title": "Session Type: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "distro",
            "mod_title": "Distro: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "kernel",
            "mod_title": "Kernel: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "device",
            "mod_title": "Device: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "vendor",
            "mod_title": "Vendor: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "ram",
            "mod_title": "Memory: "
        },
        {
            "mod_type": "env_var",
            "mod_name": "EDITOR",
            "mod_title": "Editor: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "shell",
            "mod_title": "Shell: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "cpu",
            "mod_title": "CPU: "
        },
        {
            "mod_type": "env_var",
            "mod_name": "XDG_CURRENT_DESKTOP",
            "mod_title": "DE: "
        }
    ]
}
```

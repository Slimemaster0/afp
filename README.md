# AFP
A fast sysfetch program inspired by [Archey3](https://github.com/lclarkmichalek/archey3) and [Archey4](https://github.com/HorlogeSkynet/archey4) writen in rust with 28 dependencies.

![Screenshot](https://github.com/Slimemaster0/afp/blob/main/screenshots/1.png)

## Faq (Probably)
- Q: What does **AFP** stand for?
- A: Another Fetch Program

## Configuration
AFP is configured in json.
The configuration file is placed in 1 of 2 places.
1. /etc/afp/config.json
2. Depending on if $XDG_CONFIG_HOME is set of not. ( Usually $XDG_CONFIG_HOME isn't set or is set to $HOME/.config )

  Yes -> $XDG_CONFIG_HOME/afp/config.json
  
  No  -> $HOME/.config/afp/config.json

If no config file is found, then AFP will use the default configuration.

There is a "command" module that uses the first argument as the command and the rest as arguments.

An example where you run the command "foo" with "--bar" as an argument.
```json
...
        {
	    "module": "command",
	    "args": [ "foo", "--bar" ],
	    "title": "Foo: "
	},
...
```

The default config:
```json
{
    "logo": "auto",
    "color": "None",
    "items": [
        {
            "module": "user host",
            "args": [],
            "title": "",
            "color": "None"
        },
        {
            "module": "env_var",
            "args": [ "XDG_SESSION_TYPE" ],
            "title": "Session Type: ",
            "color": "None"
        },
        {
            "module": "distro",
            "args": [],
            "title": "Distro: ",
            "color": "None"
        },
        {
            "module": "kernel",
            "args": [],
            "title": "Kernel: ",
            "color": "None"
        },
        {
            "module": "device",
            "args": [],
            "title": "Device: ",
            "color": "None"
        },
        {
            "module": "vendor",
            "args": [],
            "title": "Vendor: ",
            "color": "None"
        },
        {
            "module": "ram",
            "args": [],
            "title": "Memory: ",
            "color": "None"
        },
        {
            "module": "env_var",
            "args": [ "EDITOR" ],
            "title": "Editor: ",
            "color": "None"
        },
        {
            "module": "shell",
            "args": [],
            "title": "Shell: ",
            "color": "None"
        },
        {
            "module": "cpu",
            "args": [],
            "title": "CPU: ",
            "color": "None"
        },
        {
            "module": "env_var",
            "args": [ "XDG_CURRENT_DESKTOP" ],
            "title": "DE: ",
            "color": "None"
        }
    ]
}
```

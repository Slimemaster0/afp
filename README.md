# AFP
A fast sysfetch program inspired by [Archey3](https://github.com/lclarkmichalek/archey3) and [Archey4](https://github.com/HorlogeSkynet/archey4) writen in rust with 56 dependencies.

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

There is a "Command" module and a "LineCount" that is configured in the same way.

An example where you run the command "foo" with "--bar" and "69" as arguments.
```json
...
        {
	    "Command": {
	    	"command": "foo",
	        "args": [ "--bar", "69" ],
	        "title": "Foo: "
	    }
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
            "UserHost": {
                "title": "",
                "color": "None"
            }
        },
        {
            "EnvVar": {
                "var": "XDG_SESSION_TYPE",
                "title": "Session Type: ",
                "color": "None"
            }
        },
        {
            "Distro": {
                "title": "Distro: ",
                "color": "None"
            }
        },
        {
            "Kernel": {
                "title": "Kernel: ",
                "color": "None"
            }
        },
        {
            "Device": {
                "title": "Device: ",
                "color": "None"
            }
        },
        {
            "Vendor": {
                "title": "Vendor: ",
                "color": "None"
            }
        },
        {
            "RAM": {
                "title": "Memory: ",
                "color": "None"
            }
        },
        {
            "EnvVar": {
                "var": "EDITOR",
                "title": "Editor: ",
                "color": "None"
            }
        },
        {
            "Shell": {
                "title": "Shell: ",
                "color": "None"
            }
        },
        {
            "GPU": {
                "title": "GPU$: ",
                "color": "None",
                "brand": true
            }
        },
        {
            "CPU": {
                "title": "CPU: ",
                "color": "None"
            }
        },
        {
            "EnvVar": {
                "var": "XDG_CURRENT_DESKTOP",
                "title": "DE: ",
                "color": "None"
            }
        }
    ]
}
```

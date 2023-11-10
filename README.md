# ARMORED CORE™ VI FIRES OF RUBICON™ free-camera with extras I guess

This is a mod that allows people to reenable the AC6 developer freecam, disable UI rendering and even slow-down and speed-up the games time. This mod was originally made for VaatiVidya and released with their approval.

Current version is for 1.03.1 exclusively. I am also not sure how fast I will be with maintaining this.

## How to use this

Inject the DLL into the game while AC6 is running without EAC enabled.
```
// TODO: write step-by-step
```

Then, in the games directory, create a file called `freecam.toml`. This TOML file dictates the keybinds.

An example freecam.toml looks like this:
```toml
[[keybinds]]
key = "F"
[keybinds.command]
command = "ToggleFreecam"

[[keybinds]]
key = "H"
[keybinds.command]
command = "ToggleHUD"

[[keybinds]]
key = "T"
[keybinds.command]
command = "ToggleTime"

[[keybinds]]
key = "1"
[keybinds.command]
command = "SetTimeMultiplier"
multiplier = 0.1

[[keybinds]]
key = "5"
[keybinds.command]
command = "SetTimeMultiplier"
multiplier = 0.5

[[keybinds]]
key = "P"
[keybinds.command]
command = "SetTimeMultiplier"
multiplier = 0.0
```

Where `key` specifies the key press that should trigger a given `command`.

## Triggerable commands
| Command                  | Description                                                                                  |
|--------------------------|----------------------------------------------------------------------------------------------|
| ToggleHUD                | Toggles scaleform rendering. Effectively toggles the entire UI.                              |
| ToggleTimeControl        | Slows down or speeds up the game based on SetTimeMultiplier input.                           |
| ToggleFreecam            | Enables the developer debug free cam. Usable only with gamepad.                              |
| SetTimeMultiplier        | Determines how time is affected when time control is enable. 1.0 = normal speed, 0.5 = half. |
| SetCameraSpeedMultiplier | Determines the camera movement and rotation speed. Useful for panning shots I guess?         |

## Available keys

```
// TODO: write a proper table or something?
```

## Thanks!
- Dasaav for pointing me to the camera movement vftable

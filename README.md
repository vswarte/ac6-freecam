# ARMORED CORE™ VI FIRES OF RUBICON™ free-camera with extras I guess

This is a mod that allows people to reenable the AC6 developer freecam,
disable UI rendering and even slow-down and speed-up the game's time.
This mod was originally made for VaatiVidya and released with their approval.

Current version is for 1.03.1 through 1.04.1 exclusively.

## Installation steps

Inject the DLL into the game while AC6 is running without EAC enabled. To
inject the DLL into the game you have a few distinct options. These are
described below and you should only apply one of the approaches.

### ModEngine2
This is probably the best way if you're also running other mods and have ME2 
already setup. Place the DLL in the folder that contains the
`config_armoredcore6.toml`, and edit the `external_dlls` line to state:
```toml
external_dlls = ["ac6_freecam.dll"]
```

### ChurchGuard's lazy loader
The most straight forward way is using
[ChurchGuard's lazy loader DLL](https://www.nexusmods.com/darksouls3/mods/677).
Once that's set up you add the DLL to the dllMods folder as described in lazy
loader's read-me.

## Configuration
Then, in the games directory (the folder that contains the `armoredcore6.exe`),
create a file called `freecam.toml`. This TOML file dictates the keybinds.

An example freecam.toml looks like this:
```toml
# Toggles the freecam on and off
[[keybinds]]
key = "F"
[keybinds.command]
command = "ToggleFreecam"

# Toggles the HUD
[[keybinds]]
key = "H"
[keybinds.command]
command = "ToggleHUD"

# Toggles time control
[[keybinds]]
key = "T"
[keybinds.command]
command = "ToggleTimeControl"

# Sets the game speed to 10% of its original
[[keybinds]]
key = "1"
[keybinds.command]
command = "SetTimeMultiplier"
multiplier = 0.1

# Sets the game speed to 50% of its original
[[keybinds]]
key = "5"
[keybinds.command]
command = "SetTimeMultiplier"
multiplier = 0.5

# Sets the game speed to its original pace
[[keybinds]]
key = "0"
[keybinds.command]
command = "SetTimeMultiplier"
multiplier = 1.0

# Sets the game speed to 0.001% of its original.
# Don't use 0.0 because I am dividing with this.
[[keybinds]]
key = "P"
[keybinds.command]
command = "SetTimeMultiplier"
multiplier = 0.001

# Sets the camera movement to its original speed
[[keybinds]]
key = "C"
[keybinds.command]
command = "SetCameraSpeedMultiplier"
multiplier = 1.0

# Sets the camera movement to 10 times its original speed
[[keybinds]]
key = "8"
[keybinds.command]
command = "SetCameraSpeedMultiplier"
multiplier = 10.0
```

The `key` specifies the key press that should trigger a given `command`.
A full list all supported keys can be found [here](keys.txt).

## Triggerable commands
| Command                  | Description                                                                                  |
|--------------------------|----------------------------------------------------------------------------------------------|
| ToggleHUD                | Toggles scaleform rendering. Effectively toggles the entire UI.                              |
| ToggleTimeControl        | Slows down or speeds up the game based on SetTimeMultiplier input.                           |
| ToggleFreecam            | Enables the developer debug free cam. Usable only with gamepad.                              |
| SetTimeMultiplier        | Determines how time is affected when time control is enable. 1.0 = normal speed, 0.5 = half. |
| SetCameraSpeedMultiplier | Determines the camera movement and rotation speed. Useful for panning shots I guess?         |

## Thanks!
- Dasaav for pointing me to the camera movement vftable ([give this man your money](https://ko-fi.com/dasaav), he is the true hero behind this mod).

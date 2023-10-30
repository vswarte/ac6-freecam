use std::time;
use std::sync;
use std::collections;
use windows::Win32::UI::Input::KeyboardAndMouse;

use crate::config;

const DEBOUNCE_MS: u64 = 250;

type DebounceMap = collections::HashMap<i32, time::Instant>;
static mut DEBOUNCE_MAP: sync::OnceLock<DebounceMap> = sync::OnceLock::new();

fn init_debounce_map() -> DebounceMap {
    Default::default()
}

type KeybindMap = collections::HashMap<i32, config::KeybindCommand>;
static mut KEYBIND_MAP: sync::OnceLock<KeybindMap> = sync::OnceLock::new();

fn init_keybind_map() -> KeybindMap {
    Default::default()
}

pub unsafe fn get_pending_commands() -> Vec<&'static config::KeybindCommand> {
    let mut commands = vec![];

    let mappings = KEYBIND_MAP.get_or_init(init_keybind_map).iter();
    for mapping in mappings {
        if is_key_pressed(mapping.0.clone()) {
            commands.push(mapping.1);
        }
    }

    commands
}

pub unsafe fn set_keybinds(bindings: Vec<config::ConfigKeybind>) {
    let _mappings = KEYBIND_MAP.get_or_init(init_keybind_map);

    let mappings = KEYBIND_MAP.get_mut().unwrap();
    for binding in bindings {
        let vk = map_keys_to_vk(binding.key.as_str())
            .expect("Error setting up keybinds");

        mappings.insert(vk, binding.command);
    }
}

pub unsafe fn is_key_pressed(key: i32) -> bool {
    let debounce_time = time::Duration::from_millis(DEBOUNCE_MS);

    if KeyboardAndMouse::GetKeyState(key) < 0 {
        if let Some(last_pressed) = DEBOUNCE_MAP.get_or_init(init_debounce_map).get(&key) {
            if last_pressed.elapsed() > debounce_time {
                DEBOUNCE_MAP.get_mut().unwrap().remove(&key);
                DEBOUNCE_MAP.get_mut().unwrap().insert(key, time::Instant::now());
                return true;
            } else {
                return false;
            }
        } else {
            DEBOUNCE_MAP.get_mut().unwrap().insert(key, time::Instant::now());
            return true;
        }
    }

    false
}

#[derive(Debug)]
pub enum KeymappingError {
    UnknownKey(String),
}

pub fn map_keys_to_vk(key: &str) -> Result<i32, KeymappingError> {
    let vk = match key {
        "LBUTTON" => 0x01,
        "RBUTTON" => 0x02,
        "CANCEL" => 0x03,
        "MBUTTON" => 0x04,
        "XBUTTON1" => 0x05,
        "XBUTTON2" => 0x06,
        "BACK" => 0x08,
        "TAB" => 0x09,
        "CLEAR" => 0x0C,
        "RETURN" => 0x0D,
        "SHIFT" => 0x10,
        "CONTROL" => 0x11,
        "MENU" => 0x12,
        "PAUSE" => 0x13,
        "CAPITAL" => 0x14,
        "KANA" => 0x15,
        "HANGUEL" => 0x15,
        "HANGUL" => 0x15,
        "JUNJA" => 0x17,
        "FINAL" => 0x18,
        "HANJA" => 0x19,
        "KANJI" => 0x19,
        "ESCAPE" => 0x1B,
        "CONVERT" => 0x1C,
        "NONCONVERT" => 0x1D,
        "ACCEPT" => 0x1E,
        "MODECHANGE" => 0x1F,
        "SPACE" => 0x20,
        "PRIOR" => 0x21,
        "NEXT" => 0x22,
        "END" => 0x23,
        "HOME" => 0x24,
        "LEFT" => 0x25,
        "UP" => 0x26,
        "RIGHT" => 0x27,
        "DOWN" => 0x28,
        "SELECT" => 0x29,
        "PRINT" => 0x2A,
        "EXECUTE" => 0x2B,
        "SNAPSHOT" => 0x2C,
        "INSERT" => 0x2D,
        "DELETE" => 0x2E,
        "HELP" => 0x2F,
        "0" => 0x30,
        "1" => 0x31,
        "2" => 0x32,
        "3" => 0x33,
        "4" => 0x34,
        "5" => 0x35,
        "6" => 0x36,
        "7" => 0x37,
        "8" => 0x38,
        "9" => 0x39,
        "A" => 0x41,
        "B" => 0x42,
        "C" => 0x43,
        "D" => 0x44,
        "E" => 0x45,
        "F" => 0x46,
        "G" => 0x47,
        "H" => 0x48,
        "I" => 0x49,
        "J" => 0x4A,
        "K" => 0x4B,
        "L" => 0x4C,
        "M" => 0x4D,
        "N" => 0x4E,
        "O" => 0x4F,
        "P" => 0x50,
        "Q" => 0x51,
        "R" => 0x52,
        "S" => 0x53,
        "T" => 0x54,
        "U" => 0x55,
        "V" => 0x56,
        "W" => 0x57,
        "X" => 0x58,
        "Y" => 0x59,
        "Z" => 0x5A,
        "LWIN" => 0x5B,
        "RWIN" => 0x5C,
        "APPS" => 0x5D,
        "SLEEP" => 0x5F,
        "NUMPAD0" => 0x60,
        "NUMPAD1" => 0x61,
        "NUMPAD2" => 0x62,
        "NUMPAD3" => 0x63,
        "NUMPAD4" => 0x64,
        "NUMPAD5" => 0x65,
        "NUMPAD6" => 0x66,
        "NUMPAD7" => 0x67,
        "NUMPAD8" => 0x68,
        "NUMPAD9" => 0x69,
        "MULTIPLY" => 0x6A,
        "ADD" => 0x6B,
        "SEPARATOR" => 0x6C,
        "SUBTRACT" => 0x6D,
        "DECIMAL" => 0x6E,
        "DIVIDE" => 0x6F,
        "F1" => 0x70,
        "F2" => 0x71,
        "F3" => 0x72,
        "F4" => 0x73,
        "F5" => 0x74,
        "F6" => 0x75,
        "F7" => 0x76,
        "F8" => 0x77,
        "F9" => 0x78,
        "F10" => 0x79,
        "F11" => 0x7A,
        "F12" => 0x7B,
        "F13" => 0x7C,
        "F14" => 0x7D,
        "F15" => 0x7E,
        "F16" => 0x7F,
        "F17" => 0x80,
        "F18" => 0x81,
        "F19" => 0x82,
        "F20" => 0x83,
        "F21" => 0x84,
        "F22" => 0x85,
        "F23" => 0x86,
        "F24" => 0x87,
        "NUMLOCK" => 0x90,
        "SCROLL" => 0x91,
        "LSHIFT" => 0xA0,
        "RSHIFT" => 0xA1,
        "LCONTROL" => 0xA2,
        "RCONTROL" => 0xA3,
        "LMENU" => 0xA4,
        "RMENU" => 0xA5,
        "BROWSER_BACK" => 0xA6,
        "BROWSER_FORWARD" => 0xA7,
        "BROWSER_REFRESH" => 0xA8,
        "BROWSER_STOP" => 0xA9,
        "BROWSER_SEARCH" => 0xAA,
        "BROWSER_FAVORITES" => 0xAB,
        "BROWSER_HOME" => 0xAC,
        "VOLUME_MUTE" => 0xAD,
        "VOLUME_DOWN" => 0xAE,
        "VOLUME_UP" => 0xAF,
        "MEDIA_NEXT_TRACK" => 0xB0,
        "MEDIA_PREV_TRACK" => 0xB1,
        "MEDIA_STOP" => 0xB2,
        "MEDIA_PLAY_PAUSE" => 0xB3,
        "LAUNCH_MAIL" => 0xB4,
        "LAUNCH_MEDIA_SELECT" => 0xB5,
        "LAUNCH_APP1" => 0xB6,
        "LAUNCH_APP2" => 0xB7,
        "OEM_1" => 0xBA,
        "OEM_PLUS" => 0xBB,
        "OEM_COMMA" => 0xBC,
        "OEM_MINUS" => 0xBD,
        "OEM_PERIOD" => 0xBE,
        "OEM_2" => 0xBF,
        "OEM_3" => 0xC0,
        "OEM_4" => 0xDB,
        "OEM_5" => 0xDC,
        "OEM_6" => 0xDD,
        "OEM_7" => 0xDE,
        "OEM_8" => 0xDF,
        "OEM_102" => 0xE2,
        "PROCESSKEY" => 0xE5,
        "PACKET" => 0xE7,
        "ATTN" => 0xF6,
        "CRSEL" => 0xF7,
        "EXSEL" => 0xF8,
        "EREOF" => 0xF9,
        "PLAY" => 0xFA,
        "ZOOM" => 0xFB,
        "PA1" => 0xFD,
        "OEM_CLEAR" => 0xFE,
        _ => return Err(KeymappingError::UnknownKey(key.to_string())),
    };

    Ok(vk)
}

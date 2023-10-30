#![feature(absolute_path)]

use std::mem;

mod util;
mod game;
mod input;
mod config;

use broadsword::dll;
use detour::static_detour;

static_detour! {
    // Function that seems to update the delta time between frames. Patching the values seemingly
    // affects all simulation, from VFX timelines to gravity.
    static FLIPPER_UPDATE_HOOK: fn(*mut game::CSFlipperImp);

    // Function that seems to flush scaleform framebuffers onto the final, rendered, framebuffer.
    static SCALEFORM_UPDATE_B_HOOK: fn();

    // Seemingly a getter for the freecam debug option
    static FREECAM_DEBUG_FLAG_HOOK: fn() -> bool;
}

#[dll::entrypoint]
unsafe fn entry(_: usize) -> bool {
    broadsword::logging::init("logs/ac6_freecam.log");
    unsafe { initialize_hooks(); }

    // Read config and bind keys
    let config = config::get_config();
    input::set_keybinds(config.keybinds);

    std::thread::spawn(|| {
        loop {
            let commands = input::get_pending_commands();
            if commands.len() != 0 {
                log::debug!("Queued commands: {:#?}", commands);
            }

            for command in commands {
                match command {
                    config::KeybindCommand::ToggleHUD => toggle_hud(),
                    config::KeybindCommand::ToggleTimeControl => toggle_timecontrol(),
                    config::KeybindCommand::ToggleFreecam => toggle_freecam(),
                    config::KeybindCommand::SetTimeMultiplier{multiplier} => game::GLOBAL_TIME_MULTIPLIER = multiplier.clone(),
                    _ => todo!("Implement this command :fatcat:"),
                }
            }
        }
    });

    return true;
}

unsafe fn initialize_hooks() {
    FLIPPER_UPDATE_HOOK.initialize(
        mem::transmute(game::POINTER_FLIPPER_UPDATE),
        |flipper: *mut game::CSFlipperImp| {
            FLIPPER_UPDATE_HOOK.call(flipper);

            (*flipper).delta_time = (*flipper).delta_time * game::GLOBAL_TIME_MULTIPLIER;
        }
    ).unwrap();

    SCALEFORM_UPDATE_B_HOOK.initialize(
        mem::transmute(game::POINTER_SCALEFORM_UPDATE_B),
        || { }
    ).unwrap();

    FREECAM_DEBUG_FLAG_HOOK.initialize(
        mem::transmute(game::POINTER_FREECAM_DEBUG_FLAG),
        || true
    ).unwrap();
}

unsafe fn toggle_hud() {
    if !SCALEFORM_UPDATE_B_HOOK.is_enabled() {
        log::debug!("toggle_hud(): disabling scaleform rendering");
        SCALEFORM_UPDATE_B_HOOK.enable().unwrap();
    } else {
        log::debug!("toggle_hud(): enabling scaleform rendering");
        SCALEFORM_UPDATE_B_HOOK.disable().unwrap();
    }
}

unsafe fn toggle_freecam() {
    if !FREECAM_DEBUG_FLAG_HOOK.is_enabled() {
        if set_freecam_enable(0x1) {
            log::debug!("toggle_freecam(): enabling freecam");
            FREECAM_DEBUG_FLAG_HOOK.enable().unwrap();

            // TODO: copy coordinates
        }
    } else {
        if set_freecam_enable(0x0) {
            log::debug!("toggle_freecam(): disabling freecam");
            FREECAM_DEBUG_FLAG_HOOK.disable().unwrap();
        }
    }
}

// Cursed but whatever
unsafe fn set_freecam_enable(enabled: u8) -> bool {
    let a = *(0x144dbed30 as *const usize);
    log::trace!("set_freecam_enable(): a = {:x?}", a);
    if a as usize == 0x0 {
        return false;
    }

    let b = *((a + 0x20) as *const usize);
    log::trace!("set_freecam_enable(): b = {:x?}", b);
    if b as usize == 0x0 {
        return false;
    }

    let c = (b + 0x134) as *mut u8;
    log::trace!("set_freecam_enable(): c = {:x?}", c);
    if c as usize == 0x0 {
        return false;
    }

    *c = enabled as u8;
    true
}

unsafe fn toggle_timecontrol() {
    if !FLIPPER_UPDATE_HOOK.is_enabled() {
        log::debug!("toggle_timecontrol(): enabling time control");
        FLIPPER_UPDATE_HOOK.enable().unwrap();
    } else {
        log::debug!("toggle_timecontrol(): disabling time control");
        FLIPPER_UPDATE_HOOK.disable().unwrap();
    }
}

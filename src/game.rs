pub const POINTER_FLIPPER_UPDATE: usize     = 0x141759750;
pub const POINTER_SCALEFORM_UPDATE_B: usize = 0x1415f1c50;
pub const POINTER_FREECAM_DEBUG_FLAG: usize = 0x145df6d00;
pub const POINTER_SOME_RANDOM_BASE: usize   = 0x144e147e8;

pub const POINTER_CAMERA_SPEED_HOOK_1: usize = 0x1416f6f00;
pub const POINTER_CAMERA_SPEED_HOOK_2: usize = 0x1416f6f10;
pub const POINTER_CAMERA_SPEED_HOOK_3: usize = 0x1416f6f20;
pub const POINTER_CAMERA_SPEED_HOOK_4: usize = 0x1416f6f50;
pub const POINTER_CAMERA_SPEED_HOOK_5: usize = 0x1416f6f60;

pub static mut GLOBAL_TIME_MULTIPLIER: f32 = 1.0;
pub static mut CAMERA_SPEED_MULTIPLIER: f32 = 1.0;

#[repr(C)]
pub struct CSFlipperImp {
    pub unk: [u8; 0x268],
    pub delta_time: f32,
}

pub const POINTER_FLIPPER_UPDATE: usize     = 0x141723df0;
pub const POINTER_SCALEFORM_UPDATE_B: usize = 0x1415c1080;
pub const POINTER_FREECAM_DEBUG_FLAG: usize = 0x142d1e004;
pub const SOLO_PARAM_REPOSITORY: usize      = 0x144dd2670;

pub const POINTER_CAMERA_SPEED_HOOK_1: usize = 0x1416c1640;
pub const POINTER_CAMERA_SPEED_HOOK_2: usize = 0x1416c1650;
pub const POINTER_CAMERA_SPEED_HOOK_3: usize = 0x1416c1660;
pub const POINTER_CAMERA_SPEED_HOOK_4: usize = 0x1416c1690;
pub const POINTER_CAMERA_SPEED_HOOK_5: usize = 0x1416c16a0;

pub static mut GLOBAL_TIME_MULTIPLIER: f32 = 1.0;
pub static mut CAMERA_SPEED_MULTIPLIER: f32 = 1.0;

#[repr(C)]
pub struct CSFlipperImp {
    pub unk: [u8; 0x268],
    pub delta_time: f32,
}

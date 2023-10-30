pub const POINTER_FLIPPER_UPDATE: usize = 0x141723df0;
pub const POINTER_SCALEFORM_UPDATE_B: usize = 0x1415c1080;
pub const POINTER_FREECAM_DEBUG_FLAG: usize = 0x142d1e004;

pub static mut GLOBAL_TIME_MULTIPLIER: f32 = 1.0;

#[repr(C)]
pub struct CSFlipperImp {
    pub unk: [u8; 0x268],
    pub delta_time: f32,
}

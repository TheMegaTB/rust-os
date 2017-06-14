use super::color;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: color::ColorCode,
}

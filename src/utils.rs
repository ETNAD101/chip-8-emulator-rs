pub const X_PIXELS: u32 = 64;
pub const Y_PIXELS: u32 = 32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OFstate {
    ON,
    OFF,
}

pub enum TextType {
    Function,
}
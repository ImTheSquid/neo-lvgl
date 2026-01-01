//! Color types and utilities

/// LVGL color value
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Color(neo_lvgl_sys::lv_color_t);

impl Color {
    /// Create a color from RGB values
    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        unsafe { Self(neo_lvgl_sys::lv_color_make(r, g, b)) }
    }

    /// Create a color from a hex value (0xRRGGBB)
    #[inline]
    pub fn hex(hex: u32) -> Self {
        unsafe { Self(neo_lvgl_sys::lv_color_hex(hex)) }
    }

    /// Create a color from a 3-digit hex value (0xRGB)
    #[inline]
    pub fn hex3(hex: u32) -> Self {
        unsafe { Self(neo_lvgl_sys::lv_color_hex3(hex)) }
    }

    /// Black color
    pub const fn black() -> Self {
        Self(neo_lvgl_sys::lv_color_t { blue: 0, green: 0, red: 0 })
    }

    /// White color
    pub const fn white() -> Self {
        Self(neo_lvgl_sys::lv_color_t { blue: 255, green: 255, red: 255 })
    }

    /// Get the raw LVGL color value
    #[inline]
    pub(crate) fn raw(self) -> neo_lvgl_sys::lv_color_t {
        self.0
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

/// Opacity value (0 = transparent, 255 = opaque)
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Opacity(u8);

impl Opacity {
    /// Fully transparent
    pub const TRANSPARENT: Self = Self(0);
    /// 10% opacity
    pub const OPA_10: Self = Self(25);
    /// 20% opacity
    pub const OPA_20: Self = Self(51);
    /// 30% opacity
    pub const OPA_30: Self = Self(76);
    /// 40% opacity
    pub const OPA_40: Self = Self(102);
    /// 50% opacity
    pub const OPA_50: Self = Self(127);
    /// 60% opacity
    pub const OPA_60: Self = Self(153);
    /// 70% opacity
    pub const OPA_70: Self = Self(178);
    /// 80% opacity
    pub const OPA_80: Self = Self(204);
    /// 90% opacity
    pub const OPA_90: Self = Self(229);
    /// Fully opaque
    pub const OPAQUE: Self = Self(255);
    /// Alias for OPAQUE
    pub const COVER: Self = Self::OPAQUE;

    /// Create from a raw value (0-255)
    #[inline]
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    /// Get the raw value
    #[inline]
    pub(crate) fn raw(self) -> u8 {
        self.0
    }
}

impl Default for Opacity {
    fn default() -> Self {
        Self::OPAQUE
    }
}

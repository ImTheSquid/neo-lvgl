//! Font system for LVGL
//!
//! This module provides access to LVGL's font system, including:
//!
//! - `Font` - Reference to a built-in font
//! - `TtfFont` - Runtime-loaded TTF/OTF font (requires `ttf` feature)
//!
//! # Built-in Fonts
//!
//! LVGL includes Montserrat fonts at various sizes. The availability depends
//! on your `lv_conf.h` configuration.
//!
//! ```ignore
//! use lvgl::font::Font;
//!
//! let font = Font::montserrat_14();
//! style.set_text_font(&font);
//! ```
//!
//! # TTF Fonts (requires ttf feature)
//!
//! ```ignore
//! use lvgl::font::TtfFont;
//!
//! let font = TtfFont::from_file(c"/fonts/roboto.ttf", 24)?;
//! style.set_text_font(font.as_font());
//! ```

#[cfg(feature = "ttf")]
use core::ffi::CStr;

/// Reference to an LVGL font
///
/// This is a non-owning reference to a font. Built-in fonts are static
/// and don't need to be freed.
#[derive(Clone, Copy)]
pub struct Font {
    raw: *const neo_lvgl_sys::lv_font_t,
}

impl Font {
    /// Create from raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be valid for the lifetime of the Font.
    pub const unsafe fn from_raw(raw: *const neo_lvgl_sys::lv_font_t) -> Self {
        Self { raw }
    }

    /// Get the raw font pointer
    pub const fn raw(&self) -> *const neo_lvgl_sys::lv_font_t {
        self.raw
    }

    /// Get the line height in pixels
    pub fn line_height(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_font_get_line_height(self.raw) }
    }

    /// Get the default font (as configured in lv_conf.h)
    pub fn default() -> Self {
        unsafe { Self::from_raw(neo_lvgl_sys::lv_font_get_default()) }
    }
}

/// Error type for font operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontError {
    /// Failed to load the font file
    LoadFailed,
    /// Invalid font data
    InvalidData,
    /// Feature not enabled in lv_conf.h
    NotSupported,
}

/// TTF font loaded at runtime
///
/// Requires `LV_USE_TINY_TTF` to be enabled in `lv_conf.h`.
#[cfg(feature = "ttf")]
pub struct TtfFont {
    raw: *mut neo_lvgl_sys::lv_font_t,
}

#[cfg(feature = "ttf")]
impl TtfFont {
    /// Create a font from a TTF file path
    ///
    /// Requires `LV_TINY_TTF_FILE_SUPPORT` to be enabled.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TTF/OTF file
    /// * `size` - Font size in pixels
    pub fn from_file(path: &CStr, size: i32) -> Result<Self, FontError> {
        let ptr = unsafe {
            neo_lvgl_sys::lv_tiny_ttf_create_file(path.as_ptr(), size)
        };
        if ptr.is_null() {
            Err(FontError::LoadFailed)
        } else {
            Ok(Self { raw: ptr })
        }
    }

    /// Create a font from TTF data in memory
    ///
    /// # Arguments
    ///
    /// * `data` - TTF/OTF file data (must remain valid for font lifetime)
    /// * `size` - Font size in pixels
    pub fn from_data(data: &'static [u8], size: i32) -> Result<Self, FontError> {
        let ptr = unsafe {
            neo_lvgl_sys::lv_tiny_ttf_create_data(
                data.as_ptr() as *const _,
                data.len(),
                size,
            )
        };
        if ptr.is_null() {
            Err(FontError::LoadFailed)
        } else {
            Ok(Self { raw: ptr })
        }
    }

    /// Get as a Font reference for use with widgets
    pub fn as_font(&self) -> Font {
        unsafe { Font::from_raw(self.raw) }
    }

    /// Set the font size
    ///
    /// This allows reusing the same font at different sizes.
    pub fn set_size(&mut self, size: i32) {
        unsafe {
            neo_lvgl_sys::lv_tiny_ttf_set_size(self.raw, size);
        }
    }

    /// Get the line height
    pub fn line_height(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_font_get_line_height(self.raw) }
    }

    /// Get the raw font pointer
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_font_t {
        self.raw
    }
}

#[cfg(feature = "ttf")]
impl Drop for TtfFont {
    fn drop(&mut self) {
        unsafe {
            neo_lvgl_sys::lv_tiny_ttf_destroy(self.raw);
        }
    }
}

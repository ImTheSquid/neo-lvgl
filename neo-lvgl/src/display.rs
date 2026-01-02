//! Display management

use core::ptr::NonNull;

/// LVGL display
pub struct Display {
    raw: NonNull<neo_lvgl_sys::lv_display_t>,
}

impl Display {
    /// Create a new display with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - Horizontal resolution in pixels
    /// * `height` - Vertical resolution in pixels
    pub fn new(width: i32, height: i32) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_display_create(width, height);
            NonNull::new(ptr).map(|raw| Self { raw })
        }
    }

    /// Get the raw pointer to the display
    #[inline]
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_display_t {
        self.raw.as_ptr()
    }

    /// Set this display as the default
    pub fn set_default(&self) {
        unsafe {
            neo_lvgl_sys::lv_display_set_default(self.raw.as_ptr());
        }
    }

    /// Get the currently active screen for this display
    pub fn active_screen(&self) -> crate::widgets::Screen<'_> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_display_get_screen_active(self.raw.as_ptr());
            crate::widgets::Screen::from_raw(ptr)
        }
    }

    /// Set the display buffers for rendering.
    ///
    /// # Safety
    ///
    /// The buffers must remain valid for the lifetime of the display.
    pub unsafe fn set_buffers(
        &self,
        buf1: &'static mut [u8],
        buf2: Option<&'static mut [u8]>,
        render_mode: RenderMode,
    ) {
        let buf2_ptr = match &buf2 {
            Some(b) => b.as_ptr() as *mut _,
            None => core::ptr::null_mut(),
        };

        neo_lvgl_sys::lv_display_set_buffers(
            self.raw.as_ptr(),
            buf1.as_mut_ptr() as *mut _,
            buf2_ptr,
            buf1.len() as u32,
            render_mode.to_raw(),
        );
    }

    /// Set the flush callback for the display.
    ///
    /// This callback is invoked when LVGL needs to send pixels to the display hardware.
    pub fn set_flush_cb(&self, cb: FlushCb) {
        unsafe {
            neo_lvgl_sys::lv_display_set_flush_cb(self.raw.as_ptr(), Some(cb));
        }
    }

    /// Signal that the flush operation is complete.
    ///
    /// Call this from your flush callback when the transfer is done.
    pub fn flush_ready(&self) {
        unsafe {
            neo_lvgl_sys::lv_display_flush_ready(self.raw.as_ptr());
        }
    }

    /// Get display width
    pub fn width(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_display_get_horizontal_resolution(self.raw.as_ptr()) }
    }

    /// Get display height
    pub fn height(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_display_get_vertical_resolution(self.raw.as_ptr()) }
    }

    /// Set the color format for this display.
    ///
    /// This determines how pixels are encoded in the display buffers.
    /// The default is ARGB8888.
    pub fn set_color_format(&self, format: ColorFormat) {
        unsafe {
            neo_lvgl_sys::lv_display_set_color_format(self.raw.as_ptr(), format.to_raw());
        }
    }

    /// Get the current color format for this display.
    pub fn color_format(&self) -> ColorFormat {
        let raw = unsafe { neo_lvgl_sys::lv_display_get_color_format(self.raw.as_ptr()) };
        ColorFormat::from_raw(raw)
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            neo_lvgl_sys::lv_display_delete(self.raw.as_ptr());
        }
    }
}

/// Display render mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMode {
    /// Partial rendering - only changed areas are redrawn
    Partial,
    /// Direct rendering - buffer represents the full screen
    Direct,
    /// Full refresh - entire screen is redrawn each frame
    Full,
}

impl RenderMode {
    fn to_raw(self) -> neo_lvgl_sys::lv_display_render_mode_t {
        match self {
            RenderMode::Partial => neo_lvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_PARTIAL,
            RenderMode::Direct => neo_lvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_DIRECT,
            RenderMode::Full => neo_lvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_FULL,
        }
    }
}

impl Default for RenderMode {
    fn default() -> Self {
        RenderMode::Partial
    }
}

/// Display color format
///
/// Determines how pixels are encoded in display buffers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorFormat {
    /// 8-bit grayscale
    L8,
    /// 1-bit alpha
    A1,
    /// 2-bit alpha
    A2,
    /// 4-bit alpha
    A4,
    /// 8-bit alpha
    A8,
    /// 1-bit indexed color (palette)
    I1,
    /// 2-bit indexed color (palette)
    I2,
    /// 4-bit indexed color (palette)
    I4,
    /// 8-bit indexed color (palette)
    I8,
    /// 16-bit RGB (5-6-5)
    Rgb565,
    /// 16-bit RGB (5-6-5) with byte-swapped pixels
    Rgb565Swapped,
    /// 24-bit RGB (8-8-8)
    Rgb888,
    /// 32-bit ARGB (8-8-8-8)
    Argb8888,
    /// 32-bit XRGB (8-8-8-8, alpha ignored)
    Xrgb8888,
}

impl ColorFormat {
    fn to_raw(self) -> neo_lvgl_sys::lv_color_format_t {
        match self {
            ColorFormat::L8 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_L8,
            ColorFormat::A1 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A1,
            ColorFormat::A2 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A2,
            ColorFormat::A4 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A4,
            ColorFormat::A8 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A8,
            ColorFormat::I1 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I1,
            ColorFormat::I2 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I2,
            ColorFormat::I4 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I4,
            ColorFormat::I8 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I8,
            ColorFormat::Rgb565 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565,
            ColorFormat::Rgb565Swapped => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565_SWAPPED,
            ColorFormat::Rgb888 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB888,
            ColorFormat::Argb8888 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_ARGB8888,
            ColorFormat::Xrgb8888 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_XRGB8888,
        }
    }

    fn from_raw(raw: neo_lvgl_sys::lv_color_format_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_L8 => ColorFormat::L8,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A1 => ColorFormat::A1,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A2 => ColorFormat::A2,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A4 => ColorFormat::A4,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A8 => ColorFormat::A8,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I1 => ColorFormat::I1,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I2 => ColorFormat::I2,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I4 => ColorFormat::I4,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I8 => ColorFormat::I8,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565 => ColorFormat::Rgb565,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565_SWAPPED => ColorFormat::Rgb565Swapped,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB888 => ColorFormat::Rgb888,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_XRGB8888 => ColorFormat::Xrgb8888,
            _ => ColorFormat::Argb8888, // Default fallback
        }
    }

    /// Get the number of bytes per pixel for this format.
    ///
    /// Note: For sub-byte formats (A1, A2, A4, I1, I2, I4), this returns 1
    /// as the minimum addressable unit, though actual bits per pixel is less.
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            ColorFormat::A1 | ColorFormat::I1 => 1,
            ColorFormat::A2 | ColorFormat::I2 => 1,
            ColorFormat::A4 | ColorFormat::I4 => 1,
            ColorFormat::L8 | ColorFormat::A8 | ColorFormat::I8 => 1,
            ColorFormat::Rgb565 | ColorFormat::Rgb565Swapped => 2,
            ColorFormat::Rgb888 => 3,
            ColorFormat::Argb8888 | ColorFormat::Xrgb8888 => 4,
        }
    }
}

impl Default for ColorFormat {
    fn default() -> Self {
        ColorFormat::Argb8888
    }
}

/// Flush callback type
pub type FlushCb = unsafe extern "C" fn(
    disp: *mut neo_lvgl_sys::lv_display_t,
    area: *const neo_lvgl_sys::lv_area_t,
    px_map: *mut u8,
);

/// Display driver trait for custom display implementations
pub trait DisplayDriver {
    /// Get display dimensions (width, height)
    fn size(&self) -> (i32, i32);

    /// Flush pixels to the display
    ///
    /// # Arguments
    ///
    /// * `area` - The rectangular area to update
    /// * `pixels` - The pixel data in the display's color format
    fn flush(&mut self, area: &Area, pixels: &[u8]);

    /// Called when flush is complete (optional)
    fn flush_ready(&mut self) {}
}

/// Rectangular area
#[derive(Clone, Copy, Debug)]
pub struct Area {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}

impl Area {
    /// Create from raw LVGL area
    pub fn from_raw(raw: &neo_lvgl_sys::lv_area_t) -> Self {
        Self {
            x1: raw.x1 as i16,
            y1: raw.y1 as i16,
            x2: raw.x2 as i16,
            y2: raw.y2 as i16,
        }
    }

    /// Get area width
    #[inline]
    pub fn width(&self) -> i16 {
        self.x2 - self.x1 + 1
    }

    /// Get area height
    #[inline]
    pub fn height(&self) -> i16 {
        self.y2 - self.y1 + 1
    }

    /// Get total pixel count
    #[inline]
    pub fn pixel_count(&self) -> usize {
        (self.width() as usize) * (self.height() as usize)
    }
}

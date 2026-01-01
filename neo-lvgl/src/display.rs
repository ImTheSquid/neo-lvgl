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

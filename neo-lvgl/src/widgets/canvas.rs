//! Canvas widget

use super::{Obj, Widget};
use crate::color::{Color, Opacity};
use crate::event::EventHandler;

/// Color format for canvas buffer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorFormat {
    /// RGB565 (16-bit)
    RGB565,
    /// RGB888 (24-bit)
    RGB888,
    /// ARGB8888 (32-bit with alpha)
    ARGB8888,
    /// XRGB8888 (32-bit without alpha)
    XRGB8888,
    /// 8-bit luminance
    L8,
    /// 8-bit alpha
    A8,
    /// 1-bit indexed
    I1,
}

impl ColorFormat {
    fn to_raw(self) -> neo_lvgl_sys::lv_color_format_t {
        match self {
            ColorFormat::RGB565 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565,
            ColorFormat::RGB888 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB888,
            ColorFormat::ARGB8888 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_ARGB8888,
            ColorFormat::XRGB8888 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_XRGB8888,
            ColorFormat::L8 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_L8,
            ColorFormat::A8 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A8,
            ColorFormat::I1 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I1,
        }
    }

    /// Get bits per pixel for this format
    pub fn bpp(self) -> u8 {
        match self {
            ColorFormat::RGB565 => 16,
            ColorFormat::RGB888 => 24,
            ColorFormat::ARGB8888 | ColorFormat::XRGB8888 => 32,
            ColorFormat::L8 | ColorFormat::A8 => 8,
            ColorFormat::I1 => 1,
        }
    }
}

/// Canvas widget
///
/// A drawing surface that can be used for custom graphics.
///
/// # Example
///
/// ```ignore
/// // Calculate buffer size
/// let buf_size = Canvas::buffer_size(100, 100, ColorFormat::RGB565);
/// let mut buf = vec![0u8; buf_size as usize];
///
/// let canvas = Canvas::new(&screen).unwrap();
/// canvas.set_buffer(&mut buf, 100, 100, ColorFormat::RGB565);
/// canvas.fill_bg(Color::white(), Opacity::Cover);
/// canvas.set_px(50, 50, Color::red(), Opacity::Cover);
/// ```
#[derive(Clone, Copy)]
pub struct Canvas<'a> {
    obj: Obj<'a>,
}

impl<'a> Canvas<'a> {
    /// Create a new canvas as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_canvas_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Calculate the required buffer size for a canvas
    ///
    /// # Arguments
    ///
    /// * `width` - Canvas width in pixels
    /// * `height` - Canvas height in pixels
    /// * `cf` - Color format
    pub fn buffer_size(width: i32, height: i32, cf: ColorFormat) -> u32 {
        let stride = (width as u32 * cf.bpp() as u32 + 7) / 8;
        unsafe { neo_lvgl_sys::lv_canvas_buf_size(width, height, cf.bpp(), stride as u8) }
    }

    /// Set the canvas buffer
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer to use (must remain valid)
    /// * `width` - Canvas width
    /// * `height` - Canvas height
    /// * `cf` - Color format
    ///
    /// # Safety
    ///
    /// The buffer must remain valid and must be large enough for the specified dimensions.
    pub unsafe fn set_buffer(&self, buf: &mut [u8], width: i32, height: i32, cf: ColorFormat) {
        neo_lvgl_sys::lv_canvas_set_buffer(
            self.obj.raw(),
            buf.as_mut_ptr() as *mut _,
            width,
            height,
            cf.to_raw(),
        );
    }

    /// Set a pixel color
    pub fn set_px(&self, x: i32, y: i32, color: Color, opa: Opacity) {
        unsafe {
            neo_lvgl_sys::lv_canvas_set_px(self.obj.raw(), x, y, color.to_raw(), opa.to_raw());
        }
    }

    /// Fill the background with a color
    pub fn fill_bg(&self, color: Color, opa: Opacity) {
        unsafe {
            neo_lvgl_sys::lv_canvas_fill_bg(self.obj.raw(), color.to_raw(), opa.to_raw());
        }
    }

    /// Set a palette color (for indexed color formats)
    pub fn set_palette(&self, index: u8, color: Color) {
        unsafe {
            // Convert Color to lv_color32_t format using make function
            let raw = color.to_raw();
            let color32 = neo_lvgl_sys::lv_color32_make(raw.red, raw.green, raw.blue, 255);
            neo_lvgl_sys::lv_canvas_set_palette(self.obj.raw(), index, color32);
        }
    }
}

impl<'a> Widget<'a> for Canvas<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Canvas<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



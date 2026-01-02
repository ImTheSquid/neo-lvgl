//! Image widget

use super::{Obj, Widget};
use crate::event::EventHandler;

/// Image alignment within the widget
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageAlign {
    Default,
    TopLeft,
    TopMid,
    TopRight,
    BottomLeft,
    BottomMid,
    BottomRight,
    LeftMid,
    RightMid,
    Center,
    Stretch,
    Tile,
    Contain,
    Cover,
}

impl ImageAlign {
    fn to_raw(self) -> neo_lvgl_sys::lv_image_align_t {
        match self {
            ImageAlign::Default => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_DEFAULT,
            ImageAlign::TopLeft => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_TOP_LEFT,
            ImageAlign::TopMid => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_TOP_MID,
            ImageAlign::TopRight => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_TOP_RIGHT,
            ImageAlign::BottomLeft => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_BOTTOM_LEFT,
            ImageAlign::BottomMid => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_BOTTOM_MID,
            ImageAlign::BottomRight => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_BOTTOM_RIGHT,
            ImageAlign::LeftMid => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_LEFT_MID,
            ImageAlign::RightMid => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_RIGHT_MID,
            ImageAlign::Center => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_CENTER,
            ImageAlign::Stretch => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_STRETCH,
            ImageAlign::Tile => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_TILE,
            ImageAlign::Contain => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_CONTAIN,
            ImageAlign::Cover => neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_COVER,
        }
    }

    fn from_raw(raw: neo_lvgl_sys::lv_image_align_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_TOP_LEFT => ImageAlign::TopLeft,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_TOP_MID => ImageAlign::TopMid,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_TOP_RIGHT => ImageAlign::TopRight,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_BOTTOM_LEFT => ImageAlign::BottomLeft,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_BOTTOM_MID => ImageAlign::BottomMid,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_BOTTOM_RIGHT => ImageAlign::BottomRight,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_LEFT_MID => ImageAlign::LeftMid,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_RIGHT_MID => ImageAlign::RightMid,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_CENTER => ImageAlign::Center,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_STRETCH => ImageAlign::Stretch,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_TILE => ImageAlign::Tile,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_CONTAIN => ImageAlign::Contain,
            neo_lvgl_sys::lv_image_align_t_LV_IMAGE_ALIGN_COVER => ImageAlign::Cover,
            _ => ImageAlign::Default,
        }
    }
}

/// Scale factor as percentage
///
/// 100 = normal size, 200 = 2x, 50 = half size
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scale(u16);

impl Scale {
    /// Create scale from percentage (100 = normal size)
    pub fn percent(pct: u16) -> Self {
        Self(pct)
    }

    /// Create scale from a float factor (1.0 = normal size)
    pub fn from_f32(factor: f32) -> Self {
        Self((factor * 100.0) as u16)
    }

    /// Get as percentage
    pub fn as_percent(self) -> u16 {
        self.0
    }

    /// Get as float factor
    pub fn as_f32(self) -> f32 {
        self.0 as f32 / 100.0
    }

    /// Convert to LVGL's internal scale (256 = 100%)
    fn to_raw(self) -> u32 {
        (self.0 as u32 * 256) / 100
    }

    /// Create from LVGL's internal scale (256 = 100%)
    fn from_raw(raw: i32) -> Self {
        Self(((raw as u32 * 100) / 256) as u16)
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self(100)
    }
}

/// Rotation in degrees
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rotation(i16);

impl Rotation {
    /// Create rotation from degrees
    pub fn degrees(deg: i16) -> Self {
        Self(deg)
    }

    /// Get as degrees
    pub fn as_degrees(self) -> i16 {
        self.0
    }

    /// Convert to LVGL's internal format (0.1 degree units)
    fn to_raw(self) -> i32 {
        self.0 as i32 * 10
    }

    /// Create from LVGL's internal format (0.1 degree units)
    fn from_raw(raw: i32) -> Self {
        Self((raw / 10) as i16)
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Self(0)
    }
}

/// Image widget
///
/// Displays an image from various sources.
///
/// # Example
///
/// ```ignore
/// let img = Image::new(&screen).unwrap();
/// img.set_src_symbol(lvgl::SYMBOL_OK);
/// img.set_rotation(Rotation::degrees(45));
/// img.set_scale(Scale::percent(150));
/// ```
#[derive(Clone, Copy)]
pub struct Image<'a> {
    obj: Obj<'a>,
}

impl<'a> Image<'a> {
    /// Create a new image as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_image_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the image source (raw pointer to image descriptor or symbol)
    ///
    /// # Safety
    ///
    /// The source must remain valid for the lifetime of the image.
    pub unsafe fn set_src(&self, src: *const core::ffi::c_void) {
        neo_lvgl_sys::lv_image_set_src(self.obj.raw(), src);
    }

    /// Set the image offset
    pub fn set_offset(&self, x: i32, y: i32) {
        unsafe {
            neo_lvgl_sys::lv_image_set_offset_x(self.obj.raw(), x);
            neo_lvgl_sys::lv_image_set_offset_y(self.obj.raw(), y);
        }
    }

    /// Set the image rotation
    pub fn set_rotation(&self, rotation: Rotation) {
        unsafe {
            neo_lvgl_sys::lv_image_set_rotation(self.obj.raw(), rotation.to_raw());
        }
    }

    /// Get the image rotation
    pub fn rotation(&self) -> Rotation {
        let raw = unsafe { neo_lvgl_sys::lv_image_get_rotation(self.obj.raw()) };
        Rotation::from_raw(raw)
    }

    /// Set the rotation pivot point
    pub fn set_pivot(&self, x: i32, y: i32) {
        unsafe {
            neo_lvgl_sys::lv_image_set_pivot(self.obj.raw(), x, y);
        }
    }

    /// Set uniform scale
    pub fn set_scale(&self, scale: Scale) {
        unsafe {
            neo_lvgl_sys::lv_image_set_scale(self.obj.raw(), scale.to_raw());
        }
    }

    /// Set horizontal scale
    pub fn set_scale_x(&self, scale: Scale) {
        unsafe {
            neo_lvgl_sys::lv_image_set_scale_x(self.obj.raw(), scale.to_raw());
        }
    }

    /// Set vertical scale
    pub fn set_scale_y(&self, scale: Scale) {
        unsafe {
            neo_lvgl_sys::lv_image_set_scale_y(self.obj.raw(), scale.to_raw());
        }
    }

    /// Get the scale
    pub fn scale(&self) -> Scale {
        let raw = unsafe { neo_lvgl_sys::lv_image_get_scale(self.obj.raw()) };
        Scale::from_raw(raw)
    }

    /// Get the horizontal scale
    pub fn scale_x(&self) -> Scale {
        let raw = unsafe { neo_lvgl_sys::lv_image_get_scale_x(self.obj.raw()) };
        Scale::from_raw(raw)
    }

    /// Get the vertical scale
    pub fn scale_y(&self) -> Scale {
        let raw = unsafe { neo_lvgl_sys::lv_image_get_scale_y(self.obj.raw()) };
        Scale::from_raw(raw)
    }

    /// Enable/disable antialiasing for transformations
    pub fn set_antialias(&self, en: bool) {
        unsafe {
            neo_lvgl_sys::lv_image_set_antialias(self.obj.raw(), en);
        }
    }

    /// Check if antialiasing is enabled
    pub fn antialias(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_image_get_antialias(self.obj.raw()) }
    }

    /// Set the inner alignment
    pub fn set_inner_align(&self, align: ImageAlign) {
        unsafe {
            neo_lvgl_sys::lv_image_set_inner_align(self.obj.raw(), align.to_raw());
        }
    }

    /// Get the inner alignment
    pub fn inner_align(&self) -> ImageAlign {
        let raw = unsafe { neo_lvgl_sys::lv_image_get_inner_align(self.obj.raw()) };
        ImageAlign::from_raw(raw)
    }

    /// Get the source image width
    pub fn src_width(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_image_get_src_width(self.obj.raw()) }
    }

    /// Get the source image height
    pub fn src_height(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_image_get_src_height(self.obj.raw()) }
    }

    /// Get the transformed image width
    pub fn transformed_width(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_image_get_transformed_width(self.obj.raw()) }
    }

    /// Get the transformed image height
    pub fn transformed_height(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_image_get_transformed_height(self.obj.raw()) }
    }
}

impl<'a> Widget<'a> for Image<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Image<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



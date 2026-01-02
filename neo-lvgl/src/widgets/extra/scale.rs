//! Scale widget

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};
use core::ffi::c_char;
use core::ptr::NonNull;

/// Scale mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleMode {
    /// Horizontal scale
    Horizontal,
    /// Vertical scale
    Vertical,
    /// Round/circular scale
    Round,
}

impl ScaleMode {
    fn to_raw(self) -> neo_lvgl_sys::lv_scale_mode_t {
        match self {
            ScaleMode::Horizontal => neo_lvgl_sys::lv_scale_mode_t_LV_SCALE_MODE_HORIZONTAL_BOTTOM,
            ScaleMode::Vertical => neo_lvgl_sys::lv_scale_mode_t_LV_SCALE_MODE_VERTICAL_LEFT,
            ScaleMode::Round => neo_lvgl_sys::lv_scale_mode_t_LV_SCALE_MODE_ROUND_INNER,
        }
    }
}

/// Scale section descriptor
pub struct ScaleSectionDescr {
    raw: NonNull<neo_lvgl_sys::lv_scale_section_t>,
}

impl ScaleSectionDescr {
    /// Create from raw pointer
    unsafe fn from_raw(ptr: *mut neo_lvgl_sys::lv_scale_section_t) -> Option<Self> {
        NonNull::new(ptr).map(|raw| Self { raw })
    }

    /// Get the raw pointer
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_scale_section_t {
        self.raw.as_ptr()
    }
}

/// Scale widget
///
/// A gauge/scale with tick marks and labels.
///
/// # Example
///
/// ```ignore
/// let scale = Scale::new(&screen).unwrap();
/// scale.set_mode(ScaleMode::Round);
/// scale.set_range(0, 100);
/// scale.set_total_tick_count(11);
/// scale.set_major_tick_every(2);
/// scale.set_label_show(true);
/// ```
#[derive(Clone, Copy)]
pub struct Scale<'a> {
    obj: Obj<'a>,
}

impl<'a> Scale<'a> {
    /// Create a new scale as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_scale_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the scale mode
    pub fn set_mode(&self, mode: ScaleMode) {
        unsafe {
            neo_lvgl_sys::lv_scale_set_mode(self.obj.raw(), mode.to_raw());
        }
    }

    /// Set the value range
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe {
            neo_lvgl_sys::lv_scale_set_range(self.obj.raw(), min, max);
        }
    }

    /// Set the total number of tick marks
    pub fn set_total_tick_count(&self, count: u32) {
        unsafe {
            neo_lvgl_sys::lv_scale_set_total_tick_count(self.obj.raw(), count);
        }
    }

    /// Set how often major ticks appear
    pub fn set_major_tick_every(&self, every: u32) {
        unsafe {
            neo_lvgl_sys::lv_scale_set_major_tick_every(self.obj.raw(), every);
        }
    }

    /// Set whether to show labels on major ticks
    pub fn set_label_show(&self, show: bool) {
        unsafe {
            neo_lvgl_sys::lv_scale_set_label_show(self.obj.raw(), show);
        }
    }

    /// Add a colored section to the scale
    pub fn add_section(&self) -> Option<ScaleSectionDescr> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_scale_add_section(self.obj.raw());
            ScaleSectionDescr::from_raw(ptr)
        }
    }

    /// Set the range of a section
    pub fn set_section_range(&self, section: &ScaleSectionDescr, min: i32, max: i32) {
        unsafe {
            neo_lvgl_sys::lv_scale_section_set_range(section.raw(), min, max);
        }
    }

    /// Set the angle range for round scales
    pub fn set_angle_range(&self, angle: u32) {
        unsafe {
            neo_lvgl_sys::lv_scale_set_angle_range(self.obj.raw(), angle);
        }
    }

    /// Set the rotation offset for round scales
    pub fn set_rotation(&self, rotation: i32) {
        unsafe {
            neo_lvgl_sys::lv_scale_set_rotation(self.obj.raw(), rotation);
        }
    }

    /// Set the line needle value (for round scales with a needle indicator)
    pub fn set_line_needle_value(&self, needle: &Obj, needle_length: i32, value: i32) {
        unsafe {
            neo_lvgl_sys::lv_scale_set_line_needle_value(
                self.obj.raw(),
                needle.raw(),
                needle_length,
                value,
            );
        }
    }

    /// Set the image needle value (for round scales with an image needle)
    pub fn set_image_needle_value(&self, needle: &Obj, value: i32) {
        unsafe {
            neo_lvgl_sys::lv_scale_set_image_needle_value(self.obj.raw(), needle.raw(), value);
        }
    }

    /// Set custom text for a specific tick
    ///
    /// # Safety
    ///
    /// The text array must remain valid.
    pub unsafe fn set_text_src(&self, texts: *mut *const c_char) {
        neo_lvgl_sys::lv_scale_set_text_src(self.obj.raw(), texts);
    }

    /// Set post-draw callback for custom drawing
    pub fn set_post_draw(&self, enable: bool) {
        unsafe {
            neo_lvgl_sys::lv_scale_set_post_draw(self.obj.raw(), enable);
        }
    }
}

impl<'a> Widget<'a> for Scale<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Scale<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



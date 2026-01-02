//! Slider widget

use super::{Obj, Widget};
use super::bar::{BarMode, BarOrientation};
use crate::event::EventHandler;
use crate::observer::Subject;

/// Slider mode (same as Bar mode)
pub type SliderMode = BarMode;

/// Slider orientation (same as Bar orientation)
pub type SliderOrientation = BarOrientation;

/// Slider widget
///
/// An interactive slider for selecting a value within a range.
///
/// # Example
///
/// ```ignore
/// let slider = Slider::new(&screen).unwrap();
/// slider.set_range(0, 100);
/// slider.set_value(50, false);
///
/// slider.on_value_changed(|_| {
///     println!("Slider value changed!");
/// });
/// ```
#[derive(Clone, Copy)]
pub struct Slider<'a> {
    obj: Obj<'a>,
}

impl<'a> Slider<'a> {
    /// Create a new slider as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_slider_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the slider value
    ///
    /// # Arguments
    ///
    /// * `value` - The new value
    /// * `anim` - Whether to animate the change
    pub fn set_value(&self, value: i32, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_slider_set_value(self.obj.raw(), value, anim);
        }
    }

    /// Set the value range
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe {
            neo_lvgl_sys::lv_slider_set_range(self.obj.raw(), min, max);
        }
    }

    /// Set minimum value
    pub fn set_min_value(&self, min: i32) {
        unsafe {
            neo_lvgl_sys::lv_slider_set_min_value(self.obj.raw(), min);
        }
    }

    /// Set maximum value
    pub fn set_max_value(&self, max: i32) {
        unsafe {
            neo_lvgl_sys::lv_slider_set_max_value(self.obj.raw(), max);
        }
    }

    /// Set the slider mode
    pub fn set_mode(&self, mode: SliderMode) {
        unsafe {
            neo_lvgl_sys::lv_slider_set_mode(self.obj.raw(), mode.to_raw());
        }
    }

    /// Set the slider orientation
    pub fn set_orientation(&self, orientation: SliderOrientation) {
        unsafe {
            neo_lvgl_sys::lv_slider_set_orientation(self.obj.raw(), orientation.to_raw());
        }
    }

    /// Get the current value
    pub fn value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_slider_get_value(self.obj.raw()) }
    }

    /// Get the left (start) value for range mode
    pub fn left_value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_slider_get_left_value(self.obj.raw()) }
    }

    /// Get the minimum value
    pub fn min_value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_slider_get_min_value(self.obj.raw()) }
    }

    /// Get the maximum value
    pub fn max_value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_slider_get_max_value(self.obj.raw()) }
    }

    /// Check if the slider is being dragged
    pub fn is_dragged(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_slider_is_dragged(self.obj.raw()) }
    }

    /// Get the slider mode
    pub fn mode(&self) -> SliderMode {
        let raw = unsafe { neo_lvgl_sys::lv_slider_get_mode(self.obj.raw()) };
        SliderMode::from_raw(raw)
    }

    /// Get the slider orientation
    pub fn orientation(&self) -> SliderOrientation {
        let raw = unsafe { neo_lvgl_sys::lv_slider_get_orientation(self.obj.raw()) };
        SliderOrientation::from_raw(raw)
    }

    /// Check if the slider is symmetrical
    pub fn is_symmetrical(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_slider_is_symmetrical(self.obj.raw()) }
    }
}

impl<'a> Widget<'a> for Slider<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Slider<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}


impl<'a> Slider<'a> {
    /// Bind this slider's value to an integer subject
    ///
    /// When the subject changes, the slider updates automatically.
    /// When the slider is moved, the subject is updated.
    pub fn bind_value(
        &self,
        subject: &mut crate::observer::IntSubject,
    ) -> Option<crate::observer::Observer> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_slider_bind_value(self.obj.raw(), subject.raw());
            crate::observer::Observer::from_raw(ptr)
        }
    }
}


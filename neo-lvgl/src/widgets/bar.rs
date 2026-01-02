//! Bar widget

use super::{Obj, Widget};
use crate::event::EventHandler;
use crate::observer::Subject;

/// Bar mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarMode {
    /// Normal mode - indicator grows from minimum side
    Normal,
    /// Symmetrical mode - indicator grows from the center
    Symmetrical,
    /// Range mode - indicator has a start and end value
    Range,
}

impl BarMode {
    pub(crate) fn to_raw(self) -> neo_lvgl_sys::lv_bar_mode_t {
        match self {
            BarMode::Normal => neo_lvgl_sys::lv_bar_mode_t_LV_BAR_MODE_NORMAL,
            BarMode::Symmetrical => neo_lvgl_sys::lv_bar_mode_t_LV_BAR_MODE_SYMMETRICAL,
            BarMode::Range => neo_lvgl_sys::lv_bar_mode_t_LV_BAR_MODE_RANGE,
        }
    }

    pub(crate) fn from_raw(raw: neo_lvgl_sys::lv_bar_mode_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_bar_mode_t_LV_BAR_MODE_NORMAL => BarMode::Normal,
            neo_lvgl_sys::lv_bar_mode_t_LV_BAR_MODE_SYMMETRICAL => BarMode::Symmetrical,
            neo_lvgl_sys::lv_bar_mode_t_LV_BAR_MODE_RANGE => BarMode::Range,
            _ => BarMode::Normal,
        }
    }
}

/// Bar orientation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarOrientation {
    /// Automatically determine orientation based on size
    Auto,
    /// Horizontal bar
    Horizontal,
    /// Vertical bar
    Vertical,
}

impl BarOrientation {
    pub(crate) fn to_raw(self) -> neo_lvgl_sys::lv_bar_orientation_t {
        match self {
            BarOrientation::Auto => neo_lvgl_sys::lv_bar_orientation_t_LV_BAR_ORIENTATION_AUTO,
            BarOrientation::Horizontal => neo_lvgl_sys::lv_bar_orientation_t_LV_BAR_ORIENTATION_HORIZONTAL,
            BarOrientation::Vertical => neo_lvgl_sys::lv_bar_orientation_t_LV_BAR_ORIENTATION_VERTICAL,
        }
    }

    pub(crate) fn from_raw(raw: neo_lvgl_sys::lv_bar_orientation_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_bar_orientation_t_LV_BAR_ORIENTATION_AUTO => BarOrientation::Auto,
            neo_lvgl_sys::lv_bar_orientation_t_LV_BAR_ORIENTATION_HORIZONTAL => BarOrientation::Horizontal,
            neo_lvgl_sys::lv_bar_orientation_t_LV_BAR_ORIENTATION_VERTICAL => BarOrientation::Vertical,
            _ => BarOrientation::Auto,
        }
    }
}

/// Bar widget
///
/// A progress bar that shows a value within a range.
///
/// # Example
///
/// ```ignore
/// let bar = Bar::new(&screen).unwrap();
/// bar.set_range(0, 100);
/// bar.set_value(50, true); // 50% with animation
/// ```
#[derive(Clone, Copy)]
pub struct Bar<'a> {
    obj: Obj<'a>,
}

impl<'a> Bar<'a> {
    /// Create a new bar as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_bar_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the bar value
    ///
    /// # Arguments
    ///
    /// * `value` - The new value
    /// * `anim` - Whether to animate the change
    pub fn set_value(&self, value: i32, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_bar_set_value(self.obj.raw(), value, anim);
        }
    }

    /// Set the start value (for range mode)
    pub fn set_start_value(&self, value: i32, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_bar_set_start_value(self.obj.raw(), value, anim);
        }
    }

    /// Set the value range
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe {
            neo_lvgl_sys::lv_bar_set_range(self.obj.raw(), min, max);
        }
    }

    /// Set minimum value
    pub fn set_min_value(&self, min: i32) {
        unsafe {
            neo_lvgl_sys::lv_bar_set_min_value(self.obj.raw(), min);
        }
    }

    /// Set maximum value
    pub fn set_max_value(&self, max: i32) {
        unsafe {
            neo_lvgl_sys::lv_bar_set_max_value(self.obj.raw(), max);
        }
    }

    /// Set the bar mode
    pub fn set_mode(&self, mode: BarMode) {
        unsafe {
            neo_lvgl_sys::lv_bar_set_mode(self.obj.raw(), mode.to_raw());
        }
    }

    /// Set the bar orientation
    pub fn set_orientation(&self, orientation: BarOrientation) {
        unsafe {
            neo_lvgl_sys::lv_bar_set_orientation(self.obj.raw(), orientation.to_raw());
        }
    }

    /// Get the current value
    pub fn value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_bar_get_value(self.obj.raw()) }
    }

    /// Get the start value (for range mode)
    pub fn start_value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_bar_get_start_value(self.obj.raw()) }
    }

    /// Get the minimum value
    pub fn min_value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_bar_get_min_value(self.obj.raw()) }
    }

    /// Get the maximum value
    pub fn max_value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_bar_get_max_value(self.obj.raw()) }
    }

    /// Get the bar mode
    pub fn mode(&self) -> BarMode {
        let raw = unsafe { neo_lvgl_sys::lv_bar_get_mode(self.obj.raw()) };
        BarMode::from_raw(raw)
    }

    /// Get the bar orientation
    pub fn orientation(&self) -> BarOrientation {
        let raw = unsafe { neo_lvgl_sys::lv_bar_get_orientation(self.obj.raw()) };
        BarOrientation::from_raw(raw)
    }

    /// Check if the bar is symmetrical
    pub fn is_symmetrical(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_bar_is_symmetrical(self.obj.raw()) }
    }
}

impl<'a> Widget<'a> for Bar<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Bar<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}


impl<'a> Bar<'a> {
    /// Bind this bar's value to an integer subject
    ///
    /// When the subject changes, the bar updates automatically.
    pub fn bind_value(
        &self,
        subject: &mut crate::observer::IntSubject,
    ) -> Option<crate::observer::Observer> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_bar_bind_value(self.obj.raw(), subject.raw());
            crate::observer::Observer::from_raw(ptr)
        }
    }
}


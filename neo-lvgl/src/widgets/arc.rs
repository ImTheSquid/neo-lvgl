//! Arc widget

use super::{Obj, Widget};
use crate::event::EventHandler;
use crate::observer::Subject;

/// Arc mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArcMode {
    /// Normal mode - clockwise
    Normal,
    /// Symmetrical mode - grows from both sides
    Symmetrical,
    /// Reverse mode - counter-clockwise
    Reverse,
}

impl ArcMode {
    fn to_raw(self) -> neo_lvgl_sys::lv_arc_mode_t {
        match self {
            ArcMode::Normal => neo_lvgl_sys::lv_arc_mode_t_LV_ARC_MODE_NORMAL,
            ArcMode::Symmetrical => neo_lvgl_sys::lv_arc_mode_t_LV_ARC_MODE_SYMMETRICAL,
            ArcMode::Reverse => neo_lvgl_sys::lv_arc_mode_t_LV_ARC_MODE_REVERSE,
        }
    }

    fn from_raw(raw: neo_lvgl_sys::lv_arc_mode_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_arc_mode_t_LV_ARC_MODE_NORMAL => ArcMode::Normal,
            neo_lvgl_sys::lv_arc_mode_t_LV_ARC_MODE_SYMMETRICAL => ArcMode::Symmetrical,
            neo_lvgl_sys::lv_arc_mode_t_LV_ARC_MODE_REVERSE => ArcMode::Reverse,
            _ => ArcMode::Normal,
        }
    }
}

/// Arc widget
///
/// A circular arc that can display a value (like a circular progress bar)
/// or be used as a knob/dial.
///
/// # Example
///
/// ```ignore
/// let arc = Arc::new(&screen).unwrap();
/// arc.set_range(0, 100);
/// arc.set_value(75);
/// arc.set_bg_angles(0.0, 360.0); // Full circle background
/// ```
#[derive(Clone, Copy)]
pub struct Arc<'a> {
    obj: Obj<'a>,
}

impl<'a> Arc<'a> {
    /// Create a new arc as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_arc_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the arc indicator angles (the "filled" part)
    ///
    /// Angles are in degrees (0-360).
    pub fn set_angles(&self, start: f32, end: f32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_angles(self.obj.raw(), start as i32, end as i32);
        }
    }

    /// Set the start angle of the arc indicator
    pub fn set_start_angle(&self, angle: f32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_start_angle(self.obj.raw(), angle as i32);
        }
    }

    /// Set the end angle of the arc indicator
    pub fn set_end_angle(&self, angle: f32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_end_angle(self.obj.raw(), angle as i32);
        }
    }

    /// Set the background arc angles
    pub fn set_bg_angles(&self, start: f32, end: f32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_bg_angles(self.obj.raw(), start as i32, end as i32);
        }
    }

    /// Set the background arc start angle
    pub fn set_bg_start_angle(&self, angle: f32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_bg_start_angle(self.obj.raw(), angle as i32);
        }
    }

    /// Set the background arc end angle
    pub fn set_bg_end_angle(&self, angle: f32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_bg_end_angle(self.obj.raw(), angle as i32);
        }
    }

    /// Set rotation offset (0 degrees = 3 o'clock by default)
    pub fn set_rotation(&self, rotation: i32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_rotation(self.obj.raw(), rotation);
        }
    }

    /// Set the arc mode
    pub fn set_mode(&self, mode: ArcMode) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_mode(self.obj.raw(), mode.to_raw());
        }
    }

    /// Set the arc value
    pub fn set_value(&self, value: i32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_value(self.obj.raw(), value);
        }
    }

    /// Set the value range
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_range(self.obj.raw(), min, max);
        }
    }

    /// Set the minimum value
    pub fn set_min_value(&self, min: i32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_min_value(self.obj.raw(), min);
        }
    }

    /// Set the maximum value
    pub fn set_max_value(&self, max: i32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_max_value(self.obj.raw(), max);
        }
    }

    /// Set the change rate (for fine-tuning dragging sensitivity)
    pub fn set_change_rate(&self, rate: u32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_change_rate(self.obj.raw(), rate);
        }
    }

    /// Set knob offset
    pub fn set_knob_offset(&self, offset: i32) {
        unsafe {
            neo_lvgl_sys::lv_arc_set_knob_offset(self.obj.raw(), offset);
        }
    }

    /// Get the arc indicator start angle
    pub fn start_angle(&self) -> f32 {
        unsafe { neo_lvgl_sys::lv_arc_get_angle_start(self.obj.raw()) as f32 }
    }

    /// Get the arc indicator end angle
    pub fn end_angle(&self) -> f32 {
        unsafe { neo_lvgl_sys::lv_arc_get_angle_end(self.obj.raw()) as f32 }
    }

    /// Get the background arc start angle
    pub fn bg_start_angle(&self) -> f32 {
        unsafe { neo_lvgl_sys::lv_arc_get_bg_angle_start(self.obj.raw()) as f32 }
    }

    /// Get the background arc end angle
    pub fn bg_end_angle(&self) -> f32 {
        unsafe { neo_lvgl_sys::lv_arc_get_bg_angle_end(self.obj.raw()) as f32 }
    }

    /// Get the current value
    pub fn value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_arc_get_value(self.obj.raw()) }
    }

    /// Get the minimum value
    pub fn min_value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_arc_get_min_value(self.obj.raw()) }
    }

    /// Get the maximum value
    pub fn max_value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_arc_get_max_value(self.obj.raw()) }
    }

    /// Get the arc mode
    pub fn mode(&self) -> ArcMode {
        let raw = unsafe { neo_lvgl_sys::lv_arc_get_mode(self.obj.raw()) };
        ArcMode::from_raw(raw)
    }

    /// Get the rotation offset
    pub fn rotation(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_arc_get_rotation(self.obj.raw()) }
    }

    /// Get the knob offset
    pub fn knob_offset(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_arc_get_knob_offset(self.obj.raw()) }
    }
}

impl<'a> Widget<'a> for Arc<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Arc<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}


impl<'a> Arc<'a> {
    /// Bind this arc's value to an integer subject
    ///
    /// When the subject changes, the arc updates automatically.
    /// When the arc value changes, the subject is updated.
    pub fn bind_value(
        &self,
        subject: &mut crate::observer::IntSubject,
    ) -> Option<crate::observer::Observer> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_arc_bind_value(self.obj.raw(), subject.raw());
            crate::observer::Observer::from_raw(ptr)
        }
    }
}


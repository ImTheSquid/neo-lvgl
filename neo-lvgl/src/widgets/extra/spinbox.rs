//! Spinbox widget

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};

/// Spinbox widget
///
/// A numeric input with increment/decrement buttons.
///
/// # Example
///
/// ```ignore
/// let spinbox = Spinbox::new(&screen).unwrap();
/// spinbox.set_range(-100, 100);
/// spinbox.set_value(50);
/// spinbox.set_step(5);
/// spinbox.set_digit_format(3, 0); // 3 digits, no decimal places
/// ```
#[derive(Clone, Copy)]
pub struct Spinbox<'a> {
    obj: Obj<'a>,
}

impl<'a> Spinbox<'a> {
    /// Create a new spinbox as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_spinbox_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the value
    pub fn set_value(&self, value: i32) {
        unsafe {
            neo_lvgl_sys::lv_spinbox_set_value(self.obj.raw(), value);
        }
    }

    /// Get the current value
    pub fn value(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_spinbox_get_value(self.obj.raw()) }
    }

    /// Set the value range
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe {
            neo_lvgl_sys::lv_spinbox_set_range(self.obj.raw(), min, max);
        }
    }

    /// Set the step value for increment/decrement
    pub fn set_step(&self, step: u32) {
        unsafe {
            neo_lvgl_sys::lv_spinbox_set_step(self.obj.raw(), step);
        }
    }

    /// Get the current step value
    pub fn step(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_spinbox_get_step(self.obj.raw()) }
    }

    /// Set the digit format
    ///
    /// # Arguments
    ///
    /// * `digit_count` - Total number of digits
    /// * `separator_position` - Position of decimal separator from right (0 = no separator)
    pub fn set_digit_format(&self, digit_count: u32, separator_position: u32) {
        unsafe {
            neo_lvgl_sys::lv_spinbox_set_digit_format(
                self.obj.raw(),
                digit_count,
                separator_position,
            );
        }
    }

    /// Set the cursor position (which digit to modify)
    pub fn set_cursor_pos(&self, pos: u32) {
        unsafe {
            neo_lvgl_sys::lv_spinbox_set_cursor_pos(self.obj.raw(), pos);
        }
    }

    /// Move the cursor left
    pub fn cursor_left(&self) {
        unsafe {
            neo_lvgl_sys::lv_spinbox_step_next(self.obj.raw());
        }
    }

    /// Move the cursor right
    pub fn cursor_right(&self) {
        unsafe {
            neo_lvgl_sys::lv_spinbox_step_prev(self.obj.raw());
        }
    }

    /// Increment the value by the step
    pub fn increment(&self) {
        unsafe {
            neo_lvgl_sys::lv_spinbox_increment(self.obj.raw());
        }
    }

    /// Decrement the value by the step
    pub fn decrement(&self) {
        unsafe {
            neo_lvgl_sys::lv_spinbox_decrement(self.obj.raw());
        }
    }

    /// Enable/disable rollover (wrap from max to min and vice versa)
    pub fn set_rollover(&self, enable: bool) {
        unsafe {
            neo_lvgl_sys::lv_spinbox_set_rollover(self.obj.raw(), enable);
        }
    }

    /// Check if rollover is enabled
    pub fn rollover(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_spinbox_get_rollover(self.obj.raw()) }
    }

    /// Set whether to show digit step cursor highlight
    pub fn set_digit_step_direction(&self, direction: bool) {
        let dir = if direction {
            neo_lvgl_sys::lv_dir_t_LV_DIR_RIGHT
        } else {
            neo_lvgl_sys::lv_dir_t_LV_DIR_LEFT
        };
        unsafe {
            neo_lvgl_sys::lv_spinbox_set_digit_step_direction(self.obj.raw(), dir);
        }
    }
}

impl<'a> Widget<'a> for Spinbox<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Spinbox<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



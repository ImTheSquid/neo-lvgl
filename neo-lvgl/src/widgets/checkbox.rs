//! Checkbox widget

use super::{Obj, Widget, State};
use crate::event::EventHandler;
use core::ffi::CStr;

/// Checkbox widget
///
/// A checkbox with an optional text label.
///
/// # Example
///
/// ```ignore
/// let cb = Checkbox::new(&screen).unwrap();
/// cb.set_text(c"Enable notifications");
///
/// if cb.is_checked() {
///     // ...
/// }
/// ```
#[derive(Clone, Copy)]
pub struct Checkbox<'a> {
    obj: Obj<'a>,
}

impl<'a> Checkbox<'a> {
    /// Create a new checkbox as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_checkbox_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the checkbox text
    pub fn set_text(&self, text: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_checkbox_set_text(self.obj.raw(), text.as_ptr().cast());
        }
    }

    /// Set the checkbox text (static, no copy)
    pub fn set_text_static(&self, text: &'static CStr) {
        unsafe {
            neo_lvgl_sys::lv_checkbox_set_text_static(self.obj.raw(), text.as_ptr().cast());
        }
    }

    /// Get the checkbox text
    pub fn text(&self) -> &CStr {
        unsafe {
            let ptr = neo_lvgl_sys::lv_checkbox_get_text(self.obj.raw());
            CStr::from_ptr(ptr.cast())
        }
    }

    /// Check if the checkbox is checked
    pub fn is_checked(&self) -> bool {
        self.obj.has_state(State::CHECKED)
    }

    /// Set the checked state
    pub fn set_checked(&self, checked: bool) {
        if checked {
            self.obj.add_state(State::CHECKED);
        } else {
            self.obj.remove_state(State::CHECKED);
        }
    }
}

impl<'a> Widget<'a> for Checkbox<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Checkbox<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



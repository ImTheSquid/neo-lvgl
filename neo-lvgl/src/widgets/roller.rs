//! Roller widget

use super::{Obj, Widget};
use crate::event::EventHandler;
use crate::observer::Subject;
use core::ffi::CStr;

/// Roller mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RollerMode {
    /// Normal mode - stops at first/last option
    Normal,
    /// Infinite mode - wraps around
    Infinite,
}

impl RollerMode {
    fn to_raw(self) -> neo_lvgl_sys::lv_roller_mode_t {
        match self {
            RollerMode::Normal => neo_lvgl_sys::lv_roller_mode_t_LV_ROLLER_MODE_NORMAL,
            RollerMode::Infinite => neo_lvgl_sys::lv_roller_mode_t_LV_ROLLER_MODE_INFINITE,
        }
    }
}

/// Roller widget
///
/// A scrollable selector that looks like a wheel/drum.
///
/// # Example
///
/// ```ignore
/// let roller = Roller::new(&screen).unwrap();
/// roller.set_options(c"Mon\nTue\nWed\nThu\nFri\nSat\nSun", RollerMode::Normal);
/// roller.set_visible_row_count(3);
/// roller.set_selected(0, false);
/// ```
#[derive(Clone, Copy)]
pub struct Roller<'a> {
    obj: Obj<'a>,
}

impl<'a> Roller<'a> {
    /// Create a new roller as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_roller_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the options (newline-separated string) and mode
    pub fn set_options(&self, options: &CStr, mode: RollerMode) {
        unsafe {
            neo_lvgl_sys::lv_roller_set_options(self.obj.raw(), options.as_ptr().cast(), mode.to_raw());
        }
    }

    /// Set the selected option index
    ///
    /// # Arguments
    ///
    /// * `index` - Option index to select
    /// * `anim` - Whether to animate the change
    pub fn set_selected(&self, index: u32, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_roller_set_selected(self.obj.raw(), index, anim);
        }
    }

    /// Set the number of visible rows
    pub fn set_visible_row_count(&self, count: u32) {
        unsafe {
            neo_lvgl_sys::lv_roller_set_visible_row_count(self.obj.raw(), count);
        }
    }

    /// Get the selected option index
    pub fn selected(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_roller_get_selected(self.obj.raw()) }
    }

    /// Get the selected option text into a buffer
    pub fn selected_str(&self, buf: &mut [u8]) {
        unsafe {
            neo_lvgl_sys::lv_roller_get_selected_str(
                self.obj.raw(),
                buf.as_mut_ptr() as *mut _,
                buf.len() as u32,
            );
        }
    }

    /// Get all options as a string
    pub fn options(&self) -> &CStr {
        unsafe {
            let ptr = neo_lvgl_sys::lv_roller_get_options(self.obj.raw());
            CStr::from_ptr(ptr.cast())
        }
    }

    /// Get the number of options
    pub fn option_count(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_roller_get_option_count(self.obj.raw()) }
    }
}

impl<'a> Widget<'a> for Roller<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Roller<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}


impl<'a> Roller<'a> {
    /// Bind this roller's selected index to an integer subject
    ///
    /// When the subject changes, the roller selection updates automatically.
    /// When the user selects an option, the subject is updated.
    pub fn bind_value(
        &self,
        subject: &mut crate::observer::IntSubject,
    ) -> Option<crate::observer::Observer> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_roller_bind_value(self.obj.raw(), subject.raw());
            crate::observer::Observer::from_raw(ptr)
        }
    }
}


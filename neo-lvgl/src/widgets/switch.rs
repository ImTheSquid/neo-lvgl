//! Switch widget

use super::{Obj, Widget, State};
use crate::event::EventHandler;

/// Switch orientation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchOrientation {
    /// Automatically determine orientation based on size
    Auto,
    /// Horizontal switch
    Horizontal,
    /// Vertical switch
    Vertical,
}

impl SwitchOrientation {
    fn to_raw(self) -> neo_lvgl_sys::lv_switch_orientation_t {
        match self {
            SwitchOrientation::Auto => neo_lvgl_sys::lv_switch_orientation_t_LV_SWITCH_ORIENTATION_AUTO,
            SwitchOrientation::Horizontal => neo_lvgl_sys::lv_switch_orientation_t_LV_SWITCH_ORIENTATION_HORIZONTAL,
            SwitchOrientation::Vertical => neo_lvgl_sys::lv_switch_orientation_t_LV_SWITCH_ORIENTATION_VERTICAL,
        }
    }

    fn from_raw(raw: neo_lvgl_sys::lv_switch_orientation_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_switch_orientation_t_LV_SWITCH_ORIENTATION_AUTO => SwitchOrientation::Auto,
            neo_lvgl_sys::lv_switch_orientation_t_LV_SWITCH_ORIENTATION_HORIZONTAL => SwitchOrientation::Horizontal,
            neo_lvgl_sys::lv_switch_orientation_t_LV_SWITCH_ORIENTATION_VERTICAL => SwitchOrientation::Vertical,
            _ => SwitchOrientation::Auto,
        }
    }
}

/// Switch widget (toggle)
///
/// A toggle switch for on/off states.
///
/// # Example
///
/// ```ignore
/// let sw = Switch::new(&screen).unwrap();
/// sw.set_checked(true);
///
/// sw.on_value_changed(|_| {
///     println!("Switch toggled!");
/// });
/// ```
#[derive(Clone, Copy)]
pub struct Switch<'a> {
    obj: Obj<'a>,
}

impl<'a> Switch<'a> {
    /// Create a new switch as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_switch_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Check if the switch is on
    pub fn is_checked(&self) -> bool {
        self.obj.has_state(State::CHECKED)
    }

    /// Set the switch state
    pub fn set_checked(&self, checked: bool) {
        if checked {
            self.obj.add_state(State::CHECKED);
        } else {
            self.obj.remove_state(State::CHECKED);
        }
    }

    /// Set the switch orientation
    pub fn set_orientation(&self, orientation: SwitchOrientation) {
        unsafe {
            neo_lvgl_sys::lv_switch_set_orientation(self.obj.raw(), orientation.to_raw());
        }
    }

    /// Get the switch orientation
    pub fn orientation(&self) -> SwitchOrientation {
        let raw = unsafe { neo_lvgl_sys::lv_switch_get_orientation(self.obj.raw()) };
        SwitchOrientation::from_raw(raw)
    }
}

impl<'a> Widget<'a> for Switch<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Switch<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



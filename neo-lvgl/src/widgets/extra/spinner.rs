//! Spinner widget

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};

/// Spinner widget
///
/// A rotating arc that indicates loading/processing.
///
/// # Example
///
/// ```ignore
/// let spinner = Spinner::new(&screen).unwrap();
/// // Customize the animation
/// spinner.set_anim_params(1000, 60); // 1 second rotation, 60 degree arc
/// ```
#[derive(Clone, Copy)]
pub struct Spinner<'a> {
    obj: Obj<'a>,
}

impl<'a> Spinner<'a> {
    /// Create a new spinner as a child of the given parent.
    ///
    /// Uses default animation parameters (1000ms, 90 degrees).
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_spinner_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set animation parameters
    ///
    /// # Arguments
    ///
    /// * `duration_ms` - Time for one full rotation in milliseconds
    /// * `arc_length` - Length of the rotating arc in degrees
    pub fn set_anim_params(&self, duration_ms: u32, arc_length: u32) {
        unsafe {
            neo_lvgl_sys::lv_spinner_set_anim_params(self.obj.raw(), duration_ms, arc_length);
        }
    }
}

impl<'a> Widget<'a> for Spinner<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Spinner<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



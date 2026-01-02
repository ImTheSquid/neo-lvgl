//! Group and focus management for LVGL
//!
//! Groups allow widgets to be navigated using a keyboard or encoder.
//!
//! # Example
//!
//! ```ignore
//! use lvgl::group::Group;
//! use lvgl::indev::Indev;
//!
//! let group = Group::new().unwrap();
//! group.add(&button1);
//! group.add(&button2);
//! group.set_default();
//!
//! // Attach to keypad input device
//! indev.set_group(&group);
//! ```

use crate::indev::Key;
use crate::widgets::{Obj, Widget};
use core::ffi::c_void;
use core::ptr::NonNull;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Refocus policy when focused widget is deleted
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefocusPolicy {
    /// Focus the next widget
    Next,
    /// Focus the previous widget
    Prev,
}

impl RefocusPolicy {
    fn to_raw(self) -> neo_lvgl_sys::lv_group_refocus_policy_t {
        match self {
            RefocusPolicy::Next => neo_lvgl_sys::lv_group_refocus_policy_t_LV_GROUP_REFOCUS_POLICY_NEXT,
            RefocusPolicy::Prev => neo_lvgl_sys::lv_group_refocus_policy_t_LV_GROUP_REFOCUS_POLICY_PREV,
        }
    }
}

/// A group of widgets for keyboard/encoder navigation
pub struct Group {
    raw: NonNull<neo_lvgl_sys::lv_group_t>,
}

impl Group {
    /// Create a new group
    pub fn new() -> Option<Self> {
        let ptr = unsafe { neo_lvgl_sys::lv_group_create() };
        NonNull::new(ptr).map(|raw| Self { raw })
    }

    /// Create from raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be valid.
    pub unsafe fn from_raw(raw: *mut neo_lvgl_sys::lv_group_t) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Set this group as the default
    pub fn set_default(&self) {
        unsafe {
            neo_lvgl_sys::lv_group_set_default(self.raw.as_ptr());
        }
    }

    /// Add a widget to the group
    pub fn add<'a, W: Widget<'a>>(&self, widget: &W) {
        unsafe {
            neo_lvgl_sys::lv_group_add_obj(self.raw.as_ptr(), widget.raw());
        }
    }

    /// Remove a widget from the group
    pub fn remove<'a, W: Widget<'a>>(&self, widget: &W) {
        unsafe {
            neo_lvgl_sys::lv_group_remove_obj(widget.raw());
        }
    }

    /// Remove all widgets from the group
    pub fn remove_all(&self) {
        unsafe {
            neo_lvgl_sys::lv_group_remove_all_objs(self.raw.as_ptr());
        }
    }

    /// Focus the next widget in the group
    pub fn focus_next(&self) {
        unsafe {
            neo_lvgl_sys::lv_group_focus_next(self.raw.as_ptr());
        }
    }

    /// Focus the previous widget in the group
    pub fn focus_prev(&self) {
        unsafe {
            neo_lvgl_sys::lv_group_focus_prev(self.raw.as_ptr());
        }
    }

    /// Focus a specific widget
    pub fn focus<'a, W: Widget<'a>>(&self, widget: &W) {
        unsafe {
            neo_lvgl_sys::lv_group_focus_obj(widget.raw());
        }
    }

    /// Freeze/unfreeze focus (prevent focus changes)
    pub fn set_focus_frozen(&self, frozen: bool) {
        unsafe {
            neo_lvgl_sys::lv_group_focus_freeze(self.raw.as_ptr(), frozen);
        }
    }

    /// Get the currently focused widget
    pub fn focused(&self) -> Option<Obj<'static>> {
        let ptr = unsafe { neo_lvgl_sys::lv_group_get_focused(self.raw.as_ptr()) };
        unsafe { Obj::from_raw(ptr) }
    }

    /// Send a key to the focused widget
    pub fn send_key(&self, key: Key) {
        unsafe {
            neo_lvgl_sys::lv_group_send_data(self.raw.as_ptr(), key.to_raw());
        }
    }

    /// Set wrap mode (whether navigation wraps around)
    pub fn set_wrap(&self, wrap: bool) {
        unsafe {
            neo_lvgl_sys::lv_group_set_wrap(self.raw.as_ptr(), wrap);
        }
    }

    /// Get wrap mode
    pub fn wrap(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_group_get_wrap(self.raw.as_ptr()) }
    }

    /// Set editing mode
    ///
    /// In editing mode, keys are sent to the focused widget for editing.
    /// In navigation mode, keys navigate between widgets.
    pub fn set_editing(&self, editing: bool) {
        unsafe {
            neo_lvgl_sys::lv_group_set_editing(self.raw.as_ptr(), editing);
        }
    }

    /// Get editing mode
    pub fn is_editing(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_group_get_editing(self.raw.as_ptr()) }
    }

    /// Set the refocus policy
    pub fn set_refocus_policy(&self, policy: RefocusPolicy) {
        unsafe {
            neo_lvgl_sys::lv_group_set_refocus_policy(self.raw.as_ptr(), policy.to_raw());
        }
    }

    /// Get the number of widgets in the group
    pub fn obj_count(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_group_get_obj_count(self.raw.as_ptr()) }
    }

    /// Set user data
    pub fn set_user_data(&self, data: *mut c_void) {
        unsafe {
            neo_lvgl_sys::lv_group_set_user_data(self.raw.as_ptr(), data);
        }
    }

    /// Get user data
    pub fn user_data(&self) -> *mut c_void {
        unsafe { neo_lvgl_sys::lv_group_get_user_data(self.raw.as_ptr()) }
    }

    /// Delete the group
    pub fn delete(self) {
        unsafe {
            neo_lvgl_sys::lv_group_delete(self.raw.as_ptr());
        }
    }

    /// Get the raw pointer
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_group_t {
        self.raw.as_ptr()
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new().expect("Failed to create group")
    }
}

// Callback support
#[cfg(feature = "alloc")]
mod callback_support {
    use super::*;

    /// Container for focus callback closure
    struct FocusClosure {
        callback: Box<dyn FnMut()>,
    }

    /// Trampoline for focus callback
    unsafe extern "C" fn focus_trampoline(group: *mut neo_lvgl_sys::lv_group_t) {
        let user_data = neo_lvgl_sys::lv_group_get_user_data(group);
        if !user_data.is_null() {
            let closure = &mut *(user_data as *mut FocusClosure);
            (closure.callback)();
        }
    }

    impl Group {
        /// Set focus change callback using a closure
        pub fn on_focus<F>(&self, callback: F)
        where
            F: FnMut() + 'static,
        {
            let closure = Box::new(FocusClosure {
                callback: Box::new(callback),
            });
            let raw_closure = Box::into_raw(closure);

            unsafe {
                neo_lvgl_sys::lv_group_set_user_data(self.raw.as_ptr(), raw_closure as *mut c_void);
                neo_lvgl_sys::lv_group_set_focus_cb(self.raw.as_ptr(), Some(focus_trampoline));
            }
        }
    }
}

/// Get the default group
pub fn get_default_group() -> Option<Group> {
    let ptr = unsafe { neo_lvgl_sys::lv_group_get_default() };
    unsafe { Group::from_raw(ptr) }
}

//! ButtonMatrix widget

use super::{Obj, Widget};
use crate::event::EventHandler;
use bitflags::bitflags;
use core::ffi::{c_char, CStr};

bitflags! {
    /// Button control flags for ButtonMatrix
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ButtonMatrixCtrl: u16 {
        /// Hide the button
        const HIDDEN = neo_lvgl_sys::lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_HIDDEN as u16;
        /// Disable repeat when long pressed
        const NO_REPEAT = neo_lvgl_sys::lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_NO_REPEAT as u16;
        /// Disable the button
        const DISABLED = neo_lvgl_sys::lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_DISABLED as u16;
        /// Button can be checked/toggled
        const CHECKABLE = neo_lvgl_sys::lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_CHECKABLE as u16;
        /// Button is currently checked
        const CHECKED = neo_lvgl_sys::lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_CHECKED as u16;
        /// Send click event on press instead of release
        const CLICK_TRIG = neo_lvgl_sys::lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_CLICK_TRIG as u16;
        /// Show button text in a popover
        const POPOVER = neo_lvgl_sys::lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_POPOVER as u16;
        /// Enable text recoloring
        const RECOLOR = neo_lvgl_sys::lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_RECOLOR as u16;
    }
}

/// Special value indicating no button is selected
pub const BUTTON_NONE: u32 = neo_lvgl_sys::LV_BUTTONMATRIX_BUTTON_NONE as u32;

/// ButtonMatrix widget
///
/// A matrix of buttons defined by a string map.
///
/// # Map Format
///
/// - Each string in the slice is a button label
/// - Use "\n" to start a new row
/// - The slice must end with an empty string ""
///
/// # Example
///
/// ```ignore
/// let btnm = ButtonMatrix::new(&screen).unwrap();
/// // Creates a 3x3 button grid
/// btnm.set_map(&[c"1", c"2", c"3", c"\n",
///                c"4", c"5", c"6", c"\n",
///                c"7", c"8", c"9", c""]);
/// ```
#[derive(Clone, Copy)]
pub struct ButtonMatrix<'a> {
    obj: Obj<'a>,
}

impl<'a> ButtonMatrix<'a> {
    /// Create a new button matrix as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_buttonmatrix_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the button map
    ///
    /// The map must remain valid for the lifetime of the widget.
    /// Each string is a button label. Use "\n" to start a new row.
    /// The array must end with an empty string "".
    ///
    /// # Safety
    ///
    /// The map array and its strings must remain valid.
    pub unsafe fn set_map(&self, map: *const *const c_char) {
        neo_lvgl_sys::lv_buttonmatrix_set_map(self.obj.raw(), map.cast());
    }

    /// Set control flags for a specific button
    pub fn set_button_ctrl(&self, btn_id: u32, ctrl: ButtonMatrixCtrl) {
        unsafe {
            neo_lvgl_sys::lv_buttonmatrix_set_button_ctrl(self.obj.raw(), btn_id, ctrl.bits() as u32);
        }
    }

    /// Clear control flags for a specific button
    pub fn clear_button_ctrl(&self, btn_id: u32, ctrl: ButtonMatrixCtrl) {
        unsafe {
            neo_lvgl_sys::lv_buttonmatrix_clear_button_ctrl(self.obj.raw(), btn_id, ctrl.bits() as u32);
        }
    }

    /// Set control flags for all buttons
    pub fn set_button_ctrl_all(&self, ctrl: ButtonMatrixCtrl) {
        unsafe {
            neo_lvgl_sys::lv_buttonmatrix_set_button_ctrl_all(self.obj.raw(), ctrl.bits() as u32);
        }
    }

    /// Clear control flags for all buttons
    pub fn clear_button_ctrl_all(&self, ctrl: ButtonMatrixCtrl) {
        unsafe {
            neo_lvgl_sys::lv_buttonmatrix_clear_button_ctrl_all(self.obj.raw(), ctrl.bits() as u32);
        }
    }

    /// Set the width of a button (relative units)
    pub fn set_button_width(&self, btn_id: u32, width: u32) {
        unsafe {
            neo_lvgl_sys::lv_buttonmatrix_set_button_width(self.obj.raw(), btn_id, width);
        }
    }

    /// Select a button
    pub fn set_selected(&self, btn_id: u32) {
        unsafe {
            neo_lvgl_sys::lv_buttonmatrix_set_selected_button(self.obj.raw(), btn_id);
        }
    }

    /// Enable one-checked mode (only one button can be checked at a time)
    pub fn set_one_checked(&self, en: bool) {
        unsafe {
            neo_lvgl_sys::lv_buttonmatrix_set_one_checked(self.obj.raw(), en);
        }
    }

    /// Get the selected button index
    pub fn selected(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_buttonmatrix_get_selected_button(self.obj.raw()) }
    }

    /// Get the text of a button
    pub fn button_text(&self, btn_id: u32) -> Option<&CStr> {
        let ptr = unsafe { neo_lvgl_sys::lv_buttonmatrix_get_button_text(self.obj.raw(), btn_id) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr.cast()) })
        }
    }

    /// Check if a button has specific control flags
    pub fn has_button_ctrl(&self, btn_id: u32, ctrl: ButtonMatrixCtrl) -> bool {
        unsafe {
            neo_lvgl_sys::lv_buttonmatrix_has_button_ctrl(self.obj.raw(), btn_id, ctrl.bits() as u32)
        }
    }

    /// Check if one-checked mode is enabled
    pub fn one_checked(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_buttonmatrix_get_one_checked(self.obj.raw()) }
    }
}

impl<'a> Widget<'a> for ButtonMatrix<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for ButtonMatrix<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



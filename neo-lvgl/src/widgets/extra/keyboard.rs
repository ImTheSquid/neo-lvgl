//! Keyboard widget

use crate::event::EventHandler;
use crate::widgets::{Obj, TextArea, Widget};
use core::ffi::c_char;

/// Keyboard mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardMode {
    /// Lowercase letters
    TextLower,
    /// Uppercase letters
    TextUpper,
    /// Special characters
    Special,
    /// Numeric input
    Number,
    /// User-defined map
    UserDefined,
}

impl KeyboardMode {
    fn to_raw(self) -> neo_lvgl_sys::lv_keyboard_mode_t {
        match self {
            KeyboardMode::TextLower => neo_lvgl_sys::lv_keyboard_mode_t_LV_KEYBOARD_MODE_TEXT_LOWER,
            KeyboardMode::TextUpper => neo_lvgl_sys::lv_keyboard_mode_t_LV_KEYBOARD_MODE_TEXT_UPPER,
            KeyboardMode::Special => neo_lvgl_sys::lv_keyboard_mode_t_LV_KEYBOARD_MODE_SPECIAL,
            KeyboardMode::Number => neo_lvgl_sys::lv_keyboard_mode_t_LV_KEYBOARD_MODE_NUMBER,
            KeyboardMode::UserDefined => neo_lvgl_sys::lv_keyboard_mode_t_LV_KEYBOARD_MODE_USER_1,
        }
    }

    fn from_raw(raw: neo_lvgl_sys::lv_keyboard_mode_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_keyboard_mode_t_LV_KEYBOARD_MODE_TEXT_LOWER => KeyboardMode::TextLower,
            neo_lvgl_sys::lv_keyboard_mode_t_LV_KEYBOARD_MODE_TEXT_UPPER => KeyboardMode::TextUpper,
            neo_lvgl_sys::lv_keyboard_mode_t_LV_KEYBOARD_MODE_SPECIAL => KeyboardMode::Special,
            neo_lvgl_sys::lv_keyboard_mode_t_LV_KEYBOARD_MODE_NUMBER => KeyboardMode::Number,
            _ => KeyboardMode::UserDefined,
        }
    }
}

/// On-screen keyboard widget
///
/// A virtual keyboard that can be linked to a TextArea.
///
/// # Example
///
/// ```ignore
/// let textarea = TextArea::new(&screen).unwrap();
/// let keyboard = Keyboard::new(&screen).unwrap();
/// keyboard.set_textarea(&textarea);
/// ```
#[derive(Clone, Copy)]
pub struct Keyboard<'a> {
    obj: Obj<'a>,
}

impl<'a> Keyboard<'a> {
    /// Create a new keyboard as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_keyboard_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the associated text area
    ///
    /// Keys pressed on the keyboard will be sent to this text area.
    pub fn set_textarea(&self, textarea: &TextArea) {
        unsafe {
            neo_lvgl_sys::lv_keyboard_set_textarea(self.obj.raw(), textarea.obj().raw());
        }
    }

    /// Set the keyboard mode
    pub fn set_mode(&self, mode: KeyboardMode) {
        unsafe {
            neo_lvgl_sys::lv_keyboard_set_mode(self.obj.raw(), mode.to_raw());
        }
    }

    /// Get the current keyboard mode
    pub fn mode(&self) -> KeyboardMode {
        let raw = unsafe { neo_lvgl_sys::lv_keyboard_get_mode(self.obj.raw()) };
        KeyboardMode::from_raw(raw)
    }

    /// Enable/disable popovers on key press
    pub fn set_popovers(&self, enable: bool) {
        unsafe {
            neo_lvgl_sys::lv_keyboard_set_popovers(self.obj.raw(), enable);
        }
    }

    /// Set a custom button map
    ///
    /// # Safety
    ///
    /// The map must remain valid for the lifetime of the keyboard.
    pub unsafe fn set_map(&self, mode: KeyboardMode, map: *const *const c_char, ctrl: *const u32) {
        neo_lvgl_sys::lv_keyboard_set_map(self.obj.raw(), mode.to_raw(), map, ctrl);
    }

    /// Get the currently selected button index
    pub fn selected_button(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_keyboard_get_selected_button(self.obj.raw()) }
    }

    /// Get the text of a specific button
    pub fn button_text(&self, btn_id: u32) -> Option<&core::ffi::CStr> {
        let ptr = unsafe { neo_lvgl_sys::lv_keyboard_get_button_text(self.obj.raw(), btn_id) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { core::ffi::CStr::from_ptr(ptr.cast()) })
        }
    }
}

impl<'a> Widget<'a> for Keyboard<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Keyboard<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



//! TextArea widget

use super::{Obj, Widget};
use crate::event::EventHandler;
use core::ffi::CStr;

/// Cursor position
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorPos {
    /// Specific index position
    Index(u32),
    /// After the last character
    Last,
    /// Offset from end (negative indexing)
    FromEnd(u32),
}

impl CursorPos {
    fn to_raw(self) -> i32 {
        match self {
            CursorPos::Index(i) => i as i32,
            CursorPos::Last => neo_lvgl_sys::LV_TEXTAREA_CURSOR_LAST as i32,
            CursorPos::FromEnd(n) => -(n as i32),
        }
    }
}

/// TextArea widget
///
/// A text input area with cursor support.
///
/// # Example
///
/// ```ignore
/// let ta = TextArea::new(&screen).unwrap();
/// ta.set_placeholder(c"Enter text...");
/// ta.set_one_line(true);
/// ta.set_max_length(Some(32));
/// ```
#[derive(Clone, Copy)]
pub struct TextArea<'a> {
    obj: Obj<'a>,
}

impl<'a> TextArea<'a> {
    /// Create a new textarea as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_textarea_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Add a character at the cursor position
    pub fn add_char(&self, c: char) {
        unsafe {
            neo_lvgl_sys::lv_textarea_add_char(self.obj.raw(), c as u32);
        }
    }

    /// Add text at the cursor position
    pub fn add_text(&self, text: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_textarea_add_text(self.obj.raw(), text.as_ptr().cast());
        }
    }

    /// Delete the character before the cursor
    pub fn delete_char(&self) {
        unsafe {
            neo_lvgl_sys::lv_textarea_delete_char(self.obj.raw());
        }
    }

    /// Delete the character after the cursor
    pub fn delete_char_forward(&self) {
        unsafe {
            neo_lvgl_sys::lv_textarea_delete_char_forward(self.obj.raw());
        }
    }

    /// Set the text content
    pub fn set_text(&self, text: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_textarea_set_text(self.obj.raw(), text.as_ptr().cast());
        }
    }

    /// Get the text content
    pub fn text(&self) -> &CStr {
        unsafe {
            let ptr = neo_lvgl_sys::lv_textarea_get_text(self.obj.raw());
            CStr::from_ptr(ptr.cast())
        }
    }

    /// Set placeholder text
    pub fn set_placeholder(&self, text: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_textarea_set_placeholder_text(self.obj.raw(), text.as_ptr().cast());
        }
    }

    /// Get placeholder text
    pub fn placeholder(&self) -> Option<&CStr> {
        let ptr = unsafe { neo_lvgl_sys::lv_textarea_get_placeholder_text(self.obj.raw()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr.cast()) })
        }
    }

    /// Set cursor position
    pub fn set_cursor_pos(&self, pos: CursorPos) {
        unsafe {
            neo_lvgl_sys::lv_textarea_set_cursor_pos(self.obj.raw(), pos.to_raw());
        }
    }

    /// Get cursor position
    pub fn cursor_pos(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_textarea_get_cursor_pos(self.obj.raw()) }
    }

    /// Enable/disable cursor click positioning
    pub fn set_cursor_click_pos(&self, en: bool) {
        unsafe {
            neo_lvgl_sys::lv_textarea_set_cursor_click_pos(self.obj.raw(), en);
        }
    }

    /// Check if cursor click positioning is enabled
    pub fn cursor_click_pos(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_textarea_get_cursor_click_pos(self.obj.raw()) }
    }

    /// Enable/disable password mode (shows bullets instead of characters)
    pub fn set_password_mode(&self, en: bool) {
        unsafe {
            neo_lvgl_sys::lv_textarea_set_password_mode(self.obj.raw(), en);
        }
    }

    /// Check if password mode is enabled
    pub fn password_mode(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_textarea_get_password_mode(self.obj.raw()) }
    }

    /// Set password bullet character
    pub fn set_password_bullet(&self, bullet: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_textarea_set_password_bullet(self.obj.raw(), bullet.as_ptr().cast());
        }
    }

    /// Set password show time (how long to show typed character before hiding)
    pub fn set_password_show_time(&self, time_ms: u32) {
        unsafe {
            neo_lvgl_sys::lv_textarea_set_password_show_time(self.obj.raw(), time_ms);
        }
    }

    /// Get password show time
    pub fn password_show_time(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_textarea_get_password_show_time(self.obj.raw()) }
    }

    /// Enable/disable one-line mode
    pub fn set_one_line(&self, en: bool) {
        unsafe {
            neo_lvgl_sys::lv_textarea_set_one_line(self.obj.raw(), en);
        }
    }

    /// Check if one-line mode is enabled
    pub fn one_line(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_textarea_get_one_line(self.obj.raw()) }
    }

    /// Set accepted characters (only these can be typed)
    pub fn set_accepted_chars(&self, chars: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_textarea_set_accepted_chars(self.obj.raw(), chars.as_ptr().cast());
        }
    }

    /// Get accepted characters
    pub fn accepted_chars(&self) -> Option<&CStr> {
        let ptr = unsafe { neo_lvgl_sys::lv_textarea_get_accepted_chars(self.obj.raw()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr.cast()) })
        }
    }

    /// Set maximum text length
    ///
    /// None = no limit
    pub fn set_max_length(&self, len: Option<u32>) {
        unsafe {
            neo_lvgl_sys::lv_textarea_set_max_length(self.obj.raw(), len.unwrap_or(0));
        }
    }

    /// Get maximum text length (0 = no limit)
    pub fn max_length(&self) -> Option<u32> {
        let len = unsafe { neo_lvgl_sys::lv_textarea_get_max_length(self.obj.raw()) };
        if len == 0 {
            None
        } else {
            Some(len)
        }
    }

    /// Enable/disable text selection
    pub fn set_text_selection(&self, en: bool) {
        unsafe {
            neo_lvgl_sys::lv_textarea_set_text_selection(self.obj.raw(), en);
        }
    }

    /// Check if text selection is enabled
    pub fn text_selection(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_textarea_get_text_selection(self.obj.raw()) }
    }

    /// Check if text is currently selected
    pub fn is_text_selected(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_textarea_text_is_selected(self.obj.raw()) }
    }

    /// Clear selection
    pub fn clear_selection(&self) {
        unsafe {
            neo_lvgl_sys::lv_textarea_clear_selection(self.obj.raw());
        }
    }

    /// Move cursor right
    pub fn cursor_right(&self) {
        unsafe {
            neo_lvgl_sys::lv_textarea_cursor_right(self.obj.raw());
        }
    }

    /// Move cursor left
    pub fn cursor_left(&self) {
        unsafe {
            neo_lvgl_sys::lv_textarea_cursor_left(self.obj.raw());
        }
    }

    /// Move cursor up
    pub fn cursor_up(&self) {
        unsafe {
            neo_lvgl_sys::lv_textarea_cursor_up(self.obj.raw());
        }
    }

    /// Move cursor down
    pub fn cursor_down(&self) {
        unsafe {
            neo_lvgl_sys::lv_textarea_cursor_down(self.obj.raw());
        }
    }

    /// Get the internal label widget
    pub fn label(&self) -> Option<super::Label<'a>> {
        let ptr = unsafe { neo_lvgl_sys::lv_textarea_get_label(self.obj.raw()) };
        unsafe { Obj::from_raw(ptr) }.map(|obj| super::Label::from_obj(obj))
    }
}

impl<'a> Widget<'a> for TextArea<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for TextArea<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



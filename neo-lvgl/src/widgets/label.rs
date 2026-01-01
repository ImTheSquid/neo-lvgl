//! Label widget

use super::{Obj, Widget};
use crate::event::EventHandler;
use core::ffi::CStr;

/// Label widget
///
/// Displays text. Supports multiline text, text recoloring, and various
/// text modes (wrap, scroll, etc.).
///
/// # Example
///
/// ```ignore
/// let label = Label::new(&screen).unwrap();
/// label.set_text(c"Hello, World!");
/// label.center();
/// ```
pub struct Label<'a> {
    obj: Obj<'a>,
}

impl<'a> Label<'a> {
    /// Create a new label as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_label_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the label text.
    ///
    /// # Arguments
    ///
    /// * `text` - A null-terminated C string (use `c"..."` literals in Rust 1.77+)
    pub fn set_text(&self, text: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_label_set_text(self.obj.raw(), text.as_ptr().cast());
        }
    }

    /// Set the label text from a static string.
    ///
    /// Unlike `set_text`, this doesn't copy the string, so it must remain valid.
    ///
    /// # Safety
    ///
    /// The string must remain valid for the lifetime of the label.
    pub unsafe fn set_text_static(&self, text: &'static CStr) {
        neo_lvgl_sys::lv_label_set_text_static(self.obj.raw(), text.as_ptr().cast());
    }

    /// Set text with format string (like printf).
    ///
    /// Note: In no_std environments, formatting support may be limited.
    #[cfg(feature = "alloc")]
    pub fn set_text_fmt(&self, text: &str) {
        use alloc::ffi::CString;
        if let Ok(cstring) = CString::new(text) {
            unsafe {
                neo_lvgl_sys::lv_label_set_text(self.obj.raw(), cstring.as_ptr().cast());
            }
        }
    }

    /// Get the current text.
    ///
    /// Returns `None` if the text is not valid UTF-8.
    pub fn text(&self) -> Option<&str> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_label_get_text(self.obj.raw());
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr.cast()).to_str().ok()
            }
        }
    }

    /// Set the long mode (how to handle text that doesn't fit).
    pub fn set_long_mode(&self, mode: LabelLongMode) {
        unsafe {
            neo_lvgl_sys::lv_label_set_long_mode(self.obj.raw(), mode.to_raw());
        }
    }

    /// Enable or disable text recoloring.
    ///
    /// When enabled, text can include color codes like `#FF0000 Red text#`.
    pub fn set_recolor(&self, enable: bool) {
        unsafe {
            neo_lvgl_sys::lv_label_set_recolor(self.obj.raw(), enable);
        }
    }

    // Note: lv_label_get_line_count doesn't exist in LVGL 9.x

    /// Set the text selection range.
    #[cfg(feature = "widgets-core")]
    pub fn set_text_selection(&self, start: u32, end: u32) {
        unsafe {
            neo_lvgl_sys::lv_label_set_text_selection_start(self.obj.raw(), start);
            neo_lvgl_sys::lv_label_set_text_selection_end(self.obj.raw(), end);
        }
    }
}

impl<'a> Widget<'a> for Label<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Label<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}

#[cfg(feature = "alloc")]
impl<'a> crate::event::ClosureEventHandler for Label<'a> {}

/// Label long mode - how to handle text that doesn't fit
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelLongMode {
    /// Wrap text to the next line (default)
    Wrap,
    /// Truncate and show dots at the end
    Dots,
    /// Scroll the text horizontally
    Scroll,
    /// Scroll the text circularly
    ScrollCircular,
    /// Keep the object size and clip the text
    Clip,
}

impl LabelLongMode {
    fn to_raw(self) -> neo_lvgl_sys::lv_label_long_mode_t {
        match self {
            LabelLongMode::Wrap => neo_lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_WRAP,
            LabelLongMode::Dots => neo_lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_DOTS,
            LabelLongMode::Scroll => neo_lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_SCROLL,
            LabelLongMode::ScrollCircular => {
                neo_lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_SCROLL_CIRCULAR
            }
            LabelLongMode::Clip => neo_lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_CLIP,
        }
    }
}

impl Default for LabelLongMode {
    fn default() -> Self {
        LabelLongMode::Wrap
    }
}

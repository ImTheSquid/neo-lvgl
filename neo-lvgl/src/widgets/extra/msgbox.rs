//! Message Box widget

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};
use core::ffi::CStr;

/// Message Box widget
///
/// A modal dialog box with title, message, and buttons.
///
/// # Example
///
/// ```ignore
/// // Create a simple message box
/// let msgbox = MsgBox::new(&screen, c"Title", c"Message text", &[c"OK", c"Cancel"], true).unwrap();
///
/// // Handle button clicks
/// msgbox.on_clicked(|btn_idx| {
///     match btn_idx {
///         0 => { /* OK pressed */ }
///         1 => { /* Cancel pressed */ }
///         _ => {}
///     }
/// });
/// ```
#[derive(Clone, Copy)]
pub struct MsgBox<'a> {
    obj: Obj<'a>,
}

impl<'a> MsgBox<'a> {
    /// Create a new message box
    ///
    /// # Arguments
    ///
    /// * `parent` - Parent widget (or screen)
    /// * `title` - Dialog title (can be empty)
    /// * `text` - Message text
    /// * `buttons` - Array of button labels (null-terminated strings)
    /// * `add_close_btn` - Whether to add a close button in the header
    ///
    /// # Safety
    ///
    /// The button labels must remain valid.
    pub fn new(
        parent: &'a impl Widget<'a>,
        title: &CStr,
        text: &CStr,
        add_close_btn: bool,
    ) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_msgbox_create(parent.raw());
            if ptr.is_null() {
                return None;
            }
            neo_lvgl_sys::lv_msgbox_add_title(ptr, title.as_ptr());
            neo_lvgl_sys::lv_msgbox_add_text(ptr, text.as_ptr());
            if add_close_btn {
                neo_lvgl_sys::lv_msgbox_add_close_button(ptr);
            }
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Add a button to the message box footer
    pub fn add_button(&self, text: &CStr) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_msgbox_add_footer_button(self.obj.raw(), text.as_ptr());
            Obj::from_raw(ptr)
        }
    }

    /// Get the title label object
    pub fn title(&self) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_msgbox_get_title(self.obj.raw());
            Obj::from_raw(ptr)
        }
    }

    /// Get the content area object (which contains the text)
    pub fn text(&self) -> Option<Obj<'a>> {
        // In LVGL 9, get the content area and find the label child
        self.content()
    }

    /// Get the content area object
    pub fn content(&self) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_msgbox_get_content(self.obj.raw());
            Obj::from_raw(ptr)
        }
    }

    /// Get the header object
    pub fn header(&self) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_msgbox_get_header(self.obj.raw());
            Obj::from_raw(ptr)
        }
    }

    /// Get the footer object
    pub fn footer(&self) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_msgbox_get_footer(self.obj.raw());
            Obj::from_raw(ptr)
        }
    }

    /// Close the message box (delete it)
    pub fn close(self) {
        unsafe {
            neo_lvgl_sys::lv_msgbox_close(self.obj.raw());
        }
    }

    /// Close the message box asynchronously
    pub fn close_async(&self) {
        unsafe {
            neo_lvgl_sys::lv_msgbox_close_async(self.obj.raw());
        }
    }
}

impl<'a> Widget<'a> for MsgBox<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for MsgBox<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



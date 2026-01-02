//! Window widget

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};
use core::ffi::CStr;

/// Window widget
///
/// A container with a title bar and optional buttons.
///
/// # Example
///
/// ```ignore
/// let win = Window::new(&screen, 40).unwrap();
/// win.add_title(c"My Window");
///
/// // Add buttons to title bar
/// let close_btn = win.add_button(c"\xef\x80\x8d", 40).unwrap(); // LV_SYMBOL_CLOSE
///
/// // Get content area and add widgets
/// let content = win.content().unwrap();
/// let label = Label::new(&content).unwrap();
/// label.set_text(c"Window content");
/// ```
#[derive(Clone, Copy)]
pub struct Window<'a> {
    obj: Obj<'a>,
}

impl<'a> Window<'a> {
    /// Create a new window as a child of the given parent.
    ///
    /// # Arguments
    ///
    /// * `parent` - Parent widget
    /// * `header_height` - Height of the title bar in pixels
    pub fn new(parent: &'a impl Widget<'a>, header_height: i32) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_win_create(parent.raw());
            if ptr.is_null() {
                return None;
            }
            // Get the header and set its height
            let header = neo_lvgl_sys::lv_win_get_header(ptr);
            if !header.is_null() {
                neo_lvgl_sys::lv_obj_set_height(header, header_height);
            }
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Add a title to the window
    pub fn add_title(&self, title: &CStr) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_win_add_title(self.obj.raw(), title.as_ptr());
            Obj::from_raw(ptr)
        }
    }

    /// Add a button to the title bar
    ///
    /// # Arguments
    ///
    /// * `icon` - Icon symbol (e.g., LV_SYMBOL_CLOSE)
    /// * `width` - Button width in pixels
    pub fn add_button(&self, icon: &CStr, width: i32) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_win_add_button(self.obj.raw(), icon.as_ptr() as *const _, width);
            Obj::from_raw(ptr)
        }
    }

    /// Get the header (title bar) object
    pub fn header(&self) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_win_get_header(self.obj.raw());
            Obj::from_raw(ptr)
        }
    }

    /// Get the content area object
    pub fn content(&self) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_win_get_content(self.obj.raw());
            Obj::from_raw(ptr)
        }
    }
}

impl<'a> Widget<'a> for Window<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Window<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



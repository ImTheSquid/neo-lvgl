//! List widget

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};
use core::ffi::CStr;

/// List button item
#[derive(Clone, Copy)]
pub struct ListButton<'a> {
    obj: Obj<'a>,
}

impl<'a> ListButton<'a> {
    /// Create from raw object
    unsafe fn from_obj(obj: Obj<'a>) -> Self {
        Self { obj }
    }
}

impl<'a> Widget<'a> for ListButton<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for ListButton<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



/// List text item
#[derive(Clone, Copy)]
pub struct ListText<'a> {
    obj: Obj<'a>,
}

impl<'a> ListText<'a> {
    /// Create from raw object
    unsafe fn from_obj(obj: Obj<'a>) -> Self {
        Self { obj }
    }
}

impl<'a> Widget<'a> for ListText<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for ListText<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



/// List widget
///
/// A scrollable list of buttons and text items.
///
/// # Example
///
/// ```ignore
/// let list = List::new(&screen).unwrap();
/// list.add_text(c"Section 1");
/// let btn1 = list.add_button(c"Item 1").unwrap();
/// let btn2 = list.add_button(c"Item 2").unwrap();
/// list.add_text(c"Section 2");
/// let btn3 = list.add_button(c"Item 3").unwrap();
/// ```
#[derive(Clone, Copy)]
pub struct List<'a> {
    obj: Obj<'a>,
}

impl<'a> List<'a> {
    /// Create a new list as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_list_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Add a text item (non-clickable header/separator)
    pub fn add_text(&self, text: &CStr) -> Option<ListText<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_list_add_text(self.obj.raw(), text.as_ptr());
            Obj::from_raw(ptr).map(|obj| ListText::from_obj(obj))
        }
    }

    /// Add a button item
    pub fn add_button(&self, text: &CStr) -> Option<ListButton<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_list_add_button(
                self.obj.raw(),
                core::ptr::null(), // No icon
                text.as_ptr(),
            );
            Obj::from_raw(ptr).map(|obj| ListButton::from_obj(obj))
        }
    }

    /// Add a button item with an icon symbol
    pub fn add_button_with_icon(&self, icon: &CStr, text: &CStr) -> Option<ListButton<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_list_add_button(self.obj.raw(), icon.as_ptr() as *const _, text.as_ptr());
            Obj::from_raw(ptr).map(|obj| ListButton::from_obj(obj))
        }
    }

    /// Get the text of a list button
    pub fn button_text<'b>(button: &'b ListButton) -> Option<&'b CStr> {
        let ptr = unsafe { neo_lvgl_sys::lv_list_get_button_text(core::ptr::null_mut(), button.obj.raw()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }
}

impl<'a> Widget<'a> for List<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for List<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



//! Dropdown widget

use super::{Obj, Widget};
use crate::event::EventHandler;
use crate::observer::Subject;
use core::ffi::CStr;

/// Position for adding options
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptionPos {
    /// Add at a specific index
    Index(u32),
    /// Add at the end
    Last,
}

impl OptionPos {
    fn to_raw(self) -> u32 {
        match self {
            OptionPos::Index(i) => i,
            OptionPos::Last => neo_lvgl_sys::LV_DROPDOWN_POS_LAST as u32,
        }
    }
}

/// Dropdown direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropdownDir {
    /// Open downward (default)
    Bottom,
    /// Open upward
    Top,
    /// Open to the left
    Left,
    /// Open to the right
    Right,
}

impl DropdownDir {
    fn to_raw(self) -> neo_lvgl_sys::lv_dir_t {
        match self {
            DropdownDir::Bottom => neo_lvgl_sys::lv_dir_t_LV_DIR_BOTTOM,
            DropdownDir::Top => neo_lvgl_sys::lv_dir_t_LV_DIR_TOP,
            DropdownDir::Left => neo_lvgl_sys::lv_dir_t_LV_DIR_LEFT,
            DropdownDir::Right => neo_lvgl_sys::lv_dir_t_LV_DIR_RIGHT,
        }
    }

    fn from_raw(raw: neo_lvgl_sys::lv_dir_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_dir_t_LV_DIR_TOP => DropdownDir::Top,
            neo_lvgl_sys::lv_dir_t_LV_DIR_LEFT => DropdownDir::Left,
            neo_lvgl_sys::lv_dir_t_LV_DIR_RIGHT => DropdownDir::Right,
            _ => DropdownDir::Bottom,
        }
    }
}

/// Dropdown widget
///
/// A dropdown selector with a list of options.
///
/// # Example
///
/// ```ignore
/// let dd = Dropdown::new(&screen).unwrap();
/// dd.set_options(c"Option 1\nOption 2\nOption 3");
/// dd.set_selected(0);
///
/// dd.on_value_changed(|_| {
///     println!("Selection changed!");
/// });
/// ```
#[derive(Clone, Copy)]
pub struct Dropdown<'a> {
    obj: Obj<'a>,
}

impl<'a> Dropdown<'a> {
    /// Create a new dropdown as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_dropdown_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the options (newline-separated string)
    pub fn set_options(&self, options: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_dropdown_set_options(self.obj.raw(), options.as_ptr().cast());
        }
    }

    /// Set the options (static, no copy)
    pub fn set_options_static(&self, options: &'static CStr) {
        unsafe {
            neo_lvgl_sys::lv_dropdown_set_options_static(self.obj.raw(), options.as_ptr().cast());
        }
    }

    /// Add an option at a specific position
    pub fn add_option(&self, option: &CStr, pos: OptionPos) {
        unsafe {
            neo_lvgl_sys::lv_dropdown_add_option(self.obj.raw(), option.as_ptr().cast(), pos.to_raw());
        }
    }

    /// Clear all options
    pub fn clear_options(&self) {
        unsafe {
            neo_lvgl_sys::lv_dropdown_clear_options(self.obj.raw());
        }
    }

    /// Set the selected option index
    pub fn set_selected(&self, index: u32) {
        unsafe {
            neo_lvgl_sys::lv_dropdown_set_selected(self.obj.raw(), index);
        }
    }

    /// Get the selected option index
    pub fn selected(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_dropdown_get_selected(self.obj.raw()) }
    }

    /// Get the number of options
    pub fn option_count(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_dropdown_get_option_count(self.obj.raw()) }
    }

    /// Get the selected option text into a buffer
    pub fn selected_str(&self, buf: &mut [u8]) {
        unsafe {
            neo_lvgl_sys::lv_dropdown_get_selected_str(
                self.obj.raw(),
                buf.as_mut_ptr() as *mut _,
                buf.len() as u32,
            );
        }
    }

    /// Get all options as a string
    pub fn options(&self) -> &CStr {
        unsafe {
            let ptr = neo_lvgl_sys::lv_dropdown_get_options(self.obj.raw());
            CStr::from_ptr(ptr.cast())
        }
    }

    /// Set the dropdown direction
    pub fn set_dir(&self, dir: DropdownDir) {
        unsafe {
            neo_lvgl_sys::lv_dropdown_set_dir(self.obj.raw(), dir.to_raw());
        }
    }

    /// Get the dropdown direction
    pub fn dir(&self) -> DropdownDir {
        let raw = unsafe { neo_lvgl_sys::lv_dropdown_get_dir(self.obj.raw()) };
        DropdownDir::from_raw(raw)
    }

    /// Set text displayed on the dropdown button
    pub fn set_text(&self, text: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_dropdown_set_text(self.obj.raw(), text.as_ptr().cast());
        }
    }

    /// Get the text displayed on the dropdown button
    pub fn text(&self) -> Option<&CStr> {
        let ptr = unsafe { neo_lvgl_sys::lv_dropdown_get_text(self.obj.raw()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr.cast()) })
        }
    }

    /// Enable/disable highlight of selected item
    pub fn set_selected_highlight(&self, en: bool) {
        unsafe {
            neo_lvgl_sys::lv_dropdown_set_selected_highlight(self.obj.raw(), en);
        }
    }

    /// Check if selected highlight is enabled
    pub fn selected_highlight(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_dropdown_get_selected_highlight(self.obj.raw()) }
    }

    /// Open the dropdown list
    pub fn open(&self) {
        unsafe {
            neo_lvgl_sys::lv_dropdown_open(self.obj.raw());
        }
    }

    /// Close the dropdown list
    pub fn close(&self) {
        unsafe {
            neo_lvgl_sys::lv_dropdown_close(self.obj.raw());
        }
    }

    /// Check if the dropdown is open
    pub fn is_open(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_dropdown_is_open(self.obj.raw()) }
    }

    /// Get the dropdown list object (when open)
    pub fn list(&self) -> Option<Obj<'a>> {
        let ptr = unsafe { neo_lvgl_sys::lv_dropdown_get_list(self.obj.raw()) };
        unsafe { Obj::from_raw(ptr) }
    }
}

impl<'a> Widget<'a> for Dropdown<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Dropdown<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}


impl<'a> Dropdown<'a> {
    /// Bind this dropdown's selected index to an integer subject
    ///
    /// When the subject changes, the dropdown selection updates automatically.
    /// When the user selects an option, the subject is updated.
    pub fn bind_value(
        &self,
        subject: &mut crate::observer::IntSubject,
    ) -> Option<crate::observer::Observer> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_dropdown_bind_value(self.obj.raw(), subject.raw());
            crate::observer::Observer::from_raw(ptr)
        }
    }
}


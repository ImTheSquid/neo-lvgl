//! TabView widget

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};
use core::ffi::CStr;

/// Tab bar position
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabViewPos {
    /// Tabs at the top
    Top,
    /// Tabs at the bottom
    Bottom,
    /// Tabs on the left
    Left,
    /// Tabs on the right
    Right,
}

impl TabViewPos {
    fn to_raw(self) -> neo_lvgl_sys::lv_dir_t {
        match self {
            TabViewPos::Top => neo_lvgl_sys::lv_dir_t_LV_DIR_TOP,
            TabViewPos::Bottom => neo_lvgl_sys::lv_dir_t_LV_DIR_BOTTOM,
            TabViewPos::Left => neo_lvgl_sys::lv_dir_t_LV_DIR_LEFT,
            TabViewPos::Right => neo_lvgl_sys::lv_dir_t_LV_DIR_RIGHT,
        }
    }
}

/// TabView widget
///
/// A container with multiple tabs that can be switched between.
///
/// # Example
///
/// ```ignore
/// let tabview = TabView::new(&screen, TabViewPos::Top, 50).unwrap();
///
/// let tab1 = tabview.add_tab(c"Tab 1").unwrap();
/// let tab2 = tabview.add_tab(c"Tab 2").unwrap();
/// let tab3 = tabview.add_tab(c"Tab 3").unwrap();
///
/// // Add content to each tab
/// let label1 = Label::new(&tab1).unwrap();
/// label1.set_text(c"Content of Tab 1");
/// ```
#[derive(Clone, Copy)]
pub struct TabView<'a> {
    obj: Obj<'a>,
}

impl<'a> TabView<'a> {
    /// Create a new tab view as a child of the given parent.
    ///
    /// # Arguments
    ///
    /// * `parent` - Parent widget
    /// * `pos` - Position of the tab bar
    /// * `tab_size` - Height (for top/bottom) or width (for left/right) of the tab bar
    pub fn new(parent: &'a impl Widget<'a>, pos: TabViewPos, tab_size: i32) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_tabview_create(parent.raw());
            if ptr.is_null() {
                return None;
            }
            neo_lvgl_sys::lv_tabview_set_tab_bar_position(ptr, pos.to_raw());
            neo_lvgl_sys::lv_tabview_set_tab_bar_size(ptr, tab_size);
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Add a new tab
    pub fn add_tab(&self, name: &CStr) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_tabview_add_tab(self.obj.raw(), name.as_ptr());
            Obj::from_raw(ptr)
        }
    }

    /// Set the active tab
    pub fn set_active(&self, index: u32, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_tabview_set_active(self.obj.raw(), index, anim);
        }
    }

    /// Get the currently active tab index
    pub fn active(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_tabview_get_tab_active(self.obj.raw()) }
    }

    /// Get the number of tabs
    pub fn tab_count(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_tabview_get_tab_count(self.obj.raw()) }
    }

    /// Get the content object for a tab by index
    pub fn tab_content(&self, index: u32) -> Option<Obj<'a>> {
        unsafe {
            let content = neo_lvgl_sys::lv_tabview_get_content(self.obj.raw());
            if content.is_null() {
                return None;
            }
            let tab = neo_lvgl_sys::lv_obj_get_child(content, index as i32);
            Obj::from_raw(tab)
        }
    }

    /// Get the tab bar object
    pub fn tab_bar(&self) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_tabview_get_tab_bar(self.obj.raw());
            Obj::from_raw(ptr)
        }
    }

    /// Set the tab bar position
    pub fn set_tab_bar_position(&self, pos: TabViewPos) {
        unsafe {
            neo_lvgl_sys::lv_tabview_set_tab_bar_position(self.obj.raw(), pos.to_raw());
        }
    }

    /// Set the tab bar size
    pub fn set_tab_bar_size(&self, size: i32) {
        unsafe {
            neo_lvgl_sys::lv_tabview_set_tab_bar_size(self.obj.raw(), size);
        }
    }
}

impl<'a> Widget<'a> for TabView<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for TabView<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



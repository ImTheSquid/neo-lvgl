//! Menu widget

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};
use core::ffi::CStr;

/// A menu page
#[derive(Clone, Copy)]
pub struct MenuPage<'a> {
    obj: Obj<'a>,
}

impl<'a> MenuPage<'a> {
    /// Create from raw object
    unsafe fn from_obj(obj: Obj<'a>) -> Self {
        Self { obj }
    }
}

impl<'a> Widget<'a> for MenuPage<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for MenuPage<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



/// A menu section container
#[derive(Clone, Copy)]
pub struct MenuSection<'a> {
    obj: Obj<'a>,
}

impl<'a> MenuSection<'a> {
    /// Create from raw object
    unsafe fn from_obj(obj: Obj<'a>) -> Self {
        Self { obj }
    }
}

impl<'a> Widget<'a> for MenuSection<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for MenuSection<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



/// A menu separator
#[derive(Clone, Copy)]
pub struct MenuSeparator<'a> {
    obj: Obj<'a>,
}

impl<'a> MenuSeparator<'a> {
    /// Create from raw object
    unsafe fn from_obj(obj: Obj<'a>) -> Self {
        Self { obj }
    }
}

impl<'a> Widget<'a> for MenuSeparator<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for MenuSeparator<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



/// Menu widget
///
/// A hierarchical menu with pages and submenus.
///
/// # Example
///
/// ```ignore
/// let menu = Menu::new(&screen).unwrap();
///
/// let main_page = menu.create_page(c"Main").unwrap();
/// let settings_page = menu.create_page(c"Settings").unwrap();
///
/// // Add items to main page
/// let section = menu.add_section(&main_page).unwrap();
/// let cont = menu.add_item(&section, c"Settings", true).unwrap();
/// menu.set_page_link(&cont, &settings_page);
///
/// menu.set_page(&main_page);
/// ```
#[derive(Clone, Copy)]
pub struct Menu<'a> {
    obj: Obj<'a>,
}

impl<'a> Menu<'a> {
    /// Create a new menu as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_menu_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Create a new page
    pub fn create_page(&self, title: &CStr) -> Option<MenuPage<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_menu_page_create(self.obj.raw(), title.as_ptr());
            Obj::from_raw(ptr).map(|obj| MenuPage::from_obj(obj))
        }
    }

    /// Add a section container to a page
    pub fn add_section(&self, page: &MenuPage) -> Option<MenuSection<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_menu_section_create(page.obj.raw());
            Obj::from_raw(ptr).map(|obj| MenuSection::from_obj(obj))
        }
    }

    /// Add a separator
    pub fn add_separator(&self, parent: &impl Widget<'a>) -> Option<MenuSeparator<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_menu_separator_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| MenuSeparator::from_obj(obj))
        }
    }

    /// Add a menu item container
    ///
    /// # Arguments
    ///
    /// * `parent` - Parent section or page
    /// * `icon` - Icon symbol (pass empty string for no icon)
    /// * `text` - Item text
    pub fn add_item(&self, parent: &impl Widget<'a>, icon: &CStr, text: &CStr) -> Option<Obj<'a>> {
        unsafe {
            let cont = neo_lvgl_sys::lv_menu_cont_create(parent.raw());
            if cont.is_null() {
                return None;
            }
            // Add icon if provided
            if !icon.to_bytes().is_empty() {
                let img = neo_lvgl_sys::lv_image_create(cont);
                if !img.is_null() {
                    neo_lvgl_sys::lv_image_set_src(img, icon.as_ptr() as *const _);
                }
            }
            // Add label
            let label = neo_lvgl_sys::lv_label_create(cont);
            if !label.is_null() {
                neo_lvgl_sys::lv_label_set_text(label, text.as_ptr());
                neo_lvgl_sys::lv_obj_set_flex_grow(label, 1);
            }
            Obj::from_raw(cont)
        }
    }

    /// Set a page link for a menu item
    pub fn set_page_link(&self, item: &Obj, page: &MenuPage) {
        unsafe {
            neo_lvgl_sys::lv_menu_set_load_page_event(self.obj.raw(), item.raw(), page.obj.raw());
        }
    }

    /// Set the current page
    pub fn set_page(&self, page: &MenuPage) {
        unsafe {
            neo_lvgl_sys::lv_menu_set_page(self.obj.raw(), page.obj.raw());
        }
    }

    /// Clear the page history and go to root
    pub fn clear_history(&self) {
        unsafe {
            neo_lvgl_sys::lv_menu_clear_history(self.obj.raw());
        }
    }

    /// Get the current page
    pub fn current_page(&self) -> Option<MenuPage<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_menu_get_cur_main_page(self.obj.raw());
            Obj::from_raw(ptr).map(|obj| MenuPage::from_obj(obj))
        }
    }

    /// Set the sidebar page (always visible)
    pub fn set_sidebar_page(&self, page: &MenuPage) {
        unsafe {
            neo_lvgl_sys::lv_menu_set_sidebar_page(self.obj.raw(), page.obj.raw());
        }
    }

    /// Enable/disable the mode header button
    pub fn set_mode_header(&self, mode: bool) {
        let mode_val = if mode {
            neo_lvgl_sys::lv_menu_mode_header_t_LV_MENU_HEADER_TOP_FIXED
        } else {
            neo_lvgl_sys::lv_menu_mode_header_t_LV_MENU_HEADER_TOP_UNFIXED
        };
        unsafe {
            neo_lvgl_sys::lv_menu_set_mode_header(self.obj.raw(), mode_val);
        }
    }
}

impl<'a> Widget<'a> for Menu<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Menu<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



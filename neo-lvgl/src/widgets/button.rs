//! Button widget

use super::{Obj, Widget};
use crate::event::EventHandler;

/// Button widget
///
/// A clickable button that can contain other widgets (like labels).
///
/// # Example
///
/// ```ignore
/// let btn = Button::new(&screen).unwrap();
/// btn.set_size(100, 50);
/// btn.center();
///
/// let label = Label::new(&btn).unwrap();
/// label.set_text(c"Click me!");
/// ```
pub struct Button<'a> {
    obj: Obj<'a>,
}

impl<'a> Button<'a> {
    /// Create a new button as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_button_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Create a label inside this button.
    pub fn create_label(&'a self) -> Option<super::Label<'a>> {
        super::Label::new(self)
    }
}

impl<'a> Widget<'a> for Button<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Button<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}

#[cfg(feature = "alloc")]
impl<'a> crate::event::ClosureEventHandler for Button<'a> {}

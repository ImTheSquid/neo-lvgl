//! ImageButton widget

use super::{Obj, Widget};
use crate::event::EventHandler;
use core::ffi::c_void;

/// ImageButton state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageButtonState {
    Released,
    Pressed,
    Disabled,
    CheckedReleased,
    CheckedPressed,
    CheckedDisabled,
}

impl ImageButtonState {
    fn to_raw(self) -> neo_lvgl_sys::lv_imagebutton_state_t {
        match self {
            ImageButtonState::Released => neo_lvgl_sys::lv_imagebutton_state_t_LV_IMAGEBUTTON_STATE_RELEASED,
            ImageButtonState::Pressed => neo_lvgl_sys::lv_imagebutton_state_t_LV_IMAGEBUTTON_STATE_PRESSED,
            ImageButtonState::Disabled => neo_lvgl_sys::lv_imagebutton_state_t_LV_IMAGEBUTTON_STATE_DISABLED,
            ImageButtonState::CheckedReleased => neo_lvgl_sys::lv_imagebutton_state_t_LV_IMAGEBUTTON_STATE_CHECKED_RELEASED,
            ImageButtonState::CheckedPressed => neo_lvgl_sys::lv_imagebutton_state_t_LV_IMAGEBUTTON_STATE_CHECKED_PRESSED,
            ImageButtonState::CheckedDisabled => neo_lvgl_sys::lv_imagebutton_state_t_LV_IMAGEBUTTON_STATE_CHECKED_DISABLED,
        }
    }
}

/// ImageButton widget
///
/// A button composed of images for different states.
/// Can use three-part images (left, middle, right) for scalable buttons.
///
/// # Example
///
/// ```ignore
/// let imgbtn = ImageButton::new(&screen).unwrap();
/// imgbtn.set_src(ImageButtonState::Released, Some(&img_left), Some(&img_mid), Some(&img_right));
/// ```
#[derive(Clone, Copy)]
pub struct ImageButton<'a> {
    obj: Obj<'a>,
}

impl<'a> ImageButton<'a> {
    /// Create a new image button as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_imagebutton_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the image sources for a state
    ///
    /// Uses three-part images for scalable buttons (left edge, repeating middle, right edge).
    /// Pass None for any part you don't want to use.
    ///
    /// # Safety
    ///
    /// The image pointers must remain valid for the lifetime of the image button.
    pub unsafe fn set_src(
        &self,
        state: ImageButtonState,
        src_left: Option<*const c_void>,
        src_mid: Option<*const c_void>,
        src_right: Option<*const c_void>,
    ) {
        neo_lvgl_sys::lv_imagebutton_set_src(
            self.obj.raw(),
            state.to_raw(),
            src_left.unwrap_or(core::ptr::null()),
            src_mid.unwrap_or(core::ptr::null()),
            src_right.unwrap_or(core::ptr::null()),
        );
    }

    /// Set the state of the image button
    pub fn set_state(&self, state: ImageButtonState) {
        unsafe {
            neo_lvgl_sys::lv_imagebutton_set_state(self.obj.raw(), state.to_raw());
        }
    }
}

impl<'a> Widget<'a> for ImageButton<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for ImageButton<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



//! LED widget

use crate::color::Color;
use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};

/// LED widget
///
/// A visual indicator that can be turned on/off or set to a brightness level.
///
/// # Example
///
/// ```ignore
/// let led = Led::new(&screen).unwrap();
/// led.set_color(Color::green());
/// led.on();
///
/// // Or set brightness (0-255)
/// led.set_brightness(128);
/// ```
#[derive(Clone, Copy)]
pub struct Led<'a> {
    obj: Obj<'a>,
}

impl<'a> Led<'a> {
    /// Create a new LED as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_led_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the LED color
    pub fn set_color(&self, color: Color) {
        unsafe {
            neo_lvgl_sys::lv_led_set_color(self.obj.raw(), color.to_raw());
        }
    }

    /// Set the LED brightness
    ///
    /// # Arguments
    ///
    /// * `bright` - Brightness level from 0 (off) to 255 (full brightness)
    pub fn set_brightness(&self, bright: u8) {
        unsafe {
            neo_lvgl_sys::lv_led_set_brightness(self.obj.raw(), bright);
        }
    }

    /// Get the current brightness
    pub fn brightness(&self) -> u8 {
        unsafe { neo_lvgl_sys::lv_led_get_brightness(self.obj.raw()) }
    }

    /// Turn the LED on (full brightness)
    pub fn on(&self) {
        unsafe {
            neo_lvgl_sys::lv_led_on(self.obj.raw());
        }
    }

    /// Turn the LED off (zero brightness)
    pub fn off(&self) {
        unsafe {
            neo_lvgl_sys::lv_led_off(self.obj.raw());
        }
    }

    /// Toggle the LED state
    pub fn toggle(&self) {
        unsafe {
            neo_lvgl_sys::lv_led_toggle(self.obj.raw());
        }
    }
}

impl<'a> Widget<'a> for Led<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Led<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



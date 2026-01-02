//! Input device system for LVGL
//!
//! This module provides support for various input devices:
//!
//! - Pointer devices (touch, mouse)
//! - Keypad devices (keyboard)
//! - Encoder devices (rotary encoder)
//! - Button devices (hardware buttons)
//!
//! # Example
//!
//! ```ignore
//! use lvgl::indev::{Indev, PointerData, IndevState};
//! use lvgl::widgets::Point;
//!
//! let indev = Indev::new_pointer(|| {
//!     // Read touch coordinates
//!     PointerData {
//!         point: Point::new(touch_x, touch_y),
//!         state: if touched { IndevState::Pressed } else { IndevState::Released },
//!     }
//! });
//! ```

use crate::widgets::Point;
use core::ptr::NonNull;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Input device type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndevType {
    /// Pointer (touch/mouse)
    Pointer,
    /// Keypad (keyboard)
    Keypad,
    /// Hardware buttons at fixed screen positions
    Button,
    /// Rotary encoder
    Encoder,
}

impl IndevType {
    fn to_raw(self) -> neo_lvgl_sys::lv_indev_type_t {
        match self {
            IndevType::Pointer => neo_lvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_POINTER,
            IndevType::Keypad => neo_lvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_KEYPAD,
            IndevType::Button => neo_lvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_BUTTON,
            IndevType::Encoder => neo_lvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_ENCODER,
        }
    }

    fn from_raw(raw: neo_lvgl_sys::lv_indev_type_t) -> Option<Self> {
        match raw {
            neo_lvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_POINTER => Some(IndevType::Pointer),
            neo_lvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_KEYPAD => Some(IndevType::Keypad),
            neo_lvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_BUTTON => Some(IndevType::Button),
            neo_lvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_ENCODER => Some(IndevType::Encoder),
            _ => None,
        }
    }
}

/// Input device state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndevState {
    Released,
    Pressed,
}

impl IndevState {
    #[allow(dead_code)]
    pub(crate) fn to_raw(self) -> neo_lvgl_sys::lv_indev_state_t {
        match self {
            IndevState::Released => neo_lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_RELEASED,
            IndevState::Pressed => neo_lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_PRESSED,
        }
    }
}

// LVGL key codes (from lv_indev.h)
const LV_KEY_UP: u32 = 17;
const LV_KEY_DOWN: u32 = 18;
const LV_KEY_RIGHT: u32 = 19;
const LV_KEY_LEFT: u32 = 20;
const LV_KEY_ESC: u32 = 27;
const LV_KEY_DEL: u32 = 127;
const LV_KEY_BACKSPACE: u32 = 8;
const LV_KEY_ENTER: u32 = 10;
const LV_KEY_NEXT: u32 = 9;
const LV_KEY_PREV: u32 = 11;
const LV_KEY_HOME: u32 = 2;
const LV_KEY_END: u32 = 3;

/// Key codes for keypad input
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Backspace,
    Esc,
    Next,
    Prev,
    Home,
    End,
    Del,
    /// Custom key code
    Custom(u32),
}

impl Key {
    pub(crate) fn to_raw(self) -> u32 {
        match self {
            Key::Up => LV_KEY_UP,
            Key::Down => LV_KEY_DOWN,
            Key::Left => LV_KEY_LEFT,
            Key::Right => LV_KEY_RIGHT,
            Key::Enter => LV_KEY_ENTER,
            Key::Backspace => LV_KEY_BACKSPACE,
            Key::Esc => LV_KEY_ESC,
            Key::Next => LV_KEY_NEXT,
            Key::Prev => LV_KEY_PREV,
            Key::Home => LV_KEY_HOME,
            Key::End => LV_KEY_END,
            Key::Del => LV_KEY_DEL,
            Key::Custom(k) => k,
        }
    }

    fn from_raw(raw: u32) -> Self {
        match raw {
            LV_KEY_UP => Key::Up,
            LV_KEY_DOWN => Key::Down,
            LV_KEY_LEFT => Key::Left,
            LV_KEY_RIGHT => Key::Right,
            LV_KEY_ENTER => Key::Enter,
            LV_KEY_BACKSPACE => Key::Backspace,
            LV_KEY_ESC => Key::Esc,
            LV_KEY_NEXT => Key::Next,
            LV_KEY_PREV => Key::Prev,
            LV_KEY_HOME => Key::Home,
            LV_KEY_END => Key::End,
            LV_KEY_DEL => Key::Del,
            k => Key::Custom(k),
        }
    }
}

/// Pointer input data
#[derive(Clone, Copy, Debug)]
pub struct PointerData {
    pub point: Point,
    pub state: IndevState,
}

/// Keypad input data
#[derive(Clone, Copy, Debug)]
pub struct KeypadData {
    pub key: Key,
    pub state: IndevState,
}

/// Encoder input data
#[derive(Clone, Copy, Debug)]
pub struct EncoderData {
    /// Rotation steps (positive = clockwise, negative = counter-clockwise)
    pub diff: i16,
    /// Button state
    pub state: IndevState,
}

/// Input device read callback type
pub type IndevReadCb = unsafe extern "C" fn(*mut neo_lvgl_sys::lv_indev_t, *mut neo_lvgl_sys::lv_indev_data_t);

/// Input device
pub struct Indev {
    raw: NonNull<neo_lvgl_sys::lv_indev_t>,
}

impl Indev {
    /// Create a new input device with a static read callback
    pub fn new_static(indev_type: IndevType, read_cb: IndevReadCb) -> Option<Self> {
        let ptr = unsafe { neo_lvgl_sys::lv_indev_create() };
        let indev = NonNull::new(ptr)?;

        unsafe {
            neo_lvgl_sys::lv_indev_set_type(ptr, indev_type.to_raw());
            neo_lvgl_sys::lv_indev_set_read_cb(ptr, Some(read_cb));
        }

        Some(Self { raw: indev })
    }

    /// Create from raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be valid.
    pub unsafe fn from_raw(raw: *mut neo_lvgl_sys::lv_indev_t) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Get the input device type
    pub fn indev_type(&self) -> Option<IndevType> {
        let raw = unsafe { neo_lvgl_sys::lv_indev_get_type(self.raw.as_ptr()) };
        IndevType::from_raw(raw)
    }

    /// Enable or disable the input device
    pub fn enable(&self, en: bool) {
        unsafe {
            neo_lvgl_sys::lv_indev_enable(self.raw.as_ptr(), en);
        }
    }

    /// Set the display for this input device
    pub fn set_display(&self, display: &crate::display::Display) {
        unsafe {
            neo_lvgl_sys::lv_indev_set_display(self.raw.as_ptr(), display.raw());
        }
    }

    /// Set the group for keyboard/encoder navigation
    pub fn set_group(&self, group: &crate::group::Group) {
        unsafe {
            neo_lvgl_sys::lv_indev_set_group(self.raw.as_ptr(), group.raw());
        }
    }

    /// Set cursor object for pointer devices
    pub fn set_cursor<'a, W: crate::widgets::Widget<'a>>(&self, cursor: &W) {
        unsafe {
            neo_lvgl_sys::lv_indev_set_cursor(self.raw.as_ptr(), cursor.raw());
        }
    }

    /// Get the last pressed point (for pointer devices)
    pub fn point(&self) -> Point {
        let mut point = neo_lvgl_sys::lv_point_t { x: 0, y: 0 };
        unsafe {
            neo_lvgl_sys::lv_indev_get_point(self.raw.as_ptr(), &mut point);
        }
        Point::new(point.x, point.y)
    }

    /// Get the last pressed key (for keypad devices)
    pub fn key(&self) -> Key {
        let raw = unsafe { neo_lvgl_sys::lv_indev_get_key(self.raw.as_ptr()) };
        Key::from_raw(raw)
    }

    /// Get the scroll direction
    pub fn scroll_dir(&self) -> ScrollDir {
        let raw = unsafe { neo_lvgl_sys::lv_indev_get_scroll_dir(self.raw.as_ptr()) };
        ScrollDir::from_raw(raw)
    }

    /// Get the gesture direction
    pub fn gesture_dir(&self) -> GestureDir {
        let raw = unsafe { neo_lvgl_sys::lv_indev_get_gesture_dir(self.raw.as_ptr()) };
        GestureDir::from_raw(raw)
    }

    /// Get the movement vector (for pointer devices)
    pub fn vect(&self) -> Point {
        let mut point = neo_lvgl_sys::lv_point_t { x: 0, y: 0 };
        unsafe {
            neo_lvgl_sys::lv_indev_get_vect(self.raw.as_ptr(), &mut point);
        }
        Point::new(point.x, point.y)
    }

    /// Set long press time
    pub fn set_long_press_time(&self, time_ms: u16) {
        unsafe {
            neo_lvgl_sys::lv_indev_set_long_press_time(self.raw.as_ptr(), time_ms);
        }
    }

    /// Set scroll limit (minimum drag distance to start scrolling)
    pub fn set_scroll_limit(&self, limit: u8) {
        unsafe {
            neo_lvgl_sys::lv_indev_set_scroll_limit(self.raw.as_ptr(), limit);
        }
    }

    /// Set scroll throw (momentum after release)
    pub fn set_scroll_throw(&self, throw: u8) {
        unsafe {
            neo_lvgl_sys::lv_indev_set_scroll_throw(self.raw.as_ptr(), throw);
        }
    }

    /// Delete the input device
    pub fn delete(self) {
        unsafe {
            neo_lvgl_sys::lv_indev_delete(self.raw.as_ptr());
        }
    }

    /// Get the raw pointer
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_indev_t {
        self.raw.as_ptr()
    }
}

/// Scroll direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollDir {
    None,
    Left,
    Right,
    Top,
    Bottom,
}

impl ScrollDir {
    fn from_raw(raw: neo_lvgl_sys::lv_dir_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_dir_t_LV_DIR_LEFT => ScrollDir::Left,
            neo_lvgl_sys::lv_dir_t_LV_DIR_RIGHT => ScrollDir::Right,
            neo_lvgl_sys::lv_dir_t_LV_DIR_TOP => ScrollDir::Top,
            neo_lvgl_sys::lv_dir_t_LV_DIR_BOTTOM => ScrollDir::Bottom,
            _ => ScrollDir::None,
        }
    }
}

/// Gesture direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GestureDir {
    None,
    Left,
    Right,
    Top,
    Bottom,
}

impl GestureDir {
    fn from_raw(raw: neo_lvgl_sys::lv_dir_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_dir_t_LV_DIR_LEFT => GestureDir::Left,
            neo_lvgl_sys::lv_dir_t_LV_DIR_RIGHT => GestureDir::Right,
            neo_lvgl_sys::lv_dir_t_LV_DIR_TOP => GestureDir::Top,
            neo_lvgl_sys::lv_dir_t_LV_DIR_BOTTOM => GestureDir::Bottom,
            _ => GestureDir::None,
        }
    }
}

// Closure support (requires alloc feature)
#[cfg(feature = "alloc")]
mod closure_support {
    use super::*;
    use core::ffi::c_void;

    /// Container for pointer read closure
    struct PointerReadClosure {
        callback: Box<dyn FnMut() -> PointerData>,
    }

    /// Container for keypad read closure
    struct KeypadReadClosure {
        callback: Box<dyn FnMut() -> KeypadData>,
    }

    /// Container for encoder read closure
    struct EncoderReadClosure {
        callback: Box<dyn FnMut() -> EncoderData>,
    }

    /// Trampoline for pointer read callback
    unsafe extern "C" fn pointer_trampoline(
        indev: *mut neo_lvgl_sys::lv_indev_t,
        data: *mut neo_lvgl_sys::lv_indev_data_t,
    ) {
        let user_data = neo_lvgl_sys::lv_indev_get_driver_data(indev);
        if !user_data.is_null() {
            let closure = &mut *(user_data as *mut PointerReadClosure);
            let result = (closure.callback)();
            (*data).point.x = result.point.x;
            (*data).point.y = result.point.y;
            (*data).state = result.state.to_raw();
        }
    }

    /// Trampoline for keypad read callback
    unsafe extern "C" fn keypad_trampoline(
        indev: *mut neo_lvgl_sys::lv_indev_t,
        data: *mut neo_lvgl_sys::lv_indev_data_t,
    ) {
        let user_data = neo_lvgl_sys::lv_indev_get_driver_data(indev);
        if !user_data.is_null() {
            let closure = &mut *(user_data as *mut KeypadReadClosure);
            let result = (closure.callback)();
            (*data).key = result.key.to_raw();
            (*data).state = result.state.to_raw();
        }
    }

    /// Trampoline for encoder read callback
    unsafe extern "C" fn encoder_trampoline(
        indev: *mut neo_lvgl_sys::lv_indev_t,
        data: *mut neo_lvgl_sys::lv_indev_data_t,
    ) {
        let user_data = neo_lvgl_sys::lv_indev_get_driver_data(indev);
        if !user_data.is_null() {
            let closure = &mut *(user_data as *mut EncoderReadClosure);
            let result = (closure.callback)();
            (*data).enc_diff = result.diff;
            (*data).state = result.state.to_raw();
        }
    }

    impl Indev {
        /// Create a new pointer input device with a closure
        pub fn new_pointer<F>(read_cb: F) -> Option<Self>
        where
            F: FnMut() -> PointerData + 'static,
        {
            let closure = Box::new(PointerReadClosure {
                callback: Box::new(read_cb),
            });
            let raw_closure = Box::into_raw(closure);

            let ptr = unsafe { neo_lvgl_sys::lv_indev_create() };
            let indev = NonNull::new(ptr)?;

            unsafe {
                neo_lvgl_sys::lv_indev_set_type(ptr, IndevType::Pointer.to_raw());
                neo_lvgl_sys::lv_indev_set_read_cb(ptr, Some(pointer_trampoline));
                neo_lvgl_sys::lv_indev_set_driver_data(ptr, raw_closure as *mut c_void);
            }

            Some(Self { raw: indev })
        }

        /// Create a new keypad input device with a closure
        pub fn new_keypad<F>(read_cb: F) -> Option<Self>
        where
            F: FnMut() -> KeypadData + 'static,
        {
            let closure = Box::new(KeypadReadClosure {
                callback: Box::new(read_cb),
            });
            let raw_closure = Box::into_raw(closure);

            let ptr = unsafe { neo_lvgl_sys::lv_indev_create() };
            let indev = NonNull::new(ptr)?;

            unsafe {
                neo_lvgl_sys::lv_indev_set_type(ptr, IndevType::Keypad.to_raw());
                neo_lvgl_sys::lv_indev_set_read_cb(ptr, Some(keypad_trampoline));
                neo_lvgl_sys::lv_indev_set_driver_data(ptr, raw_closure as *mut c_void);
            }

            Some(Self { raw: indev })
        }

        /// Create a new encoder input device with a closure
        pub fn new_encoder<F>(read_cb: F) -> Option<Self>
        where
            F: FnMut() -> EncoderData + 'static,
        {
            let closure = Box::new(EncoderReadClosure {
                callback: Box::new(read_cb),
            });
            let raw_closure = Box::into_raw(closure);

            let ptr = unsafe { neo_lvgl_sys::lv_indev_create() };
            let indev = NonNull::new(ptr)?;

            unsafe {
                neo_lvgl_sys::lv_indev_set_type(ptr, IndevType::Encoder.to_raw());
                neo_lvgl_sys::lv_indev_set_read_cb(ptr, Some(encoder_trampoline));
                neo_lvgl_sys::lv_indev_set_driver_data(ptr, raw_closure as *mut c_void);
            }

            Some(Self { raw: indev })
        }
    }
}

/// Get the currently active input device
pub fn indev_active() -> Option<Indev> {
    let ptr = unsafe { neo_lvgl_sys::lv_indev_active() };
    unsafe { Indev::from_raw(ptr) }
}

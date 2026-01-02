//! Observer/Subject pattern for reactive data binding
//!
//! LVGL 9's observer system provides reactive data binding between subjects
//! (data sources) and widgets. When a subject's value changes, all bound
//! observers are automatically notified.
//!
//! # Example
//!
//! ```ignore
//! use lvgl::observer::{IntSubject, Subject};
//!
//! // Create an integer subject with initial value
//! let mut temperature = IntSubject::new(25);
//!
//! // Bind a slider to the subject - it will automatically update
//! let slider = Slider::new(&screen).unwrap();
//! slider.bind_value(&mut temperature);
//!
//! // Bind a label to show the value
//! let label = Label::new(&screen).unwrap();
//! label.bind_text(&mut temperature, c"Temp: %d C");
//!
//! // When you update the subject, all bound widgets update automatically
//! temperature.set(30);
//! ```

use crate::color::Color;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::NonNull;

/// Observer handle
///
/// Represents a subscription to a subject. When dropped or removed,
/// the observer stops receiving updates.
pub struct Observer {
    raw: NonNull<neo_lvgl_sys::lv_observer_t>,
}

impl Observer {
    /// Create from raw pointer
    pub(crate) unsafe fn from_raw(ptr: *mut neo_lvgl_sys::lv_observer_t) -> Option<Self> {
        NonNull::new(ptr).map(|raw| Self { raw })
    }

    /// Get the raw pointer
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_observer_t {
        self.raw.as_ptr()
    }

    /// Remove this observer (stop receiving updates)
    pub fn remove(self) {
        unsafe {
            neo_lvgl_sys::lv_observer_remove(self.raw.as_ptr());
        }
        // Don't run Drop since we already removed it
        core::mem::forget(self);
    }

    /// Get the user data associated with this observer
    pub fn user_data(&self) -> *mut core::ffi::c_void {
        unsafe { neo_lvgl_sys::lv_observer_get_user_data(self.raw.as_ptr()) }
    }
}

/// Common trait for all subject types
pub trait Subject {
    /// Get the raw subject pointer
    fn raw(&mut self) -> *mut neo_lvgl_sys::lv_subject_t;

    /// Manually notify all observers of a change
    fn notify(&mut self) {
        unsafe {
            neo_lvgl_sys::lv_subject_notify(self.raw());
        }
    }
}

/// Integer subject for reactive integer values
///
/// Use this to create reactive integer data that can be bound to widgets
/// like sliders, bars, arcs, etc.
pub struct IntSubject {
    raw: neo_lvgl_sys::lv_subject_t,
}

impl IntSubject {
    /// Create a new integer subject with an initial value
    pub fn new(initial_value: i32) -> Self {
        let mut raw = MaybeUninit::<neo_lvgl_sys::lv_subject_t>::uninit();
        unsafe {
            neo_lvgl_sys::lv_subject_init_int(raw.as_mut_ptr(), initial_value);
            Self {
                raw: raw.assume_init(),
            }
        }
    }

    /// Set the current value (notifies all observers)
    pub fn set(&mut self, value: i32) {
        unsafe {
            neo_lvgl_sys::lv_subject_set_int(&mut self.raw, value);
        }
    }

    /// Get the current value
    pub fn get(&mut self) -> i32 {
        unsafe { neo_lvgl_sys::lv_subject_get_int(&mut self.raw) }
    }

    /// Get the previous value (before the last change)
    pub fn previous(&mut self) -> i32 {
        unsafe { neo_lvgl_sys::lv_subject_get_previous_int(&mut self.raw) }
    }

    /// Set the minimum allowed value
    pub fn set_min(&mut self, min: i32) {
        unsafe {
            neo_lvgl_sys::lv_subject_set_min_value_int(&mut self.raw, min);
        }
    }

    /// Set the maximum allowed value
    pub fn set_max(&mut self, max: i32) {
        unsafe {
            neo_lvgl_sys::lv_subject_set_max_value_int(&mut self.raw, max);
        }
    }
}

impl Subject for IntSubject {
    fn raw(&mut self) -> *mut neo_lvgl_sys::lv_subject_t {
        &mut self.raw
    }
}

impl Drop for IntSubject {
    fn drop(&mut self) {
        unsafe {
            neo_lvgl_sys::lv_subject_deinit(&mut self.raw);
        }
    }
}

/// String subject for reactive string values
///
/// Use this to create reactive string data that can be bound to labels.
/// The subject owns a buffer for the string data.
pub struct StringSubject {
    raw: neo_lvgl_sys::lv_subject_t,
}

impl StringSubject {
    /// Create a new string subject with a buffer and initial value
    ///
    /// # Arguments
    ///
    /// * `buf` - Buffer to store the string. Must remain valid for the lifetime of the subject.
    /// * `prev_buf` - Optional buffer to store the previous value (pass null for none)
    /// * `size` - Size of the buffer
    /// * `initial` - Initial string value
    ///
    /// # Safety
    ///
    /// The buffers must remain valid for the lifetime of this subject.
    pub unsafe fn new(
        buf: &'static mut [u8],
        prev_buf: Option<&'static mut [u8]>,
        initial: &CStr,
    ) -> Self {
        let mut raw = MaybeUninit::<neo_lvgl_sys::lv_subject_t>::uninit();
        let prev_ptr = prev_buf
            .map(|b| b.as_mut_ptr().cast())
            .unwrap_or(core::ptr::null_mut());

        neo_lvgl_sys::lv_subject_init_string(
            raw.as_mut_ptr(),
            buf.as_mut_ptr().cast(),
            prev_ptr,
            buf.len(),
            initial.as_ptr().cast(),
        );

        Self {
            raw: raw.assume_init(),
        }
    }

    /// Set the string value (copies the string, notifies observers)
    pub fn set(&mut self, value: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_subject_copy_string(&mut self.raw, value.as_ptr().cast());
        }
    }

    /// Get the current string value
    pub fn get(&mut self) -> Option<&CStr> {
        let ptr = unsafe { neo_lvgl_sys::lv_subject_get_string(&mut self.raw) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr.cast()) })
        }
    }

    /// Get the previous string value
    pub fn previous(&mut self) -> Option<&CStr> {
        let ptr = unsafe { neo_lvgl_sys::lv_subject_get_previous_string(&mut self.raw) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr.cast()) })
        }
    }
}

impl Subject for StringSubject {
    fn raw(&mut self) -> *mut neo_lvgl_sys::lv_subject_t {
        &mut self.raw
    }
}

impl Drop for StringSubject {
    fn drop(&mut self) {
        unsafe {
            neo_lvgl_sys::lv_subject_deinit(&mut self.raw);
        }
    }
}

/// Pointer subject for reactive pointer values
///
/// Use this to store and observe changes to arbitrary pointer data.
pub struct PointerSubject {
    raw: neo_lvgl_sys::lv_subject_t,
}

impl PointerSubject {
    /// Create a new pointer subject
    pub fn new(initial: *mut core::ffi::c_void) -> Self {
        let mut raw = MaybeUninit::<neo_lvgl_sys::lv_subject_t>::uninit();
        unsafe {
            neo_lvgl_sys::lv_subject_init_pointer(raw.as_mut_ptr(), initial);
            Self {
                raw: raw.assume_init(),
            }
        }
    }

    /// Set the pointer value
    pub fn set(&mut self, ptr: *mut core::ffi::c_void) {
        unsafe {
            neo_lvgl_sys::lv_subject_set_pointer(&mut self.raw, ptr);
        }
    }

    /// Get the current pointer value
    pub fn get(&mut self) -> *const core::ffi::c_void {
        unsafe { neo_lvgl_sys::lv_subject_get_pointer(&mut self.raw) }
    }

    /// Get the previous pointer value
    pub fn previous(&mut self) -> *const core::ffi::c_void {
        unsafe { neo_lvgl_sys::lv_subject_get_previous_pointer(&mut self.raw) }
    }
}

impl Subject for PointerSubject {
    fn raw(&mut self) -> *mut neo_lvgl_sys::lv_subject_t {
        &mut self.raw
    }
}

impl Drop for PointerSubject {
    fn drop(&mut self) {
        unsafe {
            neo_lvgl_sys::lv_subject_deinit(&mut self.raw);
        }
    }
}

/// Color subject for reactive color values
pub struct ColorSubject {
    raw: neo_lvgl_sys::lv_subject_t,
}

impl ColorSubject {
    /// Create a new color subject
    pub fn new(initial: Color) -> Self {
        let mut raw = MaybeUninit::<neo_lvgl_sys::lv_subject_t>::uninit();
        unsafe {
            neo_lvgl_sys::lv_subject_init_color(raw.as_mut_ptr(), initial.to_raw());
            Self {
                raw: raw.assume_init(),
            }
        }
    }

    /// Set the color value
    pub fn set(&mut self, color: Color) {
        unsafe {
            neo_lvgl_sys::lv_subject_set_color(&mut self.raw, color.to_raw());
        }
    }

    /// Get the current color value
    pub fn get(&mut self) -> Color {
        unsafe { Color::from_raw(neo_lvgl_sys::lv_subject_get_color(&mut self.raw)) }
    }

    /// Get the previous color value
    pub fn previous(&mut self) -> Color {
        unsafe { Color::from_raw(neo_lvgl_sys::lv_subject_get_previous_color(&mut self.raw)) }
    }
}

impl Subject for ColorSubject {
    fn raw(&mut self) -> *mut neo_lvgl_sys::lv_subject_t {
        &mut self.raw
    }
}

impl Drop for ColorSubject {
    fn drop(&mut self) {
        unsafe {
            neo_lvgl_sys::lv_subject_deinit(&mut self.raw);
        }
    }
}

/// Extension trait for binding widgets to subjects
pub trait ObserverBindExt<'a>: crate::widgets::Widget<'a> {
    /// Bind a checkbox's checked state to an integer subject (0 = unchecked, non-zero = checked)
    fn bind_checked(&self, subject: &mut impl Subject) -> Option<Observer> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_obj_bind_checked(self.raw(), subject.raw());
            Observer::from_raw(ptr)
        }
    }
}

impl<'a, T: crate::widgets::Widget<'a>> ObserverBindExt<'a> for T {}

// Widget-specific bindings are implemented in their respective modules
// Here we provide the core observer infrastructure

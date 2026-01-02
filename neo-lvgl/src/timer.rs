//! Timer system for LVGL
//!
//! LVGL timers allow you to execute periodic tasks. This module provides:
//!
//! - `Timer` - A periodic callback timer
//!
//! # Example
//!
//! ```ignore
//! use lvgl::timer::Timer;
//!
//! // Create a timer that fires every 100ms
//! let timer = Timer::new(100, || {
//!     // Do something periodically
//! }).unwrap();
//!
//! // Pause/resume
//! timer.pause();
//! timer.resume();
//!
//! // Clean up when done
//! timer.delete();
//! ```

use crate::anim::RepeatCount;
use core::ffi::c_void;
use core::ptr::NonNull;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Timer callback type (static function)
pub type TimerCb = unsafe extern "C" fn(*mut neo_lvgl_sys::lv_timer_t);

/// A periodic timer
///
/// Timers execute a callback at a specified interval.
pub struct Timer {
    raw: NonNull<neo_lvgl_sys::lv_timer_t>,
}

impl Timer {
    /// Create a new timer with a static callback
    ///
    /// # Arguments
    ///
    /// * `period_ms` - Timer period in milliseconds
    /// * `cb` - Callback function
    pub fn new_static(period_ms: u32, cb: TimerCb) -> Option<Self> {
        let ptr =
            unsafe { neo_lvgl_sys::lv_timer_create(Some(cb), period_ms, core::ptr::null_mut()) };
        NonNull::new(ptr).map(|raw| Self { raw })
    }

    /// Create a new timer with user data and a static callback
    ///
    /// # Safety
    ///
    /// The user_data pointer must remain valid for the timer's lifetime.
    pub unsafe fn new_with_data(period_ms: u32, cb: TimerCb, user_data: *mut c_void) -> Option<Self> {
        let ptr = neo_lvgl_sys::lv_timer_create(Some(cb), period_ms, user_data);
        NonNull::new(ptr).map(|raw| Self { raw })
    }

    /// Create from raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be valid.
    pub unsafe fn from_raw(raw: *mut neo_lvgl_sys::lv_timer_t) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Pause the timer
    pub fn pause(&self) {
        unsafe {
            neo_lvgl_sys::lv_timer_pause(self.raw.as_ptr());
        }
    }

    /// Resume a paused timer
    pub fn resume(&self) {
        unsafe {
            neo_lvgl_sys::lv_timer_resume(self.raw.as_ptr());
        }
    }

    /// Check if timer is paused
    pub fn is_paused(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_timer_get_paused(self.raw.as_ptr()) }
    }

    /// Reset the timer (restart the period countdown)
    pub fn reset(&self) {
        unsafe {
            neo_lvgl_sys::lv_timer_reset(self.raw.as_ptr());
        }
    }

    /// Make the timer ready to fire immediately on next handler call
    pub fn ready(&self) {
        unsafe {
            neo_lvgl_sys::lv_timer_ready(self.raw.as_ptr());
        }
    }

    /// Set the timer period in milliseconds
    pub fn set_period(&self, period_ms: u32) {
        unsafe {
            neo_lvgl_sys::lv_timer_set_period(self.raw.as_ptr(), period_ms);
        }
    }

    /// Set the callback function
    pub fn set_cb(&self, cb: TimerCb) {
        unsafe {
            neo_lvgl_sys::lv_timer_set_cb(self.raw.as_ptr(), Some(cb));
        }
    }

    /// Set repeat count
    ///
    /// The timer will automatically delete itself after firing this many times.
    pub fn set_repeat(&self, count: RepeatCount) {
        let raw_count = match count {
            RepeatCount::Finite(n) => n as i32,
            RepeatCount::Infinite => -1,
        };
        unsafe {
            neo_lvgl_sys::lv_timer_set_repeat_count(self.raw.as_ptr(), raw_count);
        }
    }

    /// Enable auto-delete when repeat count expires
    pub fn set_auto_delete(&self, auto_delete: bool) {
        unsafe {
            neo_lvgl_sys::lv_timer_set_auto_delete(self.raw.as_ptr(), auto_delete);
        }
    }

    /// Set user data
    pub fn set_user_data(&self, data: *mut c_void) {
        unsafe {
            neo_lvgl_sys::lv_timer_set_user_data(self.raw.as_ptr(), data);
        }
    }

    /// Get user data
    pub fn user_data(&self) -> *mut c_void {
        unsafe { neo_lvgl_sys::lv_timer_get_user_data(self.raw.as_ptr()) }
    }

    /// Delete the timer
    ///
    /// This consumes the Timer, preventing further use.
    pub fn delete(self) {
        unsafe {
            neo_lvgl_sys::lv_timer_delete(self.raw.as_ptr());
        }
        // Don't run Drop since we've already deleted
        core::mem::forget(self);
    }

    /// Get the raw pointer
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_timer_t {
        self.raw.as_ptr()
    }
}

// Note: We don't implement Drop because LVGL timers may be auto-deleted
// and we don't want to double-free. Users should call delete() explicitly.

// Closure support (requires alloc feature)
#[cfg(feature = "alloc")]
mod closure_support {
    use super::*;

    /// Container for timer closure
    struct TimerClosure {
        callback: Box<dyn FnMut()>,
    }

    /// Trampoline for closure callbacks
    unsafe extern "C" fn closure_trampoline(timer: *mut neo_lvgl_sys::lv_timer_t) {
        let user_data = neo_lvgl_sys::lv_timer_get_user_data(timer);
        if !user_data.is_null() {
            let closure = &mut *(user_data as *mut TimerClosure);
            (closure.callback)();
        }
    }

    /// Trampoline that also cleans up on delete
    #[allow(dead_code)]
    unsafe extern "C" fn cleanup_trampoline(timer: *mut neo_lvgl_sys::lv_timer_t) {
        let user_data = neo_lvgl_sys::lv_timer_get_user_data(timer);
        if !user_data.is_null() {
            // Drop the boxed closure
            let _ = Box::from_raw(user_data as *mut TimerClosure);
        }
    }

    impl Timer {
        /// Create a new timer with a closure callback
        ///
        /// # Example
        ///
        /// ```ignore
        /// let timer = Timer::new(100, || {
        ///     println!("Timer fired!");
        /// });
        /// ```
        pub fn new<F>(period_ms: u32, callback: F) -> Option<Self>
        where
            F: FnMut() + 'static,
        {
            let closure = Box::new(TimerClosure {
                callback: Box::new(callback),
            });
            let raw_closure = Box::into_raw(closure);

            let ptr = unsafe {
                neo_lvgl_sys::lv_timer_create(
                    Some(closure_trampoline),
                    period_ms,
                    raw_closure as *mut c_void,
                )
            };

            NonNull::new(ptr).map(|raw| Self { raw })
        }

        /// Set callback using a closure
        ///
        /// Note: This will leak the previous closure if one was set.
        /// For best results, use this only once or manage the closure lifetime manually.
        pub fn set_closure<F>(&self, callback: F)
        where
            F: FnMut() + 'static,
        {
            let closure = Box::new(TimerClosure {
                callback: Box::new(callback),
            });
            let raw_closure = Box::into_raw(closure);

            unsafe {
                neo_lvgl_sys::lv_timer_set_cb(self.raw.as_ptr(), Some(closure_trampoline));
                neo_lvgl_sys::lv_timer_set_user_data(self.raw.as_ptr(), raw_closure as *mut c_void);
            }
        }
    }
}

// Global timer functions

/// Enable or disable all LVGL timers
pub fn timer_enable(enable: bool) {
    unsafe {
        neo_lvgl_sys::lv_timer_enable(enable);
    }
}

/// Get idle percentage (0-100)
///
/// Returns how much time was spent idle vs. processing timers.
/// 100% = all idle, 0% = constantly processing.
pub fn timer_get_idle() -> u32 {
    unsafe { neo_lvgl_sys::lv_timer_get_idle() }
}

/// Get time until next timer needs to run (milliseconds)
pub fn timer_get_time_until_next() -> u32 {
    unsafe { neo_lvgl_sys::lv_timer_get_time_until_next() }
}

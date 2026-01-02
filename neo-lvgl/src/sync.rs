//! Thread-safe wrappers for LVGL objects
//!
//! LVGL is not thread-safe by default. This module provides wrappers that ensure
//! all access to LVGL objects goes through `lv_lock()`/`lv_unlock()` synchronization.
//!
//! # Example
//!
//! ```ignore
//! use lvgl::sync::Locked;
//! use lvgl::widgets::Button;
//!
//! let btn = Button::new(&screen).unwrap();
//! let safe_btn = Locked::new(btn);
//!
//! // Can now be sent to another thread
//! std::thread::spawn(move || {
//!     // Lock and get a guard with Deref access
//!     let guard = safe_btn.lock();
//!     guard.set_x(100);
//!     guard.set_y(200);
//!     // Lock released when guard drops
//! });
//! ```

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

/// A thread-safe wrapper for LVGL objects.
///
/// This wrapper stores the widget and protects access with LVGL's global lock.
/// Works like `Mutex<T>` but uses `lv_lock()`/`lv_unlock()` instead of a separate mutex.
///
/// # Safety
///
/// This is safe because:
/// 1. All access goes through `lv_lock()`/`lv_unlock()`
/// 2. LVGL's mutex ensures only one thread accesses LVGL at a time
/// 3. `LockedGuard` provides exclusive access while the lock is held
pub struct Locked<W> {
    widget: UnsafeCell<W>,
}

// SAFETY: Access is protected by lv_lock/lv_unlock
unsafe impl<W> Send for Locked<W> {}
unsafe impl<W> Sync for Locked<W> {}

/// RAII guard that holds the LVGL lock and provides access to a widget.
///
/// This guard is returned by [`Locked::lock()`] and implements `Deref` and `DerefMut`
/// to the underlying widget type. The lock is released when the guard is dropped.
///
/// # Example
///
/// ```ignore
/// let guard = safe_btn.lock();
/// guard.set_x(100);  // Deref to Widget methods
/// guard.set_y(200);
/// // Lock released when guard drops
/// ```
pub struct LockedGuard<'a, W> {
    locked: &'a Locked<W>,
}

impl<W> Deref for LockedGuard<'_, W> {
    type Target = W;

    fn deref(&self) -> &W {
        // SAFETY: We hold the LVGL lock
        unsafe { &*self.locked.widget.get() }
    }
}

impl<W> DerefMut for LockedGuard<'_, W> {
    fn deref_mut(&mut self) -> &mut W {
        // SAFETY: We hold the LVGL lock and have exclusive access via &mut self
        unsafe { &mut *self.locked.widget.get() }
    }
}

impl<W> Drop for LockedGuard<'_, W> {
    fn drop(&mut self) {
        unsafe {
            neo_lvgl_sys::lv_unlock();
        }
    }
}

impl<W> Locked<W> {
    /// Wrap a widget for thread-safe access.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let btn = Button::new(&screen).unwrap();
    /// let safe_btn = Locked::new(btn);
    /// ```
    pub fn new(widget: W) -> Self {
        Self {
            widget: UnsafeCell::new(widget),
        }
    }

    /// Lock and get exclusive access to the widget.
    ///
    /// Returns a guard that holds the LVGL lock and provides `Deref`/`DerefMut`
    /// access to the underlying widget. The lock is released when the guard drops.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let guard = safe_btn.lock();
    /// guard.set_x(100);
    /// guard.set_y(200);
    /// // Lock released here
    /// ```
    pub fn lock(&self) -> LockedGuard<'_, W> {
        unsafe {
            neo_lvgl_sys::lv_lock();
        }
        LockedGuard { locked: self }
    }

    /// Access the widget with a closure.
    ///
    /// This is a convenience method that locks, calls the closure, and unlocks.
    ///
    /// # Example
    ///
    /// ```ignore
    /// safe_btn.with(|btn| {
    ///     btn.set_x(100);
    ///     btn.set_y(200);
    /// });
    /// ```
    pub fn with<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&W) -> R,
    {
        let guard = self.lock();
        f(&guard)
    }

    /// Access the widget mutably with a closure.
    ///
    /// # Example
    ///
    /// ```ignore
    /// safe_btn.with_mut(|btn| {
    ///     btn.set_x(100);
    /// });
    /// ```
    pub fn with_mut<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut W) -> R,
    {
        let mut guard = self.lock();
        f(&mut guard)
    }

    /// Get a pointer to the inner widget.
    ///
    /// # Safety
    ///
    /// The caller must ensure proper LVGL locking when using this pointer.
    pub unsafe fn get(&self) -> *mut W {
        self.widget.get()
    }
}

/// Acquire the global LVGL lock.
///
/// Returns a guard that releases the lock when dropped. This is useful when you
/// need to perform multiple LVGL operations atomically without using `Locked<W>`.
///
/// # Example
///
/// ```ignore
/// {
///     let _guard = lvgl_lock();
///     // All LVGL operations here are protected
///     btn.set_x(100);
///     label.set_text(c"Hello");
/// } // Lock released here
/// ```
pub fn lvgl_lock() -> LvglLockGuard {
    unsafe {
        neo_lvgl_sys::lv_lock();
    }
    LvglLockGuard { _private: () }
}

/// Try to acquire the global LVGL lock from an interrupt context.
///
/// Returns `None` if the lock could not be acquired immediately.
pub fn lvgl_try_lock_isr() -> Option<LvglLockGuard> {
    unsafe {
        if neo_lvgl_sys::lv_lock_isr() == neo_lvgl_sys::lv_result_t_LV_RESULT_OK {
            Some(LvglLockGuard { _private: () })
        } else {
            None
        }
    }
}

/// RAII guard for the global LVGL lock.
///
/// This is a simpler alternative to `Locked<W>` when you just need to protect
/// a section of code without type-specific widget access.
pub struct LvglLockGuard {
    _private: (),
}

impl Drop for LvglLockGuard {
    fn drop(&mut self) {
        unsafe {
            neo_lvgl_sys::lv_unlock();
        }
    }
}

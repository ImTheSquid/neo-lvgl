//! Animation system for LVGL
//!
//! This module provides a safe, ergonomic API for LVGL animations including:
//!
//! - `Anim` - Animation builder with fluent API
//! - `AnimHandle` - Handle to a running animation
//! - `AnimTimeline` - Sequencing multiple animations
//! - `AnimPath` - Easing functions
//!
//! # Example
//!
//! ```ignore
//! use lvgl::anim::{Anim, AnimPath, RepeatCount};
//!
//! let mut anim = Anim::new();
//! anim.set_values(0, 100)
//!     .set_duration(500)
//!     .set_path(AnimPath::EaseInOut)
//!     .set_repeat(RepeatCount::Infinite);
//!
//! // Start the animation
//! let handle = anim.start();
//!
//! // With alloc feature, use closure callbacks:
//! // let handle = anim.start_with_exec(|value| {
//! //     widget.set_x(value);
//! // });
//! ```

use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::NonNull;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Animation repeat count
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepeatCount {
    /// Repeat a finite number of times (0 = no repeat, play once)
    Finite(u32),
    /// Repeat forever
    Infinite,
}

impl RepeatCount {
    fn to_raw(self) -> u32 {
        match self {
            RepeatCount::Finite(n) => n,
            RepeatCount::Infinite => neo_lvgl_sys::LV_ANIM_REPEAT_INFINITE,
        }
    }
}

/// Animation path (easing function)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimPath {
    /// Linear progression
    Linear,
    /// Slow start, fast end
    EaseIn,
    /// Fast start, slow end
    EaseOut,
    /// Slow start and end (S-curve)
    EaseInOut,
    /// Bounces past target then settles
    Overshoot,
    /// Multiple bounces at end
    Bounce,
    /// Instant jump to end value
    Step,
    /// Custom cubic bezier curve
    CustomBezier {
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
    },
}

impl AnimPath {
    fn to_raw(self) -> neo_lvgl_sys::lv_anim_path_cb_t {
        match self {
            AnimPath::Linear => Some(neo_lvgl_sys::lv_anim_path_linear),
            AnimPath::EaseIn => Some(neo_lvgl_sys::lv_anim_path_ease_in),
            AnimPath::EaseOut => Some(neo_lvgl_sys::lv_anim_path_ease_out),
            AnimPath::EaseInOut => Some(neo_lvgl_sys::lv_anim_path_ease_in_out),
            AnimPath::Overshoot => Some(neo_lvgl_sys::lv_anim_path_overshoot),
            AnimPath::Bounce => Some(neo_lvgl_sys::lv_anim_path_bounce),
            AnimPath::Step => Some(neo_lvgl_sys::lv_anim_path_step),
            AnimPath::CustomBezier { .. } => Some(neo_lvgl_sys::lv_anim_path_custom_bezier3),
        }
    }
}

/// Progress value from 0.0 to 1.0
///
/// Internally scaled to LVGL's 0-65535 range.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Progress(f32);

impl Progress {
    /// Create a new progress value (clamped to 0.0-1.0)
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Get the progress as a float (0.0-1.0)
    pub fn as_f32(self) -> f32 {
        self.0
    }

    /// Convert to LVGL's raw progress value (0-65535)
    fn to_raw(self) -> u16 {
        (self.0 * 65535.0) as u16
    }

    /// Create from LVGL's raw progress value (0-65535)
    fn from_raw(raw: u16) -> Self {
        Self(raw as f32 / 65535.0)
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self(0.0)
    }
}

/// Animation builder
///
/// Use the builder pattern to configure an animation, then call `start()` or
/// `start_with_exec()` to begin playback.
pub struct Anim {
    raw: neo_lvgl_sys::lv_anim_t,
    bezier_params: Option<(i16, i16, i16, i16)>,
}

impl Anim {
    /// Create a new animation with default settings
    ///
    /// Default: 500ms duration, linear path, no delay, no repeat
    pub fn new() -> Self {
        let mut raw = MaybeUninit::uninit();
        unsafe {
            neo_lvgl_sys::lv_anim_init(raw.as_mut_ptr());
        }
        Self {
            raw: unsafe { raw.assume_init() },
            bezier_params: None,
        }
    }

    /// Set the variable to animate (raw pointer)
    ///
    /// This is the pointer passed to the exec callback.
    pub fn set_var(&mut self, var: *mut c_void) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_var(&mut self.raw, var);
        }
        self
    }

    /// Set the start and end values
    pub fn set_values(&mut self, start: i32, end: i32) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_values(&mut self.raw, start, end);
        }
        self
    }

    /// Set the animation duration in milliseconds
    pub fn set_duration(&mut self, ms: u32) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_duration(&mut self.raw, ms);
        }
        self
    }

    /// Set the delay before animation starts (milliseconds)
    pub fn set_delay(&mut self, ms: u32) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_delay(&mut self.raw, ms);
        }
        self
    }

    /// Set the animation path (easing function)
    pub fn set_path(&mut self, path: AnimPath) -> &mut Self {
        // Store bezier params if using custom bezier
        if let AnimPath::CustomBezier { x1, y1, x2, y2 } = path {
            self.bezier_params = Some((x1, y1, x2, y2));
            unsafe {
                neo_lvgl_sys::lv_anim_set_bezier3_param(&mut self.raw, x1, y1, x2, y2);
            }
        }

        unsafe {
            neo_lvgl_sys::lv_anim_set_path_cb(&mut self.raw, path.to_raw());
        }
        self
    }

    /// Set repeat count
    pub fn set_repeat(&mut self, count: RepeatCount) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_repeat_count(&mut self.raw, count.to_raw());
        }
        self
    }

    /// Set delay between repeats (milliseconds)
    pub fn set_repeat_delay(&mut self, ms: u32) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_repeat_delay(&mut self.raw, ms);
        }
        self
    }

    /// Set reverse playback duration (0 = no reverse)
    ///
    /// After reaching the end value, animate back to start.
    pub fn set_reverse_duration(&mut self, ms: u32) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_reverse_duration(&mut self.raw, ms);
        }
        self
    }

    /// Set delay before reverse playback (milliseconds)
    pub fn set_reverse_delay(&mut self, ms: u32) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_reverse_delay(&mut self.raw, ms);
        }
        self
    }

    /// Apply start value immediately (even with delay)
    pub fn set_early_apply(&mut self, apply: bool) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_early_apply(&mut self.raw, apply);
        }
        self
    }

    /// Set the exec callback (raw function pointer)
    ///
    /// The callback receives (var_ptr, current_value).
    pub fn set_exec_cb(
        &mut self,
        cb: Option<unsafe extern "C" fn(*mut c_void, i32)>,
    ) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_exec_cb(&mut self.raw, cb);
        }
        self
    }

    /// Set the custom exec callback (receives anim pointer)
    ///
    /// The callback receives (anim_ptr, current_value).
    pub fn set_custom_exec_cb(
        &mut self,
        cb: Option<unsafe extern "C" fn(*mut neo_lvgl_sys::lv_anim_t, i32)>,
    ) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_custom_exec_cb(&mut self.raw, cb);
        }
        self
    }

    /// Set start callback (called when animation actually starts after delay)
    pub fn set_start_cb(
        &mut self,
        cb: Option<unsafe extern "C" fn(*mut neo_lvgl_sys::lv_anim_t)>,
    ) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_start_cb(&mut self.raw, cb);
        }
        self
    }

    /// Set completed callback (called when animation finishes)
    pub fn set_completed_cb(
        &mut self,
        cb: Option<unsafe extern "C" fn(*mut neo_lvgl_sys::lv_anim_t)>,
    ) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_completed_cb(&mut self.raw, cb);
        }
        self
    }

    /// Set deleted callback (called when animation is deleted)
    pub fn set_deleted_cb(
        &mut self,
        cb: Option<unsafe extern "C" fn(*mut neo_lvgl_sys::lv_anim_t)>,
    ) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_deleted_cb(&mut self.raw, cb);
        }
        self
    }

    /// Set user data pointer
    pub fn set_user_data(&mut self, data: *mut c_void) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_set_user_data(&mut self.raw, data);
        }
        self
    }

    /// Start the animation
    ///
    /// Returns a handle to the running animation.
    /// Note: The animation is copied internally, so the handle points to the internal copy.
    pub fn start(&self) -> Option<AnimHandle> {
        let ptr = unsafe { neo_lvgl_sys::lv_anim_start(&self.raw) };
        if ptr.is_null() {
            None
        } else {
            Some(AnimHandle { raw: ptr })
        }
    }

    /// Get the raw animation struct (for advanced use)
    pub fn raw(&self) -> &neo_lvgl_sys::lv_anim_t {
        &self.raw
    }

    /// Get mutable raw animation struct (for advanced use)
    pub fn raw_mut(&mut self) -> &mut neo_lvgl_sys::lv_anim_t {
        &mut self.raw
    }
}

impl Default for Anim {
    fn default() -> Self {
        Self::new()
    }
}

// Closure support for animations (requires alloc feature)
#[cfg(feature = "alloc")]
mod closure_support {
    use super::*;

    /// Trampoline for exec callback closures
    unsafe extern "C" fn exec_trampoline(anim: *mut neo_lvgl_sys::lv_anim_t, value: i32) {
        let user_data = neo_lvgl_sys::lv_anim_get_user_data(anim);
        if !user_data.is_null() {
            let callbacks = &mut *(user_data as *mut AnimCallbacks);
            if let Some(ref mut cb) = callbacks.exec {
                cb(value);
            }
        }
    }

    /// Trampoline for completed callback closures
    unsafe extern "C" fn completed_trampoline(anim: *mut neo_lvgl_sys::lv_anim_t) {
        let user_data = neo_lvgl_sys::lv_anim_get_user_data(anim);
        if !user_data.is_null() {
            let callbacks = &mut *(user_data as *mut AnimCallbacks);
            if let Some(ref mut cb) = callbacks.on_completed {
                cb();
            }
        }
    }

    /// Trampoline for start callback closures
    unsafe extern "C" fn start_trampoline(anim: *mut neo_lvgl_sys::lv_anim_t) {
        let user_data = neo_lvgl_sys::lv_anim_get_user_data(anim);
        if !user_data.is_null() {
            let callbacks = &mut *(user_data as *mut AnimCallbacks);
            if let Some(ref mut cb) = callbacks.on_start {
                cb();
            }
        }
    }

    /// Trampoline for deleted callback - cleans up the boxed closures
    unsafe extern "C" fn deleted_trampoline(anim: *mut neo_lvgl_sys::lv_anim_t) {
        let user_data = neo_lvgl_sys::lv_anim_get_user_data(anim);
        if !user_data.is_null() {
            // Drop the boxed callbacks to free memory
            let _ = Box::from_raw(user_data as *mut AnimCallbacks);
        }
    }

    /// Container for animation callbacks
    struct AnimCallbacks {
        exec: Option<Box<dyn FnMut(i32)>>,
        on_start: Option<Box<dyn FnMut()>>,
        on_completed: Option<Box<dyn FnMut()>>,
    }

    /// Builder for animations with closure callbacks
    ///
    /// Use this when you want to set up multiple callbacks for an animation.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let handle = Anim::new()
    ///     .set_values(0, 100)
    ///     .set_duration(1000)
    ///     .with_callbacks()
    ///     .on_exec(|value| {
    ///         widget.set_x(value);
    ///     })
    ///     .on_start(|| {
    ///         println!("Animation started!");
    ///     })
    ///     .on_completed(|| {
    ///         println!("Animation completed!");
    ///     })
    ///     .start();
    /// ```
    pub struct AnimBuilder {
        anim: Anim,
        exec: Option<Box<dyn FnMut(i32)>>,
        on_start: Option<Box<dyn FnMut()>>,
        on_completed: Option<Box<dyn FnMut()>>,
    }

    impl AnimBuilder {
        /// Set the exec callback that receives animation values
        pub fn on_exec<F>(mut self, f: F) -> Self
        where
            F: FnMut(i32) + 'static,
        {
            self.exec = Some(Box::new(f));
            self
        }

        /// Set a callback to run when the animation starts
        pub fn on_start<F>(mut self, f: F) -> Self
        where
            F: FnMut() + 'static,
        {
            self.on_start = Some(Box::new(f));
            self
        }

        /// Set a callback to run when the animation completes
        pub fn on_completed<F>(mut self, f: F) -> Self
        where
            F: FnMut() + 'static,
        {
            self.on_completed = Some(Box::new(f));
            self
        }

        /// Start the animation with all configured callbacks
        pub fn start(mut self) -> Option<AnimHandle> {
            let has_exec = self.exec.is_some();
            let has_start = self.on_start.is_some();
            let has_completed = self.on_completed.is_some();

            let callbacks = Box::new(AnimCallbacks {
                exec: self.exec.take(),
                on_start: self.on_start.take(),
                on_completed: self.on_completed.take(),
            });
            let raw_callbacks = Box::into_raw(callbacks);

            self.anim.set_user_data(raw_callbacks as *mut c_void);
            self.anim.set_deleted_cb(Some(deleted_trampoline));

            if has_exec {
                self.anim.set_custom_exec_cb(Some(exec_trampoline));
            }
            if has_start {
                self.anim.set_start_cb(Some(start_trampoline));
            }
            if has_completed {
                self.anim.set_completed_cb(Some(completed_trampoline));
            }

            self.anim.start()
        }
    }

    impl Anim {
        /// Create a builder for configuring animation callbacks
        ///
        /// Use this when you need to set multiple callbacks (exec, on_start, on_completed).
        pub fn with_callbacks(self) -> AnimBuilder {
            AnimBuilder {
                anim: self,
                exec: None,
                on_start: None,
                on_completed: None,
            }
        }

        /// Start animation with a closure as the exec callback
        ///
        /// This is a convenience method for the common case where you only need
        /// an exec callback. For multiple callbacks, use `with_callbacks()`.
        ///
        /// # Example
        ///
        /// ```ignore
        /// anim.start_with_exec(|value| {
        ///     widget.set_x(value);
        /// });
        /// ```
        pub fn start_with_exec<F>(self, exec: F) -> Option<AnimHandle>
        where
            F: FnMut(i32) + 'static,
        {
            self.with_callbacks().on_exec(exec).start()
        }
    }
}

#[cfg(feature = "alloc")]
pub use closure_support::AnimBuilder;

/// Handle to a running animation
///
/// This handle can be used to control an animation that is currently playing.
pub struct AnimHandle {
    raw: *mut neo_lvgl_sys::lv_anim_t,
}

impl AnimHandle {
    /// Create from raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid running animation.
    pub unsafe fn from_raw(raw: *mut neo_lvgl_sys::lv_anim_t) -> Self {
        Self { raw }
    }

    /// Pause the animation
    pub fn pause(&self) {
        unsafe {
            neo_lvgl_sys::lv_anim_pause(self.raw);
        }
    }

    /// Resume a paused animation
    pub fn resume(&self) {
        unsafe {
            neo_lvgl_sys::lv_anim_resume(self.raw);
        }
    }

    /// Check if the animation is paused
    pub fn is_paused(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_anim_is_paused(self.raw) }
    }

    /// Delete the animation
    ///
    /// After calling this, the handle is invalid.
    pub fn delete(self) {
        unsafe {
            neo_lvgl_sys::lv_anim_delete(self.raw as *mut c_void, None);
        }
    }

    /// Get the raw pointer
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_anim_t {
        self.raw
    }
}

/// Animation timeline for sequencing multiple animations
///
/// Timelines allow you to coordinate multiple animations with precise timing.
pub struct AnimTimeline {
    raw: NonNull<neo_lvgl_sys::lv_anim_timeline_t>,
}

impl AnimTimeline {
    /// Create a new animation timeline
    pub fn new() -> Option<Self> {
        let ptr = unsafe { neo_lvgl_sys::lv_anim_timeline_create() };
        NonNull::new(ptr).map(|raw| Self { raw })
    }

    /// Add an animation to the timeline
    ///
    /// # Arguments
    ///
    /// * `start_time` - When to start this animation (milliseconds from timeline start)
    /// * `anim` - The animation to add
    pub fn add(&mut self, start_time: u32, anim: &Anim) -> &mut Self {
        unsafe {
            neo_lvgl_sys::lv_anim_timeline_add(self.raw.as_ptr(), start_time, &anim.raw);
        }
        self
    }

    /// Start the timeline
    ///
    /// Returns the total playtime in milliseconds.
    pub fn start(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_anim_timeline_start(self.raw.as_ptr()) }
    }

    /// Pause the timeline
    pub fn pause(&self) {
        unsafe {
            neo_lvgl_sys::lv_anim_timeline_pause(self.raw.as_ptr());
        }
    }

    /// Set reverse playback
    pub fn set_reverse(&self, reverse: bool) {
        unsafe {
            neo_lvgl_sys::lv_anim_timeline_set_reverse(self.raw.as_ptr(), reverse);
        }
    }

    /// Get reverse playback state
    pub fn reverse(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_anim_timeline_get_reverse(self.raw.as_ptr()) }
    }

    /// Set timeline progress
    pub fn set_progress(&self, progress: Progress) {
        unsafe {
            neo_lvgl_sys::lv_anim_timeline_set_progress(self.raw.as_ptr(), progress.to_raw());
        }
    }

    /// Get timeline progress
    pub fn progress(&self) -> Progress {
        let raw = unsafe { neo_lvgl_sys::lv_anim_timeline_get_progress(self.raw.as_ptr()) };
        Progress::from_raw(raw)
    }

    /// Set initial delay before timeline starts
    pub fn set_delay(&self, ms: u32) {
        unsafe {
            neo_lvgl_sys::lv_anim_timeline_set_delay(self.raw.as_ptr(), ms);
        }
    }

    /// Get the delay
    pub fn delay(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_anim_timeline_get_delay(self.raw.as_ptr()) }
    }

    /// Set repeat count for the entire timeline
    pub fn set_repeat(&self, count: RepeatCount) {
        unsafe {
            neo_lvgl_sys::lv_anim_timeline_set_repeat_count(self.raw.as_ptr(), count.to_raw());
        }
    }

    /// Get repeat count
    pub fn repeat_count(&self) -> RepeatCount {
        let raw = unsafe { neo_lvgl_sys::lv_anim_timeline_get_repeat_count(self.raw.as_ptr()) };
        if raw == neo_lvgl_sys::LV_ANIM_REPEAT_INFINITE {
            RepeatCount::Infinite
        } else {
            RepeatCount::Finite(raw)
        }
    }

    /// Set repeat delay
    pub fn set_repeat_delay(&self, ms: u32) {
        unsafe {
            neo_lvgl_sys::lv_anim_timeline_set_repeat_delay(self.raw.as_ptr(), ms);
        }
    }

    /// Get repeat delay
    pub fn repeat_delay(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_anim_timeline_get_repeat_delay(self.raw.as_ptr()) }
    }

    /// Get total playtime in milliseconds
    pub fn playtime(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_anim_timeline_get_playtime(self.raw.as_ptr()) }
    }

    /// Set user data
    pub fn set_user_data(&self, data: *mut c_void) {
        unsafe {
            neo_lvgl_sys::lv_anim_timeline_set_user_data(self.raw.as_ptr(), data);
        }
    }

    /// Get user data
    pub fn user_data(&self) -> *mut c_void {
        unsafe { neo_lvgl_sys::lv_anim_timeline_get_user_data(self.raw.as_ptr()) }
    }

    /// Get the raw pointer
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_anim_timeline_t {
        self.raw.as_ptr()
    }
}

impl Default for AnimTimeline {
    fn default() -> Self {
        Self::new().expect("Failed to create animation timeline")
    }
}

impl Drop for AnimTimeline {
    fn drop(&mut self) {
        unsafe {
            neo_lvgl_sys::lv_anim_timeline_delete(self.raw.as_ptr());
        }
    }
}

// Global animation functions

/// Delete all animations for a specific variable
pub fn anim_delete(var: *mut c_void) -> bool {
    unsafe { neo_lvgl_sys::lv_anim_delete(var, None) }
}

/// Delete all animations
pub fn anim_delete_all() {
    unsafe {
        neo_lvgl_sys::lv_anim_delete_all();
    }
}

/// Get the number of currently running animations
pub fn anim_count_running() -> u16 {
    unsafe { neo_lvgl_sys::lv_anim_count_running() }
}

/// Manually refresh all animations
pub fn anim_refr_now() {
    unsafe {
        neo_lvgl_sys::lv_anim_refr_now();
    }
}

/// Calculate animation duration based on speed
///
/// # Arguments
///
/// * `speed` - Speed in units per second
/// * `start` - Start value
/// * `end` - End value
///
/// Returns duration in milliseconds.
pub fn anim_speed_to_time(speed: u32, start: i32, end: i32) -> u32 {
    unsafe { neo_lvgl_sys::lv_anim_speed_to_time(speed, start, end) }
}

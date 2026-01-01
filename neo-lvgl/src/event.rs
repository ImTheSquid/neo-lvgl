//! Event handling system
//!
//! LVGL events are handled through callbacks. This module provides:
//!
//! - `EventCode` - Type-safe event codes
//! - `Event` - Wrapper for accessing event data
//! - Static function callbacks (always available)
//! - Closure callbacks (requires `alloc` feature)

use core::ffi::c_void;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Event codes for LVGL widgets
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum EventCode {
    /// Object pressed
    Pressed = neo_lvgl_sys::lv_event_code_t_LV_EVENT_PRESSED,
    /// Object still being pressed (called continuously while pressing)
    Pressing = neo_lvgl_sys::lv_event_code_t_LV_EVENT_PRESSING,
    /// Object is still being pressed but slid out of its area
    PressLost = neo_lvgl_sys::lv_event_code_t_LV_EVENT_PRESS_LOST,
    /// Object released
    Released = neo_lvgl_sys::lv_event_code_t_LV_EVENT_RELEASED,
    /// Object was pressed for a short period then released
    ShortClicked = neo_lvgl_sys::lv_event_code_t_LV_EVENT_SHORT_CLICKED,
    /// Object clicked (released after pressing)
    Clicked = neo_lvgl_sys::lv_event_code_t_LV_EVENT_CLICKED,
    /// Object pressed for long time
    LongPressed = neo_lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED,
    /// Called after long press repeatedly
    LongPressedRepeat = neo_lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED_REPEAT,
    /// Focus received
    Focused = neo_lvgl_sys::lv_event_code_t_LV_EVENT_FOCUSED,
    /// Focus lost
    Defocused = neo_lvgl_sys::lv_event_code_t_LV_EVENT_DEFOCUSED,
    /// Value changed
    ValueChanged = neo_lvgl_sys::lv_event_code_t_LV_EVENT_VALUE_CHANGED,
    /// Ready (e.g., animation complete)
    Ready = neo_lvgl_sys::lv_event_code_t_LV_EVENT_READY,
    /// Cancel
    Cancel = neo_lvgl_sys::lv_event_code_t_LV_EVENT_CANCEL,
    /// Object is being deleted
    Delete = neo_lvgl_sys::lv_event_code_t_LV_EVENT_DELETE,
    /// Child added
    ChildCreated = neo_lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_CREATED,
    /// Child removed
    ChildDeleted = neo_lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_DELETED,
    /// Child changed
    ChildChanged = neo_lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_CHANGED,
    /// Screen load started
    ScreenLoadStart = neo_lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_LOAD_START,
    /// Screen loaded
    ScreenLoaded = neo_lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_LOADED,
    /// Screen unload started
    ScreenUnloadStart = neo_lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_UNLOAD_START,
    /// Screen unloaded
    ScreenUnloaded = neo_lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_UNLOADED,
    /// Object size changed
    SizeChanged = neo_lvgl_sys::lv_event_code_t_LV_EVENT_SIZE_CHANGED,
    /// Object style changed
    StyleChanged = neo_lvgl_sys::lv_event_code_t_LV_EVENT_STYLE_CHANGED,
    /// All events (for filtering)
    All = neo_lvgl_sys::lv_event_code_t_LV_EVENT_ALL,
}

impl EventCode {
    /// Create from raw LVGL event code
    pub fn from_raw(code: neo_lvgl_sys::lv_event_code_t) -> Option<Self> {
        match code {
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_PRESSED => Some(Self::Pressed),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_PRESSING => Some(Self::Pressing),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_PRESS_LOST => Some(Self::PressLost),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_RELEASED => Some(Self::Released),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_SHORT_CLICKED => Some(Self::ShortClicked),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_CLICKED => Some(Self::Clicked),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED => Some(Self::LongPressed),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED_REPEAT => Some(Self::LongPressedRepeat),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_FOCUSED => Some(Self::Focused),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_DEFOCUSED => Some(Self::Defocused),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_VALUE_CHANGED => Some(Self::ValueChanged),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_READY => Some(Self::Ready),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_CANCEL => Some(Self::Cancel),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_DELETE => Some(Self::Delete),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_CREATED => Some(Self::ChildCreated),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_DELETED => Some(Self::ChildDeleted),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_CHANGED => Some(Self::ChildChanged),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_LOAD_START => Some(Self::ScreenLoadStart),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_LOADED => Some(Self::ScreenLoaded),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_UNLOAD_START => Some(Self::ScreenUnloadStart),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_UNLOADED => Some(Self::ScreenUnloaded),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_SIZE_CHANGED => Some(Self::SizeChanged),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_STYLE_CHANGED => Some(Self::StyleChanged),
            neo_lvgl_sys::lv_event_code_t_LV_EVENT_ALL => Some(Self::All),
            _ => None,
        }
    }

    /// Get the raw LVGL event code
    #[inline]
    pub fn to_raw(self) -> neo_lvgl_sys::lv_event_code_t {
        self as neo_lvgl_sys::lv_event_code_t
    }
}

/// Wrapper for LVGL event data
pub struct Event {
    raw: *mut neo_lvgl_sys::lv_event_t,
}

impl Event {
    /// Create from raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be valid for the duration of use.
    #[inline]
    pub unsafe fn from_raw(raw: *mut neo_lvgl_sys::lv_event_t) -> Self {
        Self { raw }
    }

    /// Get the event code
    pub fn code(&self) -> Option<EventCode> {
        unsafe {
            let code = neo_lvgl_sys::lv_event_get_code(self.raw);
            EventCode::from_raw(code)
        }
    }

    /// Get the raw event code
    pub fn code_raw(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_event_get_code(self.raw) }
    }

    /// Get the target object
    pub fn target_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        unsafe { neo_lvgl_sys::lv_event_get_target_obj(self.raw) }
    }

    /// Get the current target object (may differ from target in bubbled events)
    pub fn current_target_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        unsafe { neo_lvgl_sys::lv_event_get_current_target_obj(self.raw) }
    }

    /// Get user data pointer
    ///
    /// # Safety
    ///
    /// Caller must ensure the type T matches what was passed when registering the callback.
    pub unsafe fn user_data<T>(&self) -> Option<&mut T> {
        let ptr = neo_lvgl_sys::lv_event_get_user_data(self.raw);
        if ptr.is_null() {
            None
        } else {
            Some(&mut *(ptr as *mut T))
        }
    }

    /// Get the raw event pointer
    #[inline]
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_event_t {
        self.raw
    }

    /// Stop event propagation
    pub fn stop_bubbling(&self) {
        unsafe {
            neo_lvgl_sys::lv_event_stop_bubbling(self.raw);
        }
    }

    /// Stop further processing of this event
    pub fn stop_processing(&self) {
        unsafe {
            neo_lvgl_sys::lv_event_stop_processing(self.raw);
        }
    }
}

/// Static event callback type
pub type EventCb = unsafe extern "C" fn(*mut neo_lvgl_sys::lv_event_t);

/// Trait for adding event handlers to widgets
pub trait EventHandler {
    /// Get the raw object pointer
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t;

    /// Add an event handler using a static function.
    ///
    /// # Arguments
    ///
    /// * `event` - The event code to listen for
    /// * `cb` - The callback function
    fn on_event(&self, event: EventCode, cb: EventCb) {
        unsafe {
            neo_lvgl_sys::lv_obj_add_event_cb(
                self.obj_raw(),
                Some(cb),
                event.to_raw(),
                core::ptr::null_mut(),
            );
        }
    }

    /// Add an event handler with user data.
    ///
    /// # Arguments
    ///
    /// * `event` - The event code to listen for
    /// * `cb` - The callback function
    /// * `user_data` - Pointer to user data accessible in the callback
    ///
    /// # Safety
    ///
    /// The user_data pointer must remain valid for the lifetime of the event handler.
    unsafe fn on_event_with_data<T>(&self, event: EventCode, cb: EventCb, user_data: *mut T) {
        neo_lvgl_sys::lv_obj_add_event_cb(
            self.obj_raw(),
            Some(cb),
            event.to_raw(),
            user_data as *mut c_void,
        );
    }

    /// Add an event handler for any event.
    fn on_all_events(&self, cb: EventCb) {
        self.on_event(EventCode::All, cb);
    }
}

// Closure support (requires alloc feature)
#[cfg(feature = "alloc")]
mod closure_support {
    use super::*;

    /// Trampoline function for closure callbacks
    unsafe extern "C" fn closure_trampoline(e: *mut neo_lvgl_sys::lv_event_t) {
        let user_data = neo_lvgl_sys::lv_event_get_user_data(e);
        if !user_data.is_null() {
            let closure = &mut *(user_data as *mut Box<dyn Fn(&Event)>);
            let event = Event::from_raw(e);
            closure(&event);
        }
    }

    /// Trampoline for FnMut closures
    unsafe extern "C" fn closure_trampoline_mut(e: *mut neo_lvgl_sys::lv_event_t) {
        let user_data = neo_lvgl_sys::lv_event_get_user_data(e);
        if !user_data.is_null() {
            let closure = &mut *(user_data as *mut Box<dyn FnMut(&Event)>);
            let event = Event::from_raw(e);
            closure(&event);
        }
    }

    /// Extension trait for closure-based event handlers
    pub trait ClosureEventHandler: EventHandler {
        /// Add a click handler using a closure.
        ///
        /// # Example
        ///
        /// ```ignore
        /// btn.on_clicked(|event| {
        ///     // Handle click
        /// });
        /// ```
        fn on_clicked<F>(&self, handler: F)
        where
            F: Fn(&Event) + 'static,
        {
            self.on_event_closure(EventCode::Clicked, handler);
        }

        /// Add a press handler using a closure.
        fn on_pressed<F>(&self, handler: F)
        where
            F: Fn(&Event) + 'static,
        {
            self.on_event_closure(EventCode::Pressed, handler);
        }

        /// Add a release handler using a closure.
        fn on_released<F>(&self, handler: F)
        where
            F: Fn(&Event) + 'static,
        {
            self.on_event_closure(EventCode::Released, handler);
        }

        /// Add a value changed handler using a closure.
        fn on_value_changed<F>(&self, handler: F)
        where
            F: Fn(&Event) + 'static,
        {
            self.on_event_closure(EventCode::ValueChanged, handler);
        }

        /// Add a focus handler using a closure.
        fn on_focused<F>(&self, handler: F)
        where
            F: Fn(&Event) + 'static,
        {
            self.on_event_closure(EventCode::Focused, handler);
        }

        /// Add a defocus handler using a closure.
        fn on_defocused<F>(&self, handler: F)
        where
            F: Fn(&Event) + 'static,
        {
            self.on_event_closure(EventCode::Defocused, handler);
        }

        /// Add an event handler using a closure.
        ///
        /// This allocates the closure on the heap.
        fn on_event_closure<F>(&self, event: EventCode, handler: F)
        where
            F: Fn(&Event) + 'static,
        {
            let boxed: Box<Box<dyn Fn(&Event)>> = Box::new(Box::new(handler));
            let raw = Box::into_raw(boxed);

            unsafe {
                neo_lvgl_sys::lv_obj_add_event_cb(
                    self.obj_raw(),
                    Some(closure_trampoline),
                    event.to_raw(),
                    raw as *mut c_void,
                );
            }
        }

        /// Add an event handler using a mutable closure.
        fn on_event_closure_mut<F>(&self, event: EventCode, handler: F)
        where
            F: FnMut(&Event) + 'static,
        {
            let boxed: Box<Box<dyn FnMut(&Event)>> = Box::new(Box::new(handler));
            let raw = Box::into_raw(boxed);

            unsafe {
                neo_lvgl_sys::lv_obj_add_event_cb(
                    self.obj_raw(),
                    Some(closure_trampoline_mut),
                    event.to_raw(),
                    raw as *mut c_void,
                );
            }
        }
    }

    // Implement ClosureEventHandler for all types that implement EventHandler
    impl<T: EventHandler> ClosureEventHandler for T {}
}

#[cfg(feature = "alloc")]
pub use closure_support::ClosureEventHandler;

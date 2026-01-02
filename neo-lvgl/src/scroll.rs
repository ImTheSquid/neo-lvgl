//! Scrolling support for LVGL widgets
//!
//! This module provides scrolling functionality for containers.
//!
//! # Example
//!
//! ```ignore
//! use lvgl::prelude::*;
//! use lvgl::scroll::{ScrollbarMode, ScrollSnap};
//! use lvgl::layout::Direction;
//!
//! let container = Obj::new(&screen).unwrap();
//! container.set_size(200, 200);
//!
//! // Enable scrolling with auto-hiding scrollbar
//! container.set_scrollbar_mode(ScrollbarMode::Auto);
//! container.set_scroll_dir(Direction::VER);
//!
//! // Snap to children when scrolling ends
//! container.set_scroll_snap_y(ScrollSnap::Center);
//!
//! // Add content larger than container
//! for i in 0..20 {
//!     let label = Label::new(&container).unwrap();
//!     label.set_text(c"Item");
//! }
//! ```

use crate::layout::Direction;
use crate::widgets::Point;

/// Scrollbar visibility mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollbarMode {
    /// Never show scrollbars
    Off,
    /// Always show scrollbars
    On,
    /// Show scrollbars while actively scrolling
    Active,
    /// Show scrollbars when content is scrollable
    Auto,
}

impl ScrollbarMode {
    pub(crate) fn to_raw(self) -> neo_lvgl_sys::lv_scrollbar_mode_t {
        match self {
            ScrollbarMode::Off => neo_lvgl_sys::lv_scrollbar_mode_t_LV_SCROLLBAR_MODE_OFF,
            ScrollbarMode::On => neo_lvgl_sys::lv_scrollbar_mode_t_LV_SCROLLBAR_MODE_ON,
            ScrollbarMode::Active => neo_lvgl_sys::lv_scrollbar_mode_t_LV_SCROLLBAR_MODE_ACTIVE,
            ScrollbarMode::Auto => neo_lvgl_sys::lv_scrollbar_mode_t_LV_SCROLLBAR_MODE_AUTO,
        }
    }

    pub(crate) fn from_raw(raw: neo_lvgl_sys::lv_scrollbar_mode_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_scrollbar_mode_t_LV_SCROLLBAR_MODE_OFF => ScrollbarMode::Off,
            neo_lvgl_sys::lv_scrollbar_mode_t_LV_SCROLLBAR_MODE_ON => ScrollbarMode::On,
            neo_lvgl_sys::lv_scrollbar_mode_t_LV_SCROLLBAR_MODE_ACTIVE => ScrollbarMode::Active,
            _ => ScrollbarMode::Auto,
        }
    }
}

/// Scroll snap behavior
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollSnap {
    /// No snapping
    None,
    /// Snap children to the start edge
    Start,
    /// Snap children to the end edge
    End,
    /// Snap children to the center
    Center,
}

impl ScrollSnap {
    pub(crate) fn to_raw(self) -> neo_lvgl_sys::lv_scroll_snap_t {
        match self {
            ScrollSnap::None => neo_lvgl_sys::lv_scroll_snap_t_LV_SCROLL_SNAP_NONE,
            ScrollSnap::Start => neo_lvgl_sys::lv_scroll_snap_t_LV_SCROLL_SNAP_START,
            ScrollSnap::End => neo_lvgl_sys::lv_scroll_snap_t_LV_SCROLL_SNAP_END,
            ScrollSnap::Center => neo_lvgl_sys::lv_scroll_snap_t_LV_SCROLL_SNAP_CENTER,
        }
    }

    pub(crate) fn from_raw(raw: neo_lvgl_sys::lv_scroll_snap_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_scroll_snap_t_LV_SCROLL_SNAP_NONE => ScrollSnap::None,
            neo_lvgl_sys::lv_scroll_snap_t_LV_SCROLL_SNAP_START => ScrollSnap::Start,
            neo_lvgl_sys::lv_scroll_snap_t_LV_SCROLL_SNAP_END => ScrollSnap::End,
            _ => ScrollSnap::Center,
        }
    }
}

/// Scrolling extension trait for widgets
///
/// Provides scrolling-related methods for any widget.
pub trait ScrollExt<'a>: crate::widgets::Widget<'a> {
    // === Scrollbar configuration ===

    /// Set the scrollbar visibility mode
    fn set_scrollbar_mode(&self, mode: ScrollbarMode) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_scrollbar_mode(self.raw(), mode.to_raw());
        }
    }

    /// Get the current scrollbar mode
    fn scrollbar_mode(&self) -> ScrollbarMode {
        unsafe { ScrollbarMode::from_raw(neo_lvgl_sys::lv_obj_get_scrollbar_mode(self.raw())) }
    }

    /// Set which directions the widget can be scrolled
    fn set_scroll_dir(&self, dir: Direction) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_scroll_dir(self.raw(), dir.to_raw());
        }
    }

    /// Get the allowed scroll directions
    fn scroll_dir(&self) -> Direction {
        unsafe { Direction::from_raw(neo_lvgl_sys::lv_obj_get_scroll_dir(self.raw())) }
    }

    /// Set horizontal scroll snapping behavior
    fn set_scroll_snap_x(&self, snap: ScrollSnap) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_scroll_snap_x(self.raw(), snap.to_raw());
        }
    }

    /// Get horizontal scroll snapping behavior
    fn scroll_snap_x(&self) -> ScrollSnap {
        unsafe { ScrollSnap::from_raw(neo_lvgl_sys::lv_obj_get_scroll_snap_x(self.raw())) }
    }

    /// Set vertical scroll snapping behavior
    fn set_scroll_snap_y(&self, snap: ScrollSnap) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_scroll_snap_y(self.raw(), snap.to_raw());
        }
    }

    /// Get vertical scroll snapping behavior
    fn scroll_snap_y(&self) -> ScrollSnap {
        unsafe { ScrollSnap::from_raw(neo_lvgl_sys::lv_obj_get_scroll_snap_y(self.raw())) }
    }

    // === Scroll position ===

    /// Get the current horizontal scroll position
    fn scroll_x(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_obj_get_scroll_x(self.raw()) }
    }

    /// Get the current vertical scroll position
    fn scroll_y(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_obj_get_scroll_y(self.raw()) }
    }

    /// Get how many pixels can be scrolled down
    fn scroll_top(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_obj_get_scroll_top(self.raw()) }
    }

    /// Get how many pixels can be scrolled up from the bottom
    fn scroll_bottom(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_obj_get_scroll_bottom(self.raw()) }
    }

    /// Get how many pixels can be scrolled right
    fn scroll_left(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_obj_get_scroll_left(self.raw()) }
    }

    /// Get how many pixels can be scrolled left from the right edge
    fn scroll_right(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_obj_get_scroll_right(self.raw()) }
    }

    // === Scroll actions ===

    /// Scroll by a given amount
    ///
    /// # Arguments
    ///
    /// * `dx` - Horizontal scroll amount (positive = right)
    /// * `dy` - Vertical scroll amount (positive = down)
    /// * `anim` - Whether to animate the scroll
    fn scroll_by(&self, dx: i32, dy: i32, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_obj_scroll_by(self.raw(), dx, dy, anim);
        }
    }

    /// Scroll to an absolute position
    ///
    /// # Arguments
    ///
    /// * `x` - Target horizontal position
    /// * `y` - Target vertical position
    /// * `anim` - Whether to animate the scroll
    fn scroll_to(&self, x: i32, y: i32, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_obj_scroll_to(self.raw(), x, y, anim);
        }
    }

    /// Scroll to a horizontal position
    fn scroll_to_x(&self, x: i32, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_obj_scroll_to_x(self.raw(), x, anim);
        }
    }

    /// Scroll to a vertical position
    fn scroll_to_y(&self, y: i32, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_obj_scroll_to_y(self.raw(), y, anim);
        }
    }

    /// Scroll this widget into view within its parent
    ///
    /// If this widget is outside the visible area of its parent,
    /// the parent will be scrolled to make this widget visible.
    fn scroll_to_view(&self, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_obj_scroll_to_view(self.raw(), anim);
        }
    }

    /// Scroll this widget into view, recursively scrolling all ancestor containers
    fn scroll_to_view_recursive(&self, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_obj_scroll_to_view_recursive(self.raw(), anim);
        }
    }

    // === Scroll state ===

    /// Check if the widget is currently being scrolled
    fn is_scrolling(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_obj_is_scrolling(self.raw()) }
    }

    /// Stop any ongoing scroll animation
    fn stop_scroll_anim(&self) {
        unsafe {
            neo_lvgl_sys::lv_obj_stop_scroll_anim(self.raw());
        }
    }

    /// Update snap alignment after scroll position changes
    fn update_snap(&self, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_obj_update_snap(self.raw(), anim);
        }
    }

    /// Readjust scroll position (useful after content size changes)
    fn readjust_scroll(&self, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_obj_readjust_scroll(self.raw(), anim);
        }
    }

    /// Invalidate (redraw) the scrollbars
    fn scrollbar_invalidate(&self) {
        unsafe {
            neo_lvgl_sys::lv_obj_scrollbar_invalidate(self.raw());
        }
    }

    /// Get the final scroll position after any ongoing animation
    fn scroll_end(&self) -> Point {
        let mut end = neo_lvgl_sys::lv_point_t { x: 0, y: 0 };
        unsafe {
            neo_lvgl_sys::lv_obj_get_scroll_end(self.raw(), &mut end);
        }
        Point { x: end.x, y: end.y }
    }
}

// Implement ScrollExt for all Widget types
impl<'a, T: crate::widgets::Widget<'a>> ScrollExt<'a> for T {}

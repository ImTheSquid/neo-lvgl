//! Line widget

use super::{Obj, Widget};
use crate::event::EventHandler;

/// A point with precise coordinates
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Create a new point
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Line widget
///
/// Draws a line through a series of points.
///
/// # Example
///
/// ```ignore
/// let line = Line::new(&screen).unwrap();
/// let points = [
///     Point::new(0, 0),
///     Point::new(100, 50),
///     Point::new(200, 0),
/// ];
/// line.set_points(&points);
/// ```
#[derive(Clone, Copy)]
pub struct Line<'a> {
    obj: Obj<'a>,
}

impl<'a> Line<'a> {
    /// Create a new line as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_line_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the points that define the line
    ///
    /// Note: The points array must remain valid for the lifetime of the line widget,
    /// or until new points are set.
    pub fn set_points(&self, points: &[Point]) {
        // Convert to LVGL point format
        // Note: This is a simplification - for production use, you'd want to
        // handle the lifetime more carefully
        unsafe {
            neo_lvgl_sys::lv_line_set_points(
                self.obj.raw(),
                points.as_ptr() as *const neo_lvgl_sys::lv_point_precise_t,
                points.len() as u32,
            );
        }
    }

    /// Set the points (mutable version)
    ///
    /// The points array can be modified after setting.
    pub fn set_points_mutable(&self, points: &mut [Point]) {
        unsafe {
            neo_lvgl_sys::lv_line_set_points_mutable(
                self.obj.raw(),
                points.as_mut_ptr() as *mut neo_lvgl_sys::lv_point_precise_t,
                points.len() as u32,
            );
        }
    }

    /// Invert the Y axis (0 at bottom instead of top)
    pub fn set_y_invert(&self, invert: bool) {
        unsafe {
            neo_lvgl_sys::lv_line_set_y_invert(self.obj.raw(), invert);
        }
    }

    /// Check if Y axis is inverted
    pub fn y_invert(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_line_get_y_invert(self.obj.raw()) }
    }

    /// Get the number of points
    pub fn point_count(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_line_get_point_count(self.obj.raw()) }
    }
}

impl<'a> Widget<'a> for Line<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Line<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



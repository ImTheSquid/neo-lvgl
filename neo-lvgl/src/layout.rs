//! Layout system for LVGL
//!
//! This module provides Flexbox and Grid layout support for arranging widgets.
//!
//! # Flexbox Layout
//!
//! ```ignore
//! use lvgl::prelude::*;
//!
//! // Create a container with flex layout
//! let container = Obj::new(&screen).unwrap();
//! container.set_flex_flow(FlexFlow::Row);
//! container.set_flex_align(FlexAlign::Center, FlexAlign::Center, FlexAlign::Center);
//!
//! // Children will be arranged horizontally
//! let btn1 = Button::new(&container).unwrap();
//! let btn2 = Button::new(&container).unwrap();
//! ```
//!
//! # Grid Layout
//!
//! ```ignore
//! use lvgl::prelude::*;
//! use lvgl::layout::{grid_fr, GRID_TEMPLATE_LAST, GRID_CONTENT};
//!
//! let container = Obj::new(&screen).unwrap();
//!
//! // Define columns: 1fr, 1fr, 1fr (3 equal columns)
//! static COL_DSC: [i32; 4] = [grid_fr(1), grid_fr(1), grid_fr(1), GRID_TEMPLATE_LAST];
//! // Define rows: auto, auto
//! static ROW_DSC: [i32; 3] = [GRID_CONTENT, GRID_CONTENT, GRID_TEMPLATE_LAST];
//!
//! container.set_grid_dsc_array(&COL_DSC, &ROW_DSC);
//!
//! // Place child at column 0, row 0
//! let btn = Button::new(&container).unwrap();
//! btn.set_grid_cell(GridAlign::Stretch, 0, 1, GridAlign::Stretch, 0, 1);
//! ```

use bitflags::bitflags;

/// Flex flow direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexFlow {
    /// Arrange children in a row (horizontal)
    Row,
    /// Arrange children in a column (vertical)
    Column,
    /// Row with wrapping to next line
    RowWrap,
    /// Row in reverse order
    RowReverse,
    /// Row with wrap in reverse
    RowWrapReverse,
    /// Column with wrapping
    ColumnWrap,
    /// Column in reverse order
    ColumnReverse,
    /// Column with wrap in reverse
    ColumnWrapReverse,
}

impl FlexFlow {
    pub(crate) fn to_raw(self) -> neo_lvgl_sys::lv_flex_flow_t {
        match self {
            FlexFlow::Row => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW,
            FlexFlow::Column => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN,
            FlexFlow::RowWrap => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW_WRAP,
            FlexFlow::RowReverse => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW_REVERSE,
            FlexFlow::RowWrapReverse => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW_WRAP_REVERSE,
            FlexFlow::ColumnWrap => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_WRAP,
            FlexFlow::ColumnReverse => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_REVERSE,
            FlexFlow::ColumnWrapReverse => {
                neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_WRAP_REVERSE
            }
        }
    }
}

/// Flex alignment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexAlign {
    /// Align to the start
    Start,
    /// Align to the end
    End,
    /// Align to the center
    Center,
    /// Distribute with equal space around items
    SpaceEvenly,
    /// Distribute with space around items (half space at edges)
    SpaceAround,
    /// Distribute with space between items only
    SpaceBetween,
}

impl FlexAlign {
    pub(crate) fn to_raw(self) -> neo_lvgl_sys::lv_flex_align_t {
        match self {
            FlexAlign::Start => neo_lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_START,
            FlexAlign::End => neo_lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_END,
            FlexAlign::Center => neo_lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_CENTER,
            FlexAlign::SpaceEvenly => neo_lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_SPACE_EVENLY,
            FlexAlign::SpaceAround => neo_lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_SPACE_AROUND,
            FlexAlign::SpaceBetween => neo_lvgl_sys::lv_flex_align_t_LV_FLEX_ALIGN_SPACE_BETWEEN,
        }
    }
}

/// Grid alignment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridAlign {
    /// Align to the start
    Start,
    /// Align to the center
    Center,
    /// Align to the end
    End,
    /// Stretch to fill the cell
    Stretch,
    /// Distribute with equal space around items
    SpaceEvenly,
    /// Distribute with space around items
    SpaceAround,
    /// Distribute with space between items
    SpaceBetween,
}

impl GridAlign {
    pub(crate) fn to_raw(self) -> neo_lvgl_sys::lv_grid_align_t {
        match self {
            GridAlign::Start => neo_lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_START,
            GridAlign::Center => neo_lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_CENTER,
            GridAlign::End => neo_lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_END,
            GridAlign::Stretch => neo_lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
            GridAlign::SpaceEvenly => neo_lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_SPACE_EVENLY,
            GridAlign::SpaceAround => neo_lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_SPACE_AROUND,
            GridAlign::SpaceBetween => neo_lvgl_sys::lv_grid_align_t_LV_GRID_ALIGN_SPACE_BETWEEN,
        }
    }
}

bitflags! {
    /// Direction flags for scrolling and other directional operations
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Direction: u8 {
        /// No direction
        const NONE = 0;
        /// Left direction
        const LEFT = 1;
        /// Right direction
        const RIGHT = 2;
        /// Top direction
        const TOP = 4;
        /// Bottom direction
        const BOTTOM = 8;
        /// Horizontal (left + right)
        const HOR = 3;
        /// Vertical (top + bottom)
        const VER = 12;
        /// All directions
        const ALL = 15;
    }
}

impl Direction {
    pub(crate) fn to_raw(self) -> neo_lvgl_sys::lv_dir_t {
        self.bits() as neo_lvgl_sys::lv_dir_t
    }

    pub(crate) fn from_raw(raw: neo_lvgl_sys::lv_dir_t) -> Self {
        Self::from_bits_truncate(raw as u8)
    }
}

/// Grid fractional unit
///
/// Use this to specify proportional sizes in grid layouts.
/// For example, `grid_fr(1)` means 1 fractional unit.
pub fn grid_fr(x: u8) -> i32 {
    unsafe { neo_lvgl_sys::lv_grid_fr(x) }
}

/// Marker for the end of grid template arrays
pub const GRID_TEMPLATE_LAST: i32 = neo_lvgl_sys::LV_GRID_TEMPLATE_LAST as i32;

/// Grid content size - the row/column will be sized to fit its content
pub const GRID_CONTENT: i32 = neo_lvgl_sys::LV_GRID_CONTENT as i32;

/// Layout extension trait for widgets
///
/// Provides flexbox and grid layout methods for any widget.
pub trait LayoutExt<'a>: crate::widgets::Widget<'a> {
    // === Flexbox methods ===

    /// Set the flex flow direction for this container
    fn set_flex_flow(&self, flow: FlexFlow) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_flex_flow(self.raw(), flow.to_raw());
        }
    }

    /// Set flex alignment for this container
    ///
    /// # Arguments
    ///
    /// * `main_place` - Alignment along the main axis (direction of flow)
    /// * `cross_place` - Alignment along the cross axis (perpendicular to flow)
    /// * `track_cross_place` - Alignment of tracks (rows/columns) along cross axis
    fn set_flex_align(
        &self,
        main_place: FlexAlign,
        cross_place: FlexAlign,
        track_cross_place: FlexAlign,
    ) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_flex_align(
                self.raw(),
                main_place.to_raw(),
                cross_place.to_raw(),
                track_cross_place.to_raw(),
            );
        }
    }

    /// Set the flex grow factor for this widget
    ///
    /// Widgets with higher grow values will take more available space.
    /// A value of 0 means the widget won't grow.
    fn set_flex_grow(&self, grow: u8) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_flex_grow(self.raw(), grow);
        }
    }

    // === Grid methods ===

    /// Set the grid template for this container
    ///
    /// # Arguments
    ///
    /// * `col_dsc` - Column descriptors (sizes). Must end with GRID_TEMPLATE_LAST.
    /// * `row_dsc` - Row descriptors (sizes). Must end with GRID_TEMPLATE_LAST.
    ///
    /// # Safety
    ///
    /// The arrays must remain valid for the lifetime of the object.
    /// Use static arrays or ensure the arrays outlive the widget.
    ///
    /// # Example
    ///
    /// ```ignore
    /// static COL_DSC: [i32; 4] = [grid_fr(1), grid_fr(1), grid_fr(1), GRID_TEMPLATE_LAST];
    /// static ROW_DSC: [i32; 3] = [GRID_CONTENT, GRID_CONTENT, GRID_TEMPLATE_LAST];
    /// container.set_grid_dsc_array(&COL_DSC, &ROW_DSC);
    /// ```
    fn set_grid_dsc_array(&self, col_dsc: &'static [i32], row_dsc: &'static [i32]) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_grid_dsc_array(
                self.raw(),
                col_dsc.as_ptr(),
                row_dsc.as_ptr(),
            );
        }
    }

    /// Set grid alignment for the container
    fn set_grid_align(&self, column_align: GridAlign, row_align: GridAlign) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_grid_align(
                self.raw(),
                column_align.to_raw(),
                row_align.to_raw(),
            );
        }
    }

    /// Place this widget in a grid cell
    ///
    /// # Arguments
    ///
    /// * `col_align` - Horizontal alignment within the cell
    /// * `col_pos` - Column position (0-based)
    /// * `col_span` - Number of columns to span
    /// * `row_align` - Vertical alignment within the cell
    /// * `row_pos` - Row position (0-based)
    /// * `row_span` - Number of rows to span
    fn set_grid_cell(
        &self,
        col_align: GridAlign,
        col_pos: u8,
        col_span: u8,
        row_align: GridAlign,
        row_pos: u8,
        row_span: u8,
    ) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_grid_cell(
                self.raw(),
                col_align.to_raw(),
                col_pos as i32,
                col_span as i32,
                row_align.to_raw(),
                row_pos as i32,
                row_span as i32,
            );
        }
    }

    /// Update the layout of this widget and its children
    ///
    /// Call this after modifying layout properties to force an immediate recalculation.
    fn update_layout(&self) {
        unsafe {
            neo_lvgl_sys::lv_obj_update_layout(self.raw());
        }
    }
}

// Implement LayoutExt for all Widget types
impl<'a, T: crate::widgets::Widget<'a>> LayoutExt<'a> for T {}

/// Initialize the flex layout system
///
/// This is called automatically by LVGL during init, but can be called
/// explicitly if needed.
pub fn flex_init() {
    unsafe {
        neo_lvgl_sys::lv_flex_init();
    }
}

/// Initialize the grid layout system
///
/// This is called automatically by LVGL during init, but can be called
/// explicitly if needed.
pub fn grid_init() {
    unsafe {
        neo_lvgl_sys::lv_grid_init();
    }
}

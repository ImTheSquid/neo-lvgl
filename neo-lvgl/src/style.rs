//! Style system for widgets
//!
//! LVGL styles allow customizing the appearance of widgets.
//! Styles can be applied to different parts and states of widgets.

use crate::color::{Color, Opacity};
use bitflags::bitflags;
use core::mem::MaybeUninit;

/// LVGL style
///
/// Styles define visual properties that can be applied to widgets.
/// A style can be shared between multiple widgets.
pub struct Style {
    raw: neo_lvgl_sys::lv_style_t,
    initialized: bool,
}

impl Style {
    /// Create a new uninitialized style.
    ///
    /// Call methods on the style to initialize it automatically.
    pub const fn new() -> Self {
        Self {
            raw: unsafe { MaybeUninit::zeroed().assume_init() },
            initialized: false,
        }
    }

    /// Ensure the style is initialized
    fn ensure_init(&mut self) {
        if !self.initialized {
            unsafe {
                neo_lvgl_sys::lv_style_init(&mut self.raw);
            }
            self.initialized = true;
        }
    }

    /// Get raw pointer to the style
    #[inline]
    pub fn raw(&self) -> *const neo_lvgl_sys::lv_style_t {
        &self.raw
    }

    /// Get mutable raw pointer to the style
    #[inline]
    pub fn raw_mut(&mut self) -> *mut neo_lvgl_sys::lv_style_t {
        &mut self.raw
    }

    // Background properties

    /// Set background color
    pub fn set_bg_color(&mut self, color: Color) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_bg_color(&mut self.raw, color.raw());
        }
    }

    /// Set background opacity
    pub fn set_bg_opa(&mut self, opa: Opacity) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_bg_opa(&mut self.raw, opa.raw());
        }
    }

    /// Set background gradient color
    pub fn set_bg_grad_color(&mut self, color: Color) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_bg_grad_color(&mut self.raw, color.raw());
        }
    }

    /// Set background gradient direction
    pub fn set_bg_grad_dir(&mut self, dir: GradDir) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_bg_grad_dir(&mut self.raw, dir.to_raw());
        }
    }

    // Border properties

    /// Set border color
    pub fn set_border_color(&mut self, color: Color) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_border_color(&mut self.raw, color.raw());
        }
    }

    /// Set border opacity
    pub fn set_border_opa(&mut self, opa: Opacity) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_border_opa(&mut self.raw, opa.raw());
        }
    }

    /// Set border width
    pub fn set_border_width(&mut self, width: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_border_width(&mut self.raw, width);
        }
    }

    /// Set which sides have borders
    pub fn set_border_side(&mut self, side: BorderSide) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_border_side(
                &mut self.raw,
                side.bits() as neo_lvgl_sys::lv_border_side_t,
            );
        }
    }

    // Corner radius

    /// Set corner radius
    pub fn set_radius(&mut self, radius: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_radius(&mut self.raw, radius);
        }
    }

    // Padding

    /// Set all padding values
    pub fn set_pad_all(&mut self, pad: i32) {
        self.set_pad_top(pad);
        self.set_pad_bottom(pad);
        self.set_pad_left(pad);
        self.set_pad_right(pad);
    }

    /// Set horizontal padding (left and right)
    pub fn set_pad_hor(&mut self, pad: i32) {
        self.set_pad_left(pad);
        self.set_pad_right(pad);
    }

    /// Set vertical padding (top and bottom)
    pub fn set_pad_ver(&mut self, pad: i32) {
        self.set_pad_top(pad);
        self.set_pad_bottom(pad);
    }

    /// Set top padding
    pub fn set_pad_top(&mut self, pad: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_pad_top(&mut self.raw, pad);
        }
    }

    /// Set bottom padding
    pub fn set_pad_bottom(&mut self, pad: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_pad_bottom(&mut self.raw, pad);
        }
    }

    /// Set left padding
    pub fn set_pad_left(&mut self, pad: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_pad_left(&mut self.raw, pad);
        }
    }

    /// Set right padding
    pub fn set_pad_right(&mut self, pad: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_pad_right(&mut self.raw, pad);
        }
    }

    /// Set padding between rows (for flex/grid layouts)
    pub fn set_pad_row(&mut self, pad: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_pad_row(&mut self.raw, pad);
        }
    }

    /// Set padding between columns (for flex/grid layouts)
    pub fn set_pad_column(&mut self, pad: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_pad_column(&mut self.raw, pad);
        }
    }

    // Size

    /// Set width
    pub fn set_width(&mut self, width: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_width(&mut self.raw, width);
        }
    }

    /// Set minimum width
    pub fn set_min_width(&mut self, width: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_min_width(&mut self.raw, width);
        }
    }

    /// Set maximum width
    pub fn set_max_width(&mut self, width: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_max_width(&mut self.raw, width);
        }
    }

    /// Set height
    pub fn set_height(&mut self, height: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_height(&mut self.raw, height);
        }
    }

    /// Set minimum height
    pub fn set_min_height(&mut self, height: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_min_height(&mut self.raw, height);
        }
    }

    /// Set maximum height
    pub fn set_max_height(&mut self, height: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_max_height(&mut self.raw, height);
        }
    }

    // Text properties

    /// Set text color
    pub fn set_text_color(&mut self, color: Color) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_text_color(&mut self.raw, color.raw());
        }
    }

    /// Set text opacity
    pub fn set_text_opa(&mut self, opa: Opacity) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_text_opa(&mut self.raw, opa.raw());
        }
    }

    /// Set text letter spacing
    pub fn set_text_letter_space(&mut self, space: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_text_letter_space(&mut self.raw, space);
        }
    }

    /// Set text line spacing
    pub fn set_text_line_space(&mut self, space: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_text_line_space(&mut self.raw, space);
        }
    }

    /// Set text alignment
    pub fn set_text_align(&mut self, align: TextAlign) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_text_align(&mut self.raw, align.to_raw());
        }
    }

    // Outline

    /// Set outline color
    pub fn set_outline_color(&mut self, color: Color) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_outline_color(&mut self.raw, color.raw());
        }
    }

    /// Set outline opacity
    pub fn set_outline_opa(&mut self, opa: Opacity) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_outline_opa(&mut self.raw, opa.raw());
        }
    }

    /// Set outline width
    pub fn set_outline_width(&mut self, width: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_outline_width(&mut self.raw, width);
        }
    }

    /// Set outline padding (offset from object)
    pub fn set_outline_pad(&mut self, pad: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_outline_pad(&mut self.raw, pad);
        }
    }

    // Shadow

    /// Set shadow color
    pub fn set_shadow_color(&mut self, color: Color) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_shadow_color(&mut self.raw, color.raw());
        }
    }

    /// Set shadow opacity
    pub fn set_shadow_opa(&mut self, opa: Opacity) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_shadow_opa(&mut self.raw, opa.raw());
        }
    }

    /// Set shadow width (blur)
    pub fn set_shadow_width(&mut self, width: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_shadow_width(&mut self.raw, width);
        }
    }

    /// Set shadow X offset
    pub fn set_shadow_offset_x(&mut self, offset: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_shadow_offset_x(&mut self.raw, offset);
        }
    }

    /// Set shadow Y offset
    pub fn set_shadow_offset_y(&mut self, offset: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_shadow_offset_y(&mut self.raw, offset);
        }
    }

    /// Set shadow spread
    pub fn set_shadow_spread(&mut self, spread: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_shadow_spread(&mut self.raw, spread);
        }
    }

    // Transform

    /// Set rotation angle (0.1 degree units, e.g., 450 = 45 degrees)
    pub fn set_transform_rotation(&mut self, angle: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_transform_rotation(&mut self.raw, angle);
        }
    }

    /// Set horizontal scale (256 = 100%)
    pub fn set_transform_scale_x(&mut self, scale: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_transform_scale_x(&mut self.raw, scale);
        }
    }

    /// Set vertical scale (256 = 100%)
    pub fn set_transform_scale_y(&mut self, scale: i32) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_transform_scale_y(&mut self.raw, scale);
        }
    }

    // Opacity

    /// Set overall opacity
    pub fn set_opa(&mut self, opa: Opacity) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_opa(&mut self.raw, opa.raw());
        }
    }

    // Layout

    /// Set flex flow direction
    pub fn set_flex_flow(&mut self, flow: FlexFlow) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_flex_flow(&mut self.raw, flow.to_raw());
        }
    }

    /// Set flex grow factor
    pub fn set_flex_grow(&mut self, grow: u8) {
        self.ensure_init();
        unsafe {
            neo_lvgl_sys::lv_style_set_flex_grow(&mut self.raw, grow);
        }
    }

    /// Reset the style to default values
    pub fn reset(&mut self) {
        if self.initialized {
            unsafe {
                neo_lvgl_sys::lv_style_reset(&mut self.raw);
            }
        }
        self.initialized = false;
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Style {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                neo_lvgl_sys::lv_style_reset(&mut self.raw);
            }
        }
    }
}

bitflags! {
    /// Style selector for specifying widget parts and states
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct StyleSelector: u32 {
        // Parts
        const MAIN = neo_lvgl_sys::lv_part_t_LV_PART_MAIN;
        const SCROLLBAR = neo_lvgl_sys::lv_part_t_LV_PART_SCROLLBAR;
        const INDICATOR = neo_lvgl_sys::lv_part_t_LV_PART_INDICATOR;
        const KNOB = neo_lvgl_sys::lv_part_t_LV_PART_KNOB;
        const SELECTED = neo_lvgl_sys::lv_part_t_LV_PART_SELECTED;
        const ITEMS = neo_lvgl_sys::lv_part_t_LV_PART_ITEMS;
        const CURSOR = neo_lvgl_sys::lv_part_t_LV_PART_CURSOR;

        // States
        const DEFAULT = neo_lvgl_sys::lv_state_t_LV_STATE_DEFAULT as u32;
        const CHECKED = neo_lvgl_sys::lv_state_t_LV_STATE_CHECKED as u32;
        const FOCUSED = neo_lvgl_sys::lv_state_t_LV_STATE_FOCUSED as u32;
        const FOCUS_KEY = neo_lvgl_sys::lv_state_t_LV_STATE_FOCUS_KEY as u32;
        const EDITED = neo_lvgl_sys::lv_state_t_LV_STATE_EDITED as u32;
        const HOVERED = neo_lvgl_sys::lv_state_t_LV_STATE_HOVERED as u32;
        const PRESSED = neo_lvgl_sys::lv_state_t_LV_STATE_PRESSED as u32;
        const SCROLLED = neo_lvgl_sys::lv_state_t_LV_STATE_SCROLLED as u32;
        const DISABLED = neo_lvgl_sys::lv_state_t_LV_STATE_DISABLED as u32;
    }
}

impl Default for StyleSelector {
    fn default() -> Self {
        Self::MAIN | Self::DEFAULT
    }
}

/// Gradient direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GradDir {
    /// No gradient
    None,
    /// Horizontal gradient
    Horizontal,
    /// Vertical gradient
    Vertical,
}

impl GradDir {
    fn to_raw(self) -> neo_lvgl_sys::lv_grad_dir_t {
        match self {
            GradDir::None => neo_lvgl_sys::lv_grad_dir_t_LV_GRAD_DIR_NONE,
            GradDir::Horizontal => neo_lvgl_sys::lv_grad_dir_t_LV_GRAD_DIR_HOR,
            GradDir::Vertical => neo_lvgl_sys::lv_grad_dir_t_LV_GRAD_DIR_VER,
        }
    }
}

bitflags! {
    /// Border sides
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct BorderSide: u8 {
        const NONE = neo_lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_NONE as u8;
        const BOTTOM = neo_lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_BOTTOM as u8;
        const TOP = neo_lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_TOP as u8;
        const LEFT = neo_lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_LEFT as u8;
        const RIGHT = neo_lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_RIGHT as u8;
        const FULL = neo_lvgl_sys::lv_border_side_t_LV_BORDER_SIDE_FULL as u8;
    }
}

/// Text alignment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlign {
    Auto,
    Left,
    Center,
    Right,
}

impl TextAlign {
    fn to_raw(self) -> neo_lvgl_sys::lv_text_align_t {
        match self {
            TextAlign::Auto => neo_lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_AUTO,
            TextAlign::Left => neo_lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_LEFT,
            TextAlign::Center => neo_lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_CENTER,
            TextAlign::Right => neo_lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_RIGHT,
        }
    }
}

/// Flex layout flow direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexFlow {
    Row,
    Column,
    RowWrap,
    RowReverse,
    RowWrapReverse,
    ColumnWrap,
    ColumnReverse,
    ColumnWrapReverse,
}

impl FlexFlow {
    fn to_raw(self) -> neo_lvgl_sys::lv_flex_flow_t {
        match self {
            FlexFlow::Row => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW,
            FlexFlow::Column => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN,
            FlexFlow::RowWrap => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW_WRAP,
            FlexFlow::RowReverse => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW_REVERSE,
            FlexFlow::RowWrapReverse => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_ROW_WRAP_REVERSE,
            FlexFlow::ColumnWrap => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_WRAP,
            FlexFlow::ColumnReverse => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_REVERSE,
            FlexFlow::ColumnWrapReverse => neo_lvgl_sys::lv_flex_flow_t_LV_FLEX_FLOW_COLUMN_WRAP_REVERSE,
        }
    }
}

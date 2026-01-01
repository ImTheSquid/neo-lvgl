//! Widget types and the Widget trait
//!
//! All LVGL UI elements are widgets. This module provides:
//!
//! - `Obj` - The base widget type
//! - `Widget` - Trait implemented by all widgets
//! - `Screen` - The root widget for a display
//! - Specific widget types: `Button`, `Label`, etc.

mod button;
mod label;

pub use button::Button;
pub use label::Label;

use crate::event::EventHandler;
use crate::style::{Style, StyleSelector};
use core::marker::PhantomData;
use core::ptr::NonNull;

/// Special size value that makes the widget fit its content
pub const SIZE_CONTENT: i32 = neo_lvgl_sys::LV_COORD_MAX as i32 | (1 << 29);

/// Base widget type
///
/// `Obj` is a non-owning reference to an LVGL object.
/// The lifetime parameter ensures the widget reference doesn't outlive its parent.
///
/// # Ownership
///
/// LVGL manages widget memory internally. When a parent widget is deleted,
/// all its children are automatically deleted. Rust lifetimes help prevent
/// dangling references, but users must be careful with dynamic widget deletion.
pub struct Obj<'a> {
    raw: NonNull<neo_lvgl_sys::lv_obj_t>,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> Obj<'a> {
    /// Create from raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be valid and the object must live at least as long as 'a.
    pub unsafe fn from_raw(ptr: *mut neo_lvgl_sys::lv_obj_t) -> Option<Self> {
        NonNull::new(ptr).map(|raw| Self {
            raw,
            _lifetime: PhantomData,
        })
    }

    /// Get the raw pointer
    #[inline]
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.raw.as_ptr()
    }

    /// Create a new generic object as a child of this one
    pub fn create_child(&'a self) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_obj_create(self.raw.as_ptr());
            Self::from_raw(ptr)
        }
    }
}

impl<'a> Widget<'a> for Obj<'a> {
    fn obj(&self) -> &Obj<'a> {
        self
    }
}

impl EventHandler for Obj<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.raw.as_ptr()
    }
}

/// Trait implemented by all widgets
///
/// Provides common operations like positioning, sizing, and styling.
pub trait Widget<'a>: EventHandler {
    /// Get the underlying Obj reference
    fn obj(&self) -> &Obj<'a>;

    /// Get the raw object pointer
    fn raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj().raw()
    }

    // Positioning

    /// Set widget position
    fn set_pos(&self, x: i32, y: i32) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_pos(self.raw(), x, y);
        }
    }

    /// Set X position
    fn set_x(&self, x: i32) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_x(self.raw(), x);
        }
    }

    /// Set Y position
    fn set_y(&self, y: i32) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_y(self.raw(), y);
        }
    }

    /// Get X position
    fn x(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_obj_get_x(self.raw()) }
    }

    /// Get Y position
    fn y(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_obj_get_y(self.raw()) }
    }

    // Sizing

    /// Set widget size
    fn set_size(&self, width: i32, height: i32) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_size(self.raw(), width, height);
        }
    }

    /// Set widget width
    fn set_width(&self, width: i32) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_width(self.raw(), width);
        }
    }

    /// Set widget height
    fn set_height(&self, height: i32) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_height(self.raw(), height);
        }
    }

    /// Get widget width
    fn width(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_obj_get_width(self.raw()) }
    }

    /// Get widget height
    fn height(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_obj_get_height(self.raw()) }
    }

    /// Set content width (fits content)
    fn set_content_width(&self) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_width(self.raw(), SIZE_CONTENT);
        }
    }

    /// Set content height (fits content)
    fn set_content_height(&self) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_height(self.raw(), SIZE_CONTENT);
        }
    }

    // Alignment

    /// Center the widget in its parent
    fn center(&self) {
        unsafe {
            neo_lvgl_sys::lv_obj_center(self.raw());
        }
    }

    /// Align widget relative to parent
    fn align(&self, align: Align, x_offset: i32, y_offset: i32) {
        unsafe {
            neo_lvgl_sys::lv_obj_align(self.raw(), align.to_raw(), x_offset, y_offset);
        }
    }

    /// Align widget relative to another widget
    fn align_to(&self, other: &impl Widget<'a>, align: Align, x_offset: i32, y_offset: i32) {
        unsafe {
            neo_lvgl_sys::lv_obj_align_to(self.raw(), other.raw(), align.to_raw(), x_offset, y_offset);
        }
    }

    // Styling

    /// Add a style to this widget
    fn add_style(&self, style: &Style, selector: StyleSelector) {
        unsafe {
            neo_lvgl_sys::lv_obj_add_style(self.raw(), style.raw() as *mut _, selector.bits());
        }
    }

    /// Remove all styles from this widget
    fn remove_all_styles(&self) {
        unsafe {
            neo_lvgl_sys::lv_obj_remove_style_all(self.raw());
        }
    }

    /// Refresh the style (call after modifying a shared style)
    fn refresh_style(&self) {
        unsafe {
            neo_lvgl_sys::lv_obj_refresh_style(
                self.raw(),
                neo_lvgl_sys::lv_part_t_LV_PART_ANY,
                neo_lvgl_sys::_lv_style_id_t_LV_STYLE_PROP_ANY as u8,
            );
        }
    }

    // Visibility and state

    /// Add a state flag
    fn add_state(&self, state: State) {
        unsafe {
            neo_lvgl_sys::lv_obj_add_state(self.raw(), state.bits());
        }
    }

    /// Remove a state flag
    fn remove_state(&self, state: State) {
        unsafe {
            neo_lvgl_sys::lv_obj_remove_state(self.raw(), state.bits());
        }
    }

    /// Check if a state is active
    fn has_state(&self, state: State) -> bool {
        unsafe { neo_lvgl_sys::lv_obj_has_state(self.raw(), state.bits()) }
    }

    /// Add a flag
    fn add_flag(&self, flag: Flag) {
        unsafe {
            neo_lvgl_sys::lv_obj_add_flag(self.raw(), flag.bits());
        }
    }

    /// Remove a flag
    fn remove_flag(&self, flag: Flag) {
        unsafe {
            neo_lvgl_sys::lv_obj_remove_flag(self.raw(), flag.bits());
        }
    }

    /// Check if a flag is set
    fn has_flag(&self, flag: Flag) -> bool {
        unsafe { neo_lvgl_sys::lv_obj_has_flag(self.raw(), flag.bits()) }
    }

    /// Set widget visibility
    fn set_hidden(&self, hidden: bool) {
        if hidden {
            self.add_flag(Flag::HIDDEN);
        } else {
            self.remove_flag(Flag::HIDDEN);
        }
    }

    /// Check if widget is hidden
    fn is_hidden(&self) -> bool {
        self.has_flag(Flag::HIDDEN)
    }

    /// Enable or disable the widget
    fn set_disabled(&self, disabled: bool) {
        if disabled {
            self.add_state(State::DISABLED);
        } else {
            self.remove_state(State::DISABLED);
        }
    }

    /// Check if widget is disabled
    fn is_disabled(&self) -> bool {
        self.has_state(State::DISABLED)
    }

    // Scrolling

    /// Scroll to a specific position
    fn scroll_to(&self, x: i32, y: i32, animate: bool) {
        unsafe {
            neo_lvgl_sys::lv_obj_scroll_to(self.raw(), x, y, animate);
        }
    }

    /// Invalidate the widget (trigger redraw)
    fn invalidate(&self) {
        unsafe {
            neo_lvgl_sys::lv_obj_invalidate(self.raw());
        }
    }

    /// Delete the widget
    ///
    /// # Safety
    ///
    /// After calling this, any references to this widget or its children become invalid.
    unsafe fn delete(self)
    where
        Self: Sized,
    {
        neo_lvgl_sys::lv_obj_delete(self.raw());
    }
}

/// Screen widget (root of the widget tree)
pub struct Screen<'a> {
    obj: Obj<'a>,
}

impl<'a> Screen<'a> {
    /// Create from raw pointer (internal use)
    pub(crate) fn from_raw(ptr: *mut neo_lvgl_sys::lv_obj_t) -> Self {
        Self {
            obj: unsafe { Obj::from_raw(ptr).expect("Screen pointer must be valid") },
        }
    }

    /// Create a button on this screen
    pub fn create_button(&'a self) -> Option<Button<'a>> {
        Button::new(self)
    }

    /// Create a label on this screen
    pub fn create_label(&'a self) -> Option<Label<'a>> {
        Label::new(self)
    }

    /// Load this screen (make it active)
    pub fn load(&self) {
        unsafe {
            neo_lvgl_sys::lv_screen_load(self.obj.raw());
        }
    }

    /// Load this screen with animation
    pub fn load_anim(&self, anim: ScreenLoadAnim, time_ms: u32, delay_ms: u32, auto_delete: bool) {
        unsafe {
            neo_lvgl_sys::lv_screen_load_anim(
                self.obj.raw(),
                anim.to_raw(),
                time_ms,
                delay_ms,
                auto_delete,
            );
        }
    }
}

impl<'a> Widget<'a> for Screen<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Screen<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}

/// Alignment options
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Default,
    TopLeft,
    TopMid,
    TopRight,
    BottomLeft,
    BottomMid,
    BottomRight,
    LeftMid,
    RightMid,
    Center,
    OutTopLeft,
    OutTopMid,
    OutTopRight,
    OutBottomLeft,
    OutBottomMid,
    OutBottomRight,
    OutLeftTop,
    OutLeftMid,
    OutLeftBottom,
    OutRightTop,
    OutRightMid,
    OutRightBottom,
}

impl Align {
    fn to_raw(self) -> neo_lvgl_sys::lv_align_t {
        match self {
            Align::Default => neo_lvgl_sys::lv_align_t_LV_ALIGN_DEFAULT,
            Align::TopLeft => neo_lvgl_sys::lv_align_t_LV_ALIGN_TOP_LEFT,
            Align::TopMid => neo_lvgl_sys::lv_align_t_LV_ALIGN_TOP_MID,
            Align::TopRight => neo_lvgl_sys::lv_align_t_LV_ALIGN_TOP_RIGHT,
            Align::BottomLeft => neo_lvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_LEFT,
            Align::BottomMid => neo_lvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_MID,
            Align::BottomRight => neo_lvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_RIGHT,
            Align::LeftMid => neo_lvgl_sys::lv_align_t_LV_ALIGN_LEFT_MID,
            Align::RightMid => neo_lvgl_sys::lv_align_t_LV_ALIGN_RIGHT_MID,
            Align::Center => neo_lvgl_sys::lv_align_t_LV_ALIGN_CENTER,
            Align::OutTopLeft => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_LEFT,
            Align::OutTopMid => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_MID,
            Align::OutTopRight => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_RIGHT,
            Align::OutBottomLeft => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_LEFT,
            Align::OutBottomMid => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_MID,
            Align::OutBottomRight => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_RIGHT,
            Align::OutLeftTop => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_TOP,
            Align::OutLeftMid => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_MID,
            Align::OutLeftBottom => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_BOTTOM,
            Align::OutRightTop => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_TOP,
            Align::OutRightMid => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_MID,
            Align::OutRightBottom => neo_lvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_BOTTOM,
        }
    }
}

use bitflags::bitflags;

bitflags! {
    /// Widget state flags
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct State: u32 {
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

bitflags! {
    /// Widget flags
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Flag: u32 {
        const HIDDEN = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_HIDDEN;
        const CLICKABLE = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_CLICKABLE;
        const CLICK_FOCUSABLE = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_CLICK_FOCUSABLE;
        const CHECKABLE = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_CHECKABLE;
        const SCROLLABLE = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLLABLE;
        const SCROLL_ELASTIC = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_ELASTIC;
        const SCROLL_MOMENTUM = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_MOMENTUM;
        const SCROLL_ONE = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_ONE;
        const SCROLL_CHAIN_HOR = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_CHAIN_HOR;
        const SCROLL_CHAIN_VER = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_CHAIN_VER;
        const SCROLL_ON_FOCUS = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_ON_FOCUS;
        const SCROLL_WITH_ARROW = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SCROLL_WITH_ARROW;
        const SNAPPABLE = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SNAPPABLE;
        const PRESS_LOCK = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_PRESS_LOCK;
        const EVENT_BUBBLE = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_EVENT_BUBBLE;
        const GESTURE_BUBBLE = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_GESTURE_BUBBLE;
        const ADV_HITTEST = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_ADV_HITTEST;
        const IGNORE_LAYOUT = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_IGNORE_LAYOUT;
        const FLOATING = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_FLOATING;
        const SEND_DRAW_TASK_EVENTS = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_SEND_DRAW_TASK_EVENTS;
        const OVERFLOW_VISIBLE = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_OVERFLOW_VISIBLE;
        const FLEX_IN_NEW_TRACK = neo_lvgl_sys::lv_obj_flag_t_LV_OBJ_FLAG_FLEX_IN_NEW_TRACK;
    }
}

/// Screen load animation types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenLoadAnim {
    None,
    OverLeft,
    OverRight,
    OverTop,
    OverBottom,
    MoveLeft,
    MoveRight,
    MoveTop,
    MoveBottom,
    FadeIn,
    FadeOut,
    OutLeft,
    OutRight,
    OutTop,
    OutBottom,
}

impl ScreenLoadAnim {
    fn to_raw(self) -> neo_lvgl_sys::lv_screen_load_anim_t {
        match self {
            ScreenLoadAnim::None => neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_NONE,
            ScreenLoadAnim::OverLeft => {
                neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_OVER_LEFT
            }
            ScreenLoadAnim::OverRight => {
                neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_OVER_RIGHT
            }
            ScreenLoadAnim::OverTop => neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_OVER_TOP,
            ScreenLoadAnim::OverBottom => {
                neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_OVER_BOTTOM
            }
            ScreenLoadAnim::MoveLeft => {
                neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_MOVE_LEFT
            }
            ScreenLoadAnim::MoveRight => {
                neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_MOVE_RIGHT
            }
            ScreenLoadAnim::MoveTop => neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_MOVE_TOP,
            ScreenLoadAnim::MoveBottom => {
                neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_MOVE_BOTTOM
            }
            ScreenLoadAnim::FadeIn => neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_FADE_IN,
            ScreenLoadAnim::FadeOut => neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_FADE_OUT,
            ScreenLoadAnim::OutLeft => neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_OUT_LEFT,
            ScreenLoadAnim::OutRight => {
                neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_OUT_RIGHT
            }
            ScreenLoadAnim::OutTop => neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_OUT_TOP,
            ScreenLoadAnim::OutBottom => {
                neo_lvgl_sys::lv_screen_load_anim_t_LV_SCREEN_LOAD_ANIM_OUT_BOTTOM
            }
        }
    }
}

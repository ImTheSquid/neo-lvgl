//! Span widget for rich text

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};
use core::ffi::CStr;
use core::ptr::NonNull;

/// Span overflow mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanOverflow {
    /// Text can overflow the widget bounds
    Clip,
    /// Add ellipsis (...) when text overflows
    Ellipsis,
}

impl SpanOverflow {
    fn to_raw(self) -> neo_lvgl_sys::lv_span_overflow_t {
        match self {
            SpanOverflow::Clip => neo_lvgl_sys::lv_span_overflow_t_LV_SPAN_OVERFLOW_CLIP,
            SpanOverflow::Ellipsis => neo_lvgl_sys::lv_span_overflow_t_LV_SPAN_OVERFLOW_ELLIPSIS,
        }
    }
}

/// Span mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanMode {
    /// Fixed size
    Fixed,
    /// Expand to fit content
    Expand,
    /// Break text into lines
    Break,
}

impl SpanMode {
    fn to_raw(self) -> neo_lvgl_sys::lv_span_mode_t {
        match self {
            SpanMode::Fixed => neo_lvgl_sys::lv_span_mode_t_LV_SPAN_MODE_FIXED,
            SpanMode::Expand => neo_lvgl_sys::lv_span_mode_t_LV_SPAN_MODE_EXPAND,
            SpanMode::Break => neo_lvgl_sys::lv_span_mode_t_LV_SPAN_MODE_BREAK,
        }
    }
}

/// A text span with its own style
pub struct Span {
    raw: NonNull<neo_lvgl_sys::lv_span_t>,
}

impl Span {
    /// Create from raw pointer
    unsafe fn from_raw(ptr: *mut neo_lvgl_sys::lv_span_t) -> Option<Self> {
        NonNull::new(ptr).map(|raw| Self { raw })
    }

    /// Get the raw pointer
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_span_t {
        self.raw.as_ptr()
    }

    /// Set the text for this span
    pub fn set_text(&self, text: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_span_set_text(self.raw.as_ptr(), text.as_ptr());
        }
    }

    /// Set static text for this span (not copied)
    pub fn set_text_static(&self, text: &'static CStr) {
        unsafe {
            neo_lvgl_sys::lv_span_set_text_static(self.raw.as_ptr(), text.as_ptr());
        }
    }

    /// Get the style for this span
    pub fn style(&self) -> &mut neo_lvgl_sys::lv_style_t {
        unsafe { &mut *neo_lvgl_sys::lv_span_get_style(self.raw.as_ptr()) }
    }
}

/// SpanGroup widget
///
/// A container for multiple text spans with different styles.
///
/// # Example
///
/// ```ignore
/// let spangroup = SpanGroup::new(&screen).unwrap();
/// spangroup.set_mode(SpanMode::Break);
///
/// let span1 = spangroup.add_span().unwrap();
/// span1.set_text(c"Hello ");
///
/// let span2 = spangroup.add_span().unwrap();
/// span2.set_text(c"World!");
/// // Style span2 differently using span2.style()
///
/// spangroup.refr_mode();
/// ```
#[derive(Clone, Copy)]
pub struct SpanGroup<'a> {
    obj: Obj<'a>,
}

impl<'a> SpanGroup<'a> {
    /// Create a new span group as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_spangroup_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Add a new span
    pub fn add_span(&self) -> Option<Span> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_spangroup_add_span(self.obj.raw());
            Span::from_raw(ptr)
        }
    }

    /// Delete a span
    pub fn delete_span(&self, span: Span) {
        unsafe {
            neo_lvgl_sys::lv_spangroup_delete_span(self.obj.raw(), span.raw());
        }
    }

    /// Set the text alignment
    pub fn set_align(&self, align: u8) {
        unsafe {
            neo_lvgl_sys::lv_spangroup_set_align(self.obj.raw(), align as neo_lvgl_sys::lv_text_align_t);
        }
    }

    /// Set the overflow mode
    pub fn set_overflow(&self, overflow: SpanOverflow) {
        unsafe {
            neo_lvgl_sys::lv_spangroup_set_overflow(self.obj.raw(), overflow.to_raw());
        }
    }

    /// Set the indent for the first line
    pub fn set_indent(&self, indent: i32) {
        unsafe {
            neo_lvgl_sys::lv_spangroup_set_indent(self.obj.raw(), indent);
        }
    }

    /// Set the span mode
    pub fn set_mode(&self, mode: SpanMode) {
        unsafe {
            neo_lvgl_sys::lv_spangroup_set_mode(self.obj.raw(), mode.to_raw());
        }
    }

    /// Set the maximum number of lines (-1 for no limit)
    pub fn set_max_lines(&self, lines: Option<i32>) {
        unsafe {
            neo_lvgl_sys::lv_spangroup_set_max_lines(self.obj.raw(), lines.unwrap_or(-1));
        }
    }

    /// Get a span by index
    pub fn span(&self, index: u32) -> Option<Span> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_spangroup_get_child(self.obj.raw(), index as i32);
            Span::from_raw(ptr)
        }
    }

    /// Get the number of spans
    pub fn span_count(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_spangroup_get_span_count(self.obj.raw()) }
    }

    /// Refresh the layout after changes
    ///
    /// Call `invalidate()` instead in LVGL 9 to trigger a redraw.
    pub fn refr_mode(&self) {
        // In LVGL 9, use invalidate() to refresh
        unsafe {
            neo_lvgl_sys::lv_obj_invalidate(self.obj.raw());
        }
    }
}

impl<'a> Widget<'a> for SpanGroup<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for SpanGroup<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



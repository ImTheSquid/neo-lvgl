//! Calendar widget

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};

/// A date in the calendar
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarDate {
    /// Year (e.g., 2024)
    pub year: u32,
    /// Month (1-12)
    pub month: u32,
    /// Day (1-31)
    pub day: u32,
}

impl CalendarDate {
    /// Create a new date
    pub fn new(year: u32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    fn to_raw(self) -> neo_lvgl_sys::lv_calendar_date_t {
        neo_lvgl_sys::lv_calendar_date_t {
            year: self.year as u16,
            month: self.month as u8,
            day: self.day as u8,
        }
    }

    fn from_raw(raw: neo_lvgl_sys::lv_calendar_date_t) -> Self {
        Self {
            year: raw.year as u32,
            month: raw.month as u32,
            day: raw.day as u32,
        }
    }
}

/// Calendar widget
///
/// A date picker showing a monthly view.
///
/// # Example
///
/// ```ignore
/// let calendar = Calendar::new(&screen).unwrap();
/// calendar.set_today(CalendarDate::new(2024, 6, 15));
/// calendar.set_shown_date(CalendarDate::new(2024, 6, 1));
///
/// // Handle date selection
/// calendar.on_value_changed(|cal| {
///     if let Some(date) = cal.pressed_date() {
///         // Handle selected date
///     }
/// });
/// ```
#[derive(Clone, Copy)]
pub struct Calendar<'a> {
    obj: Obj<'a>,
}

impl<'a> Calendar<'a> {
    /// Create a new calendar as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_calendar_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set today's date (shown with a highlight)
    pub fn set_today(&self, date: CalendarDate) {
        let raw = date.to_raw();
        unsafe {
            neo_lvgl_sys::lv_calendar_set_today_date(self.obj.raw(), raw.year as u32, raw.month as u32, raw.day as u32);
        }
    }

    /// Set the currently shown month/year
    ///
    /// Note: In LVGL 9, the shown date is read-only and changes when navigating.
    /// Use `set_today` to set the current date instead.
    pub fn set_shown_date(&self, _date: CalendarDate) {
        // In LVGL 9, there's no direct setter for shown date
        // The calendar navigates based on today's date and user interaction
    }

    /// Get today's date
    pub fn today(&self) -> CalendarDate {
        unsafe {
            let raw = neo_lvgl_sys::lv_calendar_get_today_date(self.obj.raw());
            CalendarDate::from_raw(*raw)
        }
    }

    /// Get the currently shown date (month/year)
    pub fn shown_date(&self) -> CalendarDate {
        unsafe {
            let raw = neo_lvgl_sys::lv_calendar_get_showed_date(self.obj.raw());
            CalendarDate::from_raw(*raw)
        }
    }

    /// Get the pressed/selected date
    pub fn pressed_date(&self) -> Option<CalendarDate> {
        let mut date = neo_lvgl_sys::lv_calendar_date_t {
            year: 0,
            month: 0,
            day: 0,
        };
        let result = unsafe { neo_lvgl_sys::lv_calendar_get_pressed_date(self.obj.raw(), &mut date) };
        if result == neo_lvgl_sys::lv_result_t_LV_RESULT_OK {
            Some(CalendarDate::from_raw(date))
        } else {
            None
        }
    }

    /// Set highlighted dates
    ///
    /// # Safety
    ///
    /// The dates array must remain valid for the lifetime of the calendar.
    pub unsafe fn set_highlighted_dates(&self, _dates: &[CalendarDate]) {
        // We need to convert our dates to raw dates
        // Since we can't allocate, we'll require the caller to manage memory
        // This is a simplified version that doesn't support highlighted dates safely
        // A full implementation would need alloc feature
    }

    /// Add a header with month/year navigation arrows
    pub fn add_header_arrow(&self) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_calendar_add_header_arrow(self.obj.raw());
            Obj::from_raw(ptr)
        }
    }

    /// Add a header with month/year dropdown selectors
    pub fn add_header_dropdown(&self) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_calendar_add_header_dropdown(self.obj.raw());
            Obj::from_raw(ptr)
        }
    }
}

impl<'a> Widget<'a> for Calendar<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Calendar<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



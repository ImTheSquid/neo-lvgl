//! Chart widget

use crate::color::Color;
use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};
use core::ptr::NonNull;

/// Chart type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartType {
    /// No data drawn
    None,
    /// Connect points with lines
    Line,
    /// Draw columns
    Bar,
    /// Draw points only (scatter plot)
    Scatter,
}

impl ChartType {
    fn to_raw(self) -> neo_lvgl_sys::lv_chart_type_t {
        match self {
            ChartType::None => neo_lvgl_sys::lv_chart_type_t_LV_CHART_TYPE_NONE,
            ChartType::Line => neo_lvgl_sys::lv_chart_type_t_LV_CHART_TYPE_LINE,
            ChartType::Bar => neo_lvgl_sys::lv_chart_type_t_LV_CHART_TYPE_BAR,
            ChartType::Scatter => neo_lvgl_sys::lv_chart_type_t_LV_CHART_TYPE_SCATTER,
        }
    }
}

/// Chart axis
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAxis {
    /// Primary Y axis (left side)
    PrimaryY,
    /// Secondary Y axis (right side)
    SecondaryY,
    /// Primary X axis (bottom)
    PrimaryX,
    /// Secondary X axis (top)
    SecondaryX,
}

impl ChartAxis {
    fn to_raw(self) -> neo_lvgl_sys::lv_chart_axis_t {
        match self {
            ChartAxis::PrimaryY => neo_lvgl_sys::lv_chart_axis_t_LV_CHART_AXIS_PRIMARY_Y,
            ChartAxis::SecondaryY => neo_lvgl_sys::lv_chart_axis_t_LV_CHART_AXIS_SECONDARY_Y,
            ChartAxis::PrimaryX => neo_lvgl_sys::lv_chart_axis_t_LV_CHART_AXIS_PRIMARY_X,
            ChartAxis::SecondaryX => neo_lvgl_sys::lv_chart_axis_t_LV_CHART_AXIS_SECONDARY_X,
        }
    }
}

/// Handle to a chart data series
pub struct ChartSeries {
    raw: NonNull<neo_lvgl_sys::lv_chart_series_t>,
}

impl ChartSeries {
    /// Create from raw pointer
    ///
    /// # Safety
    ///
    /// Pointer must be valid.
    unsafe fn from_raw(ptr: *mut neo_lvgl_sys::lv_chart_series_t) -> Option<Self> {
        NonNull::new(ptr).map(|raw| Self { raw })
    }

    /// Get the raw pointer
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_chart_series_t {
        self.raw.as_ptr()
    }
}

/// Chart widget
///
/// Displays data as line, bar, or scatter charts.
///
/// # Example
///
/// ```ignore
/// let chart = Chart::new(&screen).unwrap();
/// chart.set_type(ChartType::Line);
/// chart.set_point_count(10);
/// chart.set_range(ChartAxis::PrimaryY, 0, 100);
///
/// let series = chart.add_series(Color::red(), ChartAxis::PrimaryY).unwrap();
/// chart.set_next(&series, 10);
/// chart.set_next(&series, 25);
/// chart.set_next(&series, 40);
/// ```
#[derive(Clone, Copy)]
pub struct Chart<'a> {
    obj: Obj<'a>,
}

impl<'a> Chart<'a> {
    /// Create a new chart as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_chart_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the chart type
    pub fn set_type(&self, chart_type: ChartType) {
        unsafe {
            neo_lvgl_sys::lv_chart_set_type(self.obj.raw(), chart_type.to_raw());
        }
    }

    /// Set the number of data points per series
    pub fn set_point_count(&self, count: u32) {
        unsafe {
            neo_lvgl_sys::lv_chart_set_point_count(self.obj.raw(), count);
        }
    }

    /// Get the number of data points per series
    pub fn point_count(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_chart_get_point_count(self.obj.raw()) }
    }

    /// Set the range for an axis
    pub fn set_range(&self, axis: ChartAxis, min: i32, max: i32) {
        unsafe {
            neo_lvgl_sys::lv_chart_set_axis_range(self.obj.raw(), axis.to_raw(), min, max);
        }
    }

    /// Set the number of horizontal division lines
    pub fn set_div_line_count(&self, hdiv: u32, vdiv: u32) {
        unsafe {
            neo_lvgl_sys::lv_chart_set_div_line_count(self.obj.raw(), hdiv, vdiv);
        }
    }

    /// Add a data series to the chart
    pub fn add_series(&self, color: Color, axis: ChartAxis) -> Option<ChartSeries> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_chart_add_series(self.obj.raw(), color.to_raw(), axis.to_raw());
            ChartSeries::from_raw(ptr)
        }
    }

    /// Remove a data series from the chart
    pub fn remove_series(&self, series: &ChartSeries) {
        unsafe {
            neo_lvgl_sys::lv_chart_remove_series(self.obj.raw(), series.raw());
        }
    }

    /// Hide a data series
    pub fn hide_series(&self, series: &ChartSeries, hide: bool) {
        unsafe {
            neo_lvgl_sys::lv_chart_hide_series(self.obj.raw(), series.raw(), hide);
        }
    }

    /// Set the next value in a series (shifts older values)
    pub fn set_next(&self, series: &ChartSeries, value: i32) {
        unsafe {
            neo_lvgl_sys::lv_chart_set_next_value(self.obj.raw(), series.raw(), value);
        }
    }

    /// Set a specific value in a series
    pub fn set_value(&self, series: &ChartSeries, index: u32, value: i32) {
        unsafe {
            neo_lvgl_sys::lv_chart_set_series_value_by_id(self.obj.raw(), series.raw(), index, value);
        }
    }

    /// Get a pointer to the Y values array
    pub fn get_y_array(&self, series: &ChartSeries) -> *mut i32 {
        unsafe { neo_lvgl_sys::lv_chart_get_series_y_array(self.obj.raw(), series.raw()) }
    }

    /// Refresh the chart (call after modifying external arrays)
    pub fn refresh(&self) {
        unsafe {
            neo_lvgl_sys::lv_chart_refresh(self.obj.raw());
        }
    }

    /// Set whether to update the chart when new values are added
    pub fn set_update_mode(&self, update: bool) {
        let mode = if update {
            neo_lvgl_sys::lv_chart_update_mode_t_LV_CHART_UPDATE_MODE_SHIFT
        } else {
            neo_lvgl_sys::lv_chart_update_mode_t_LV_CHART_UPDATE_MODE_CIRCULAR
        };
        unsafe {
            neo_lvgl_sys::lv_chart_set_update_mode(self.obj.raw(), mode);
        }
    }

    /// Get the index of the currently pressed point
    pub fn pressed_point(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_chart_get_pressed_point(self.obj.raw()) }
    }

    /// Set the cursor position for a series
    pub fn set_cursor_point(&self, series: &ChartSeries, point_id: u32) {
        // Cursor API requires getting cursor first
        unsafe {
            // Get the first cursor or create if needed
            let cursor = neo_lvgl_sys::lv_chart_add_cursor(
                self.obj.raw(),
                neo_lvgl_sys::lv_color_t { blue: 0, green: 0, red: 0 },
                neo_lvgl_sys::lv_dir_t_LV_DIR_NONE,
            );
            if !cursor.is_null() {
                neo_lvgl_sys::lv_chart_set_cursor_point(self.obj.raw(), cursor, series.raw(), point_id);
            }
        }
    }
}

impl<'a> Widget<'a> for Chart<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Chart<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



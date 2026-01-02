//! Table widget

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};
use bitflags::bitflags;
use core::ffi::CStr;

bitflags! {
    /// Table cell control flags
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct TableCellCtrl: u32 {
        /// Cell spans multiple columns
        const MERGE_RIGHT = neo_lvgl_sys::lv_table_cell_ctrl_t_LV_TABLE_CELL_CTRL_MERGE_RIGHT;
        /// Enable text recoloring for this cell
        const TEXT_CROP = neo_lvgl_sys::lv_table_cell_ctrl_t_LV_TABLE_CELL_CTRL_TEXT_CROP;
        /// Custom style 1
        const CUSTOM_1 = neo_lvgl_sys::lv_table_cell_ctrl_t_LV_TABLE_CELL_CTRL_CUSTOM_1;
        /// Custom style 2
        const CUSTOM_2 = neo_lvgl_sys::lv_table_cell_ctrl_t_LV_TABLE_CELL_CTRL_CUSTOM_2;
        /// Custom style 3
        const CUSTOM_3 = neo_lvgl_sys::lv_table_cell_ctrl_t_LV_TABLE_CELL_CTRL_CUSTOM_3;
        /// Custom style 4
        const CUSTOM_4 = neo_lvgl_sys::lv_table_cell_ctrl_t_LV_TABLE_CELL_CTRL_CUSTOM_4;
    }
}

/// Table widget
///
/// A grid of cells displaying text data.
///
/// # Example
///
/// ```ignore
/// let table = Table::new(&screen).unwrap();
/// table.set_column_count(3);
/// table.set_row_count(4);
///
/// // Set header row
/// table.set_cell_value(0, 0, c"Name");
/// table.set_cell_value(0, 1, c"Age");
/// table.set_cell_value(0, 2, c"City");
///
/// // Add data
/// table.set_cell_value(1, 0, c"Alice");
/// table.set_cell_value(1, 1, c"25");
/// table.set_cell_value(1, 2, c"NYC");
/// ```
#[derive(Clone, Copy)]
pub struct Table<'a> {
    obj: Obj<'a>,
}

impl<'a> Table<'a> {
    /// Create a new table as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_table_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Set the cell text at the given row and column
    pub fn set_cell_value(&self, row: u32, col: u32, text: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_table_set_cell_value(self.obj.raw(), row, col, text.as_ptr());
        }
    }

    /// Set the cell text with printf-style formatting
    ///
    /// Note: Only the text parameter is used here, not format arguments.
    pub fn set_cell_value_fmt(&self, row: u32, col: u32, text: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_table_set_cell_value(self.obj.raw(), row, col, text.as_ptr());
        }
    }

    /// Get the cell text at the given row and column
    pub fn cell_value(&self, row: u32, col: u32) -> Option<&CStr> {
        let ptr = unsafe { neo_lvgl_sys::lv_table_get_cell_value(self.obj.raw(), row, col) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }

    /// Set the number of rows
    pub fn set_row_count(&self, count: u32) {
        unsafe {
            neo_lvgl_sys::lv_table_set_row_count(self.obj.raw(), count);
        }
    }

    /// Get the number of rows
    pub fn row_count(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_table_get_row_count(self.obj.raw()) }
    }

    /// Set the number of columns
    pub fn set_column_count(&self, count: u32) {
        unsafe {
            neo_lvgl_sys::lv_table_set_column_count(self.obj.raw(), count);
        }
    }

    /// Get the number of columns
    pub fn column_count(&self) -> u32 {
        unsafe { neo_lvgl_sys::lv_table_get_column_count(self.obj.raw()) as u32 }
    }

    /// Set the width of a column
    pub fn set_column_width(&self, col: u32, width: i32) {
        unsafe {
            neo_lvgl_sys::lv_table_set_column_width(self.obj.raw(), col, width);
        }
    }

    /// Get the width of a column
    pub fn column_width(&self, col: u32) -> i32 {
        unsafe { neo_lvgl_sys::lv_table_get_column_width(self.obj.raw(), col) }
    }

    /// Set cell control flags
    pub fn set_cell_ctrl(&self, row: u32, col: u32, ctrl: TableCellCtrl) {
        unsafe {
            neo_lvgl_sys::lv_table_set_cell_ctrl(self.obj.raw(), row, col, ctrl.bits());
        }
    }

    /// Check if a cell has certain control flags
    pub fn has_cell_ctrl(&self, row: u32, col: u32, ctrl: TableCellCtrl) -> bool {
        unsafe { neo_lvgl_sys::lv_table_has_cell_ctrl(self.obj.raw(), row, col, ctrl.bits()) }
    }

    /// Get the selected cell (row, col)
    pub fn selected_cell(&self) -> (u32, u32) {
        let mut row = 0u32;
        let mut col = 0u32;
        unsafe {
            neo_lvgl_sys::lv_table_get_selected_cell(self.obj.raw(), &mut row, &mut col);
        }
        (row, col)
    }
}

impl<'a> Widget<'a> for Table<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for Table<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



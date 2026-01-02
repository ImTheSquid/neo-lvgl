//! TileView widget

use crate::event::EventHandler;
use crate::widgets::{Obj, Widget};

/// A tile within a TileView
#[derive(Clone, Copy)]
pub struct TileViewTile<'a> {
    obj: Obj<'a>,
}

impl<'a> TileViewTile<'a> {
    /// Create from raw object
    unsafe fn from_obj(obj: Obj<'a>) -> Self {
        Self { obj }
    }
}

impl<'a> Widget<'a> for TileViewTile<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for TileViewTile<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



/// TileView widget
///
/// A scrollable container of full-screen tiles arranged in a grid.
///
/// # Example
///
/// ```ignore
/// let tileview = TileView::new(&screen).unwrap();
///
/// // Create a 3x1 grid of tiles
/// let tile1 = tileview.add_tile(0, 0, Direction::Right).unwrap();
/// let tile2 = tileview.add_tile(1, 0, Direction::Left | Direction::Right).unwrap();
/// let tile3 = tileview.add_tile(2, 0, Direction::Left).unwrap();
///
/// // Add content to tiles
/// let label = Label::new(&tile1).unwrap();
/// label.set_text(c"Swipe right");
/// ```
#[derive(Clone, Copy)]
pub struct TileView<'a> {
    obj: Obj<'a>,
}

impl<'a> TileView<'a> {
    /// Create a new tileview as a child of the given parent.
    pub fn new(parent: &'a impl Widget<'a>) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_tileview_create(parent.raw());
            Obj::from_raw(ptr).map(|obj| Self { obj })
        }
    }

    /// Add a tile at the given column and row
    ///
    /// # Arguments
    ///
    /// * `col` - Column index
    /// * `row` - Row index
    /// * `dir` - Allowed scroll directions from this tile
    pub fn add_tile(&self, col: u32, row: u32, dir: u8) -> Option<TileViewTile<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_tileview_add_tile(
                self.obj.raw(),
                col as u8,
                row as u8,
                dir as neo_lvgl_sys::lv_dir_t,
            );
            Obj::from_raw(ptr).map(|obj| TileViewTile::from_obj(obj))
        }
    }

    /// Set the currently active tile
    pub fn set_tile(&self, tile: &TileViewTile, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_tileview_set_tile(self.obj.raw(), tile.obj.raw(), anim);
        }
    }

    /// Set the active tile by index
    pub fn set_tile_by_index(&self, col: u32, row: u32, anim: bool) {
        unsafe {
            neo_lvgl_sys::lv_tileview_set_tile_by_index(self.obj.raw(), col, row, anim);
        }
    }

    /// Get the currently active tile
    pub fn active_tile(&self) -> Option<TileViewTile<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_tileview_get_tile_active(self.obj.raw());
            Obj::from_raw(ptr).map(|obj| TileViewTile::from_obj(obj))
        }
    }
}

impl<'a> Widget<'a> for TileView<'a> {
    fn obj(&self) -> &Obj<'a> {
        &self.obj
    }
}

impl EventHandler for TileView<'_> {
    fn obj_raw(&self) -> *mut neo_lvgl_sys::lv_obj_t {
        self.obj.raw()
    }
}



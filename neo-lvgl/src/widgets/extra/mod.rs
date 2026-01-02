//! Extra widgets (feature-gated)
//!
//! These widgets provide additional functionality beyond the core widget set.
//! Enable them via Cargo features like `widgets-extra` or individual `widget-*` features.

#[cfg(feature = "widgets-extra")]
mod calendar;
#[cfg(feature = "widgets-extra")]
mod chart;
#[cfg(feature = "widgets-extra")]
mod keyboard;
#[cfg(feature = "widgets-extra")]
mod led;
#[cfg(feature = "widgets-extra")]
mod list;
#[cfg(feature = "widgets-extra")]
mod menu;
#[cfg(feature = "widgets-extra")]
mod msgbox;
#[cfg(feature = "widgets-extra")]
mod scale;
#[cfg(feature = "widgets-extra")]
mod span;
#[cfg(feature = "widgets-extra")]
mod spinbox;
#[cfg(feature = "widgets-extra")]
mod spinner;
#[cfg(feature = "widgets-extra")]
mod table;
#[cfg(feature = "widgets-extra")]
mod tabview;
#[cfg(feature = "widgets-extra")]
mod tileview;
#[cfg(feature = "widgets-extra")]
mod window;

#[cfg(feature = "widgets-extra")]
pub use calendar::{Calendar, CalendarDate};
#[cfg(feature = "widgets-extra")]
pub use chart::{Chart, ChartAxis, ChartSeries, ChartType};
#[cfg(feature = "widgets-extra")]
pub use keyboard::{Keyboard, KeyboardMode};
#[cfg(feature = "widgets-extra")]
pub use led::Led;
#[cfg(feature = "widgets-extra")]
pub use list::{List, ListButton, ListText};
#[cfg(feature = "widgets-extra")]
pub use menu::{Menu, MenuPage, MenuSection, MenuSeparator};
#[cfg(feature = "widgets-extra")]
pub use msgbox::MsgBox;
#[cfg(feature = "widgets-extra")]
pub use scale::{Scale, ScaleMode, ScaleSectionDescr};
#[cfg(feature = "widgets-extra")]
pub use span::{Span, SpanGroup, SpanMode, SpanOverflow};
#[cfg(feature = "widgets-extra")]
pub use spinbox::Spinbox;
#[cfg(feature = "widgets-extra")]
pub use spinner::Spinner;
#[cfg(feature = "widgets-extra")]
pub use table::{Table, TableCellCtrl};
#[cfg(feature = "widgets-extra")]
pub use tabview::{TabView, TabViewPos};
#[cfg(feature = "widgets-extra")]
pub use tileview::{TileView, TileViewTile};
#[cfg(feature = "widgets-extra")]
pub use window::Window;

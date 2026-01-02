//! Raw FFI bindings for LVGL 9.x
//!
//! This crate provides unsafe, low-level bindings to the LVGL graphics library.
//! For safe, idiomatic Rust APIs, use the `lvgl` crate instead.
//!
//! # Safety
//!
//! All functions in this crate are unsafe and follow C conventions.
//! Refer to the [LVGL documentation](https://docs.lvgl.io/) for usage details.

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]
// Suppress doc warnings from auto-generated C comments (contain [ms], [px], etc.)
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_make() {
        unsafe {
            // Basic sanity check that bindings work
            let _color = lv_color_make(255, 0, 0);
        }
    }
}

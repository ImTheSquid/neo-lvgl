//! XML UI loading for LVGL
//!
//! This module provides support for loading UI components from XML files or data.
//! Components defined in XML can be registered and then instantiated at runtime.
//!
//! # Example
//!
//! ```ignore
//! use lvgl::xml;
//!
//! // Initialize XML system
//! xml::init();
//!
//! // Set the asset path for loading files
//! xml::set_default_asset_path(c"A:assets/");
//!
//! // Register a component from file
//! xml::register_component_from_file(c"A:components/my_button.xml").unwrap();
//!
//! // Or register from inline XML data
//! xml::register_component_from_data(c"my_button", c"<component>...</component>").unwrap();
//!
//! // Create an instance of the component (simple, no attributes)
//! let obj = xml::create_simple(&screen, c"my_button").unwrap();
//!
//! // Or with attributes (requires alloc feature)
//! let obj = xml::create(&screen, c"my_button", &[
//!     (c"x", c"10"),
//!     (c"y", c"20"),
//! ]).unwrap();
//! ```

use crate::widgets::{Obj, Widget};
use core::ffi::CStr;

#[cfg(feature = "alloc")]
use core::ffi::c_char;

/// XML loading error
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XmlError {
    /// Failed to load or parse XML
    LoadFailed,
    /// Component not found
    NotFound,
    /// Invalid XML data
    InvalidData,
}

/// Initialize the XML system
///
/// Call this before using any other XML functions.
/// It's safe to call multiple times.
pub fn init() {
    unsafe {
        neo_lvgl_sys::lv_xml_init();
    }
}

/// Deinitialize the XML system
///
/// Call this to free XML-related resources.
pub fn deinit() {
    unsafe {
        neo_lvgl_sys::lv_xml_deinit();
    }
}

/// Set the default asset path prefix for loading XML files and resources
///
/// # Arguments
///
/// * `path_prefix` - The path prefix (e.g., "A:assets/")
pub fn set_default_asset_path(path_prefix: &CStr) {
    unsafe {
        neo_lvgl_sys::lv_xml_set_default_asset_path(path_prefix.as_ptr().cast());
    }
}

/// Register a component from an XML file
///
/// The component can then be created using `create()` with the component name.
///
/// # Arguments
///
/// * `path` - Path to the XML file (e.g., "A:components/my_button.xml")
///
/// # Returns
///
/// `Ok(())` on success, `Err(XmlError)` on failure
pub fn register_component_from_file(path: &CStr) -> Result<(), XmlError> {
    let result = unsafe { neo_lvgl_sys::lv_xml_register_component_from_file(path.as_ptr().cast()) };
    if result == neo_lvgl_sys::lv_result_t_LV_RESULT_OK {
        Ok(())
    } else {
        Err(XmlError::LoadFailed)
    }
}

/// Register a component from XML data string
///
/// # Arguments
///
/// * `name` - The name to register the component as
/// * `xml_data` - The XML component definition
///
/// # Returns
///
/// `Ok(())` on success, `Err(XmlError)` on failure
pub fn register_component_from_data(name: &CStr, xml_data: &CStr) -> Result<(), XmlError> {
    let result = unsafe {
        neo_lvgl_sys::lv_xml_register_component_from_data(name.as_ptr().cast(), xml_data.as_ptr().cast())
    };
    if result == neo_lvgl_sys::lv_result_t_LV_RESULT_OK {
        Ok(())
    } else {
        Err(XmlError::InvalidData)
    }
}

/// Unregister a previously registered component
///
/// # Arguments
///
/// * `name` - The name of the component to unregister
pub fn unregister_component(name: &CStr) -> Result<(), XmlError> {
    let result = unsafe { neo_lvgl_sys::lv_xml_unregister_component(name.as_ptr().cast()) };
    if result == neo_lvgl_sys::lv_result_t_LV_RESULT_OK {
        Ok(())
    } else {
        Err(XmlError::NotFound)
    }
}

/// Load all XML components from a directory path
///
/// This recursively loads all XML files from the given path.
///
/// # Arguments
///
/// * `path` - Directory path to load from
pub fn load_all_from_path(path: &CStr) -> Result<(), XmlError> {
    let result = unsafe { neo_lvgl_sys::lv_xml_load_all_from_path(path.as_ptr().cast()) };
    if result == neo_lvgl_sys::lv_result_t_LV_RESULT_OK {
        Ok(())
    } else {
        Err(XmlError::LoadFailed)
    }
}

/// Create an instance of a registered XML component with attributes
///
/// # Arguments
///
/// * `parent` - The parent widget
/// * `name` - The name of the registered component
/// * `attrs` - Attribute pairs (name, value)
///
/// # Returns
///
/// The created widget, or `None` if creation failed
///
/// # Example
///
/// ```ignore
/// let obj = xml::create(&screen, c"my_button", &[
///     (c"x", c"10"),
///     (c"y", c"20"),
///     (c"width", c"100"),
/// ]).unwrap();
/// ```
#[cfg(feature = "alloc")]
pub fn create<'a>(
    parent: &'a impl Widget<'a>,
    name: &CStr,
    attrs: &[(&CStr, &CStr)],
) -> Option<Obj<'a>> {
    unsafe {
        // Build null-terminated array of attribute strings
        // We need: [name1, val1, name2, val2, ..., NULL, NULL]
        let mut attr_ptrs: alloc::vec::Vec<*const c_char> =
            alloc::vec::Vec::with_capacity(attrs.len() * 2 + 2);

        for (key, value) in attrs {
            attr_ptrs.push(key.as_ptr().cast());
            attr_ptrs.push(value.as_ptr().cast());
        }
        attr_ptrs.push(core::ptr::null());
        attr_ptrs.push(core::ptr::null());

        let ptr = neo_lvgl_sys::lv_xml_create(
            parent.raw(),
            name.as_ptr().cast(),
            attr_ptrs.as_mut_ptr().cast(),
        );

        Obj::from_raw(ptr as *mut neo_lvgl_sys::lv_obj_t)
    }
}

/// Create an instance of a registered XML component (no attributes version)
///
/// This is a simpler version that doesn't require the alloc feature.
///
/// # Arguments
///
/// * `parent` - The parent widget
/// * `name` - The name of the registered component
pub fn create_simple<'a>(parent: &'a impl Widget<'a>, name: &CStr) -> Option<Obj<'a>> {
    unsafe {
        let ptr = neo_lvgl_sys::lv_xml_create(parent.raw(), name.as_ptr().cast(), core::ptr::null_mut());
        Obj::from_raw(ptr as *mut neo_lvgl_sys::lv_obj_t)
    }
}

/// Create a screen from an XML component
///
/// The component should be a screen-type component.
///
/// # Arguments
///
/// * `name` - The name of the registered screen component
pub fn create_screen(name: &CStr) -> Option<Obj<'static>> {
    unsafe {
        let ptr = neo_lvgl_sys::lv_xml_create_screen(name.as_ptr().cast());
        Obj::from_raw(ptr)
    }
}

/// Register a font with the XML system (global scope)
///
/// After registration, the font can be referenced by name in XML.
///
/// # Arguments
///
/// * `name` - The name to register the font as
/// * `font` - The font to register
pub fn register_font(name: &CStr, font: &crate::font::Font) {
    unsafe {
        neo_lvgl_sys::lv_xml_register_font(core::ptr::null_mut(), name.as_ptr().cast(), font.raw());
    }
}

/// Register an image source with the XML system (global scope)
///
/// After registration, the image can be referenced by name in XML.
///
/// # Arguments
///
/// * `name` - The name to register the image as
/// * `src` - The image source (path or image descriptor)
pub fn register_image(name: &CStr, src: *const core::ffi::c_void) {
    unsafe {
        neo_lvgl_sys::lv_xml_register_image(core::ptr::null_mut(), name.as_ptr().cast(), src);
    }
}

/// Register a subject with the XML system for data binding (global scope)
///
/// After registration, the subject can be referenced by name in XML for binding.
///
/// # Arguments
///
/// * `name` - The name to register the subject as
/// * `subject` - The subject to register
pub fn register_subject(name: &CStr, subject: &mut impl crate::observer::Subject) {
    unsafe {
        neo_lvgl_sys::lv_xml_register_subject(core::ptr::null_mut(), name.as_ptr().cast(), subject.raw());
    }
}

/// Register a constant value with the XML system (global scope)
///
/// # Arguments
///
/// * `name` - The name of the constant
/// * `value` - The value as a string
pub fn register_const(name: &CStr, value: &CStr) {
    unsafe {
        neo_lvgl_sys::lv_xml_register_const(core::ptr::null_mut(), name.as_ptr().cast(), value.as_ptr().cast());
    }
}

// === Widget name support ===

/// Extension trait for widget naming (used by XML)
pub trait NameExt<'a>: Widget<'a> {
    /// Set the name of this widget
    ///
    /// Names can be used to find widgets with `find_by_name()`.
    fn set_name(&self, name: &CStr) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_name(self.raw(), name.as_ptr().cast());
        }
    }

    /// Set a static name for this widget (not copied)
    ///
    /// The string must have static lifetime.
    fn set_name_static(&self, name: &'static CStr) {
        unsafe {
            neo_lvgl_sys::lv_obj_set_name_static(self.raw(), name.as_ptr().cast());
        }
    }

    /// Find a child widget by name
    ///
    /// Searches this widget's descendants for one with the given name.
    fn find_by_name(&self, name: &CStr) -> Option<Obj<'a>> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_obj_find_by_name(self.raw(), name.as_ptr().cast());
            Obj::from_raw(ptr)
        }
    }
}

impl<'a, T: Widget<'a>> NameExt<'a> for T {}

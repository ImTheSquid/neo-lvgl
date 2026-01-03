//! Fragment system for LVGL
//!
//! Fragments are reusable UI components with lifecycle management,
//! similar to Android's Fragment system. They support:
//!
//! - Navigation stack (push/pop)
//! - Lifecycle callbacks (create, attach, detach, destroy)
//! - Nested fragment managers
//!
//! # Example
//!
//! ```ignore
//! use lvgl::fragment::{Fragment, FragmentManager, FragmentImpl};
//! use lvgl::widgets::{Obj, Label, Widget};
//!
//! struct HomeFragment {
//!     title: &'static str,
//! }
//!
//! impl FragmentImpl for HomeFragment {
//!     fn create_obj(&mut self, container: &Obj) -> Option<Obj> {
//!         let obj = Obj::new(container)?;
//!         let label = Label::new(&obj)?;
//!         label.set_text(c"Home Screen");
//!         Some(obj)
//!     }
//! }
//!
//! // Create manager and push fragment
//! let manager = FragmentManager::new(None);
//! let home = Fragment::new(HomeFragment { title: "Home" });
//! manager.push(home, &screen);
//! ```

use crate::widgets::{Obj, Widget};
use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Trait for implementing fragment behavior.
///
/// Implement this trait to define your fragment's lifecycle and UI creation.
pub trait FragmentImpl: Sized {
    /// Called when the fragment is constructed.
    ///
    /// Use this to initialize your fragment state.
    fn constructor(&mut self) {}

    /// Called when the fragment is destroyed.
    ///
    /// Use this to clean up resources.
    fn destructor(&mut self) {}

    /// Called when the fragment is attached to a manager.
    fn attached(&mut self) {}

    /// Called when the fragment is detached from a manager.
    fn detached(&mut self) {}

    /// Create the UI objects for this fragment.
    ///
    /// This is called when the fragment needs to create its visual representation.
    /// Return the root object of your fragment's UI.
    fn create_obj(&mut self, container: &Obj) -> Option<Obj<'_>>;

    /// Called after `create_obj` with the created object.
    fn obj_created(&mut self, _obj: &Obj) {}

    /// Called before the fragment's objects will be deleted.
    fn obj_will_delete(&mut self, _obj: &Obj) {}

    /// Called when the fragment's objects have been deleted.
    fn obj_deleted(&mut self) {}

    /// Handle a custom event sent to this fragment.
    ///
    /// Return `true` if the event was handled.
    fn on_event(&mut self, _code: i32, _userdata: *mut c_void) -> bool {
        false
    }
}

/// A fragment instance wrapping user data.
///
/// This manages the lifecycle of a fragment and its associated LVGL resources.
#[cfg(feature = "alloc")]
pub struct Fragment<T: FragmentImpl> {
    raw: NonNull<neo_lvgl_sys::lv_fragment_t>,
    _marker: PhantomData<T>,
}

#[cfg(feature = "alloc")]
impl<T: FragmentImpl + 'static> Fragment<T> {
    /// Create a new fragment with the given implementation.
    pub fn new(data: T) -> Option<Self> {
        // Create the fragment class for this type
        let cls = FragmentClass::<T>::get();

        // Box the user data
        let boxed_data = Box::new(data);
        let args = Box::into_raw(boxed_data) as *mut c_void;

        let raw = unsafe { neo_lvgl_sys::lv_fragment_create(cls, args) };

        NonNull::new(raw).map(|raw| Self {
            raw,
            _marker: PhantomData,
        })
    }

    /// Get the raw LVGL fragment pointer.
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_fragment_t {
        self.raw.as_ptr()
    }

    /// Get a reference to the user data.
    ///
    /// # Safety
    ///
    /// The fragment must have been created with this type.
    pub fn data(&self) -> &T {
        unsafe {
            let wrapper = get_wrapper::<T>(self.raw.as_ptr());
            &(*wrapper).data
        }
    }

    /// Get a mutable reference to the user data.
    ///
    /// # Safety
    ///
    /// The fragment must have been created with this type.
    pub fn data_mut(&mut self) -> &mut T {
        unsafe {
            let wrapper = get_wrapper::<T>(self.raw.as_ptr());
            &mut (*wrapper).data
        }
    }

    /// Get the object created by this fragment.
    pub fn obj(&self) -> Option<Obj<'_>> {
        unsafe {
            let obj = (*self.raw.as_ptr()).obj;
            Obj::from_raw(obj)
        }
    }

    /// Recreate the fragment's objects.
    pub fn recreate_obj(&self) {
        unsafe {
            neo_lvgl_sys::lv_fragment_recreate_obj(self.raw.as_ptr());
        }
    }

    /// Delete the fragment's objects without destroying the fragment.
    pub fn delete_obj(&self) {
        unsafe {
            neo_lvgl_sys::lv_fragment_delete_obj(self.raw.as_ptr());
        }
    }
}

/// Fragment manager for managing fragment lifecycle and navigation.
pub struct FragmentManager {
    raw: NonNull<neo_lvgl_sys::lv_fragment_manager_t>,
    owned: bool,
}

impl FragmentManager {
    /// Create a new fragment manager.
    ///
    /// If `parent` is provided, this manager is nested within another fragment.
    pub fn new(parent: Option<&neo_lvgl_sys::lv_fragment_t>) -> Option<Self> {
        let parent_ptr = parent
            .map(|p| p as *const _ as *mut _)
            .unwrap_or(core::ptr::null_mut());

        let raw = unsafe { neo_lvgl_sys::lv_fragment_manager_create(parent_ptr) };

        NonNull::new(raw).map(|raw| Self { raw, owned: true })
    }

    /// Create from a raw pointer (non-owning).
    ///
    /// # Safety
    ///
    /// The pointer must be valid for the lifetime of this wrapper.
    pub unsafe fn from_raw(raw: *mut neo_lvgl_sys::lv_fragment_manager_t) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw, owned: false })
    }

    /// Get the raw pointer.
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_fragment_manager_t {
        self.raw.as_ptr()
    }

    /// Add a fragment to this manager.
    ///
    /// The fragment will be attached and its objects created in the container.
    #[cfg(feature = "alloc")]
    pub fn add<'a, T: FragmentImpl + 'static>(
        &self,
        fragment: Fragment<T>,
        container: &impl Widget<'a>,
    ) {
        let container_ptr = container.raw();
        unsafe {
            // LVGL expects a pointer to a pointer for the container
            let container_ref = Box::new(container_ptr);
            let container_ptr_ptr = Box::into_raw(container_ref);
            neo_lvgl_sys::lv_fragment_manager_add(
                self.raw.as_ptr(),
                fragment.raw(),
                container_ptr_ptr,
            );
            // Note: LVGL keeps the container pointer, we leak intentionally
        }
        // Don't drop the fragment - it's now owned by the manager
        core::mem::forget(fragment);
    }

    /// Push a fragment onto the navigation stack.
    ///
    /// The previous top fragment's objects will be deleted (but not destroyed).
    #[cfg(feature = "alloc")]
    pub fn push<'a, T: FragmentImpl + 'static>(
        &self,
        fragment: Fragment<T>,
        container: &impl Widget<'a>,
    ) {
        let container_ptr = container.raw();
        unsafe {
            let container_ref = Box::new(container_ptr);
            let container_ptr_ptr = Box::into_raw(container_ref);
            neo_lvgl_sys::lv_fragment_manager_push(
                self.raw.as_ptr(),
                fragment.raw(),
                container_ptr_ptr,
            );
        }
        core::mem::forget(fragment);
    }

    /// Pop the top fragment from the navigation stack.
    ///
    /// Returns `true` if a fragment was popped.
    pub fn pop(&self) -> bool {
        unsafe { neo_lvgl_sys::lv_fragment_manager_pop(self.raw.as_ptr()) }
    }

    /// Replace the top fragment with a new one.
    #[cfg(feature = "alloc")]
    pub fn replace<'a, T: FragmentImpl + 'static>(
        &self,
        fragment: Fragment<T>,
        container: &impl Widget<'a>,
    ) {
        let container_ptr = container.raw();
        unsafe {
            let container_ref = Box::new(container_ptr);
            let container_ptr_ptr = Box::into_raw(container_ref);
            neo_lvgl_sys::lv_fragment_manager_replace(
                self.raw.as_ptr(),
                fragment.raw(),
                container_ptr_ptr,
            );
        }
        core::mem::forget(fragment);
    }

    /// Get the number of fragments in the navigation stack.
    pub fn stack_size(&self) -> usize {
        unsafe { neo_lvgl_sys::lv_fragment_manager_get_stack_size(self.raw.as_ptr()) }
    }

    /// Send an event to the top fragment.
    ///
    /// Returns `true` if the fragment handled the event.
    pub fn send_event(&self, code: i32, userdata: *mut c_void) -> bool {
        unsafe { neo_lvgl_sys::lv_fragment_manager_send_event(self.raw.as_ptr(), code, userdata) }
    }

    /// Create objects for all managed fragments.
    pub fn create_obj(&self) {
        unsafe {
            neo_lvgl_sys::lv_fragment_manager_create_obj(self.raw.as_ptr());
        }
    }

    /// Delete objects for all managed fragments.
    pub fn delete_obj(&self) {
        unsafe {
            neo_lvgl_sys::lv_fragment_manager_delete_obj(self.raw.as_ptr());
        }
    }
}

impl Drop for FragmentManager {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                neo_lvgl_sys::lv_fragment_manager_delete(self.raw.as_ptr());
            }
        }
    }
}

// Internal wrapper struct that holds both the LVGL fragment and user data
#[repr(C)]
struct FragmentWrapper<T> {
    // Must be first - LVGL fragment base
    base: neo_lvgl_sys::lv_fragment_t,
    // User data
    data: T,
}

// Get the wrapper from a fragment pointer
unsafe fn get_wrapper<T>(fragment: *mut neo_lvgl_sys::lv_fragment_t) -> *mut FragmentWrapper<T> {
    fragment as *mut FragmentWrapper<T>
}

/// Internal: Fragment class with static callbacks for a given type.
#[cfg(feature = "alloc")]
struct FragmentClass<T> {
    _marker: PhantomData<T>,
}

#[cfg(feature = "alloc")]
impl<T: FragmentImpl + 'static> FragmentClass<T> {
    /// Get or create the fragment class for this type.
    fn get() -> *const neo_lvgl_sys::lv_fragment_class_t {
        // Use a static for each type
        static_class::<T>()
    }
}

// Generate static class instance for each fragment type
#[cfg(feature = "alloc")]
fn static_class<T: FragmentImpl + 'static>() -> *const neo_lvgl_sys::lv_fragment_class_t {
    use core::sync::atomic::{AtomicPtr, Ordering};
    use neo_lvgl_sys::lv_fragment_class_t;

    static CLASS: AtomicPtr<lv_fragment_class_t> = AtomicPtr::new(core::ptr::null_mut());

    let ptr = CLASS.load(Ordering::Acquire);
    if !ptr.is_null() {
        return ptr;
    }

    // Create the class
    let cls = Box::new(lv_fragment_class_t {
        constructor_cb: Some(constructor_cb::<T>),
        destructor_cb: Some(destructor_cb::<T>),
        attached_cb: Some(attached_cb::<T>),
        detached_cb: Some(detached_cb::<T>),
        create_obj_cb: Some(create_obj_cb::<T>),
        obj_created_cb: Some(obj_created_cb::<T>),
        obj_will_delete_cb: Some(obj_will_delete_cb::<T>),
        obj_deleted_cb: Some(obj_deleted_cb::<T>),
        event_cb: Some(event_cb::<T>),
        instance_size: core::mem::size_of::<FragmentWrapper<T>>(),
    });

    let raw = Box::into_raw(cls);
    CLASS.store(raw, Ordering::Release);
    raw
}

// C callback trampolines
#[cfg(feature = "alloc")]
unsafe extern "C" fn constructor_cb<T: FragmentImpl>(
    fragment: *mut neo_lvgl_sys::lv_fragment_t,
    args: *mut c_void,
) {
    // args contains the boxed user data
    let data = Box::from_raw(args as *mut T);
    let wrapper = get_wrapper::<T>(fragment);

    // Initialize the user data in place
    core::ptr::write(&mut (*wrapper).data, *data);

    // Call user constructor
    (*wrapper).data.constructor();
}

#[cfg(feature = "alloc")]
unsafe extern "C" fn destructor_cb<T: FragmentImpl>(fragment: *mut neo_lvgl_sys::lv_fragment_t) {
    let wrapper = get_wrapper::<T>(fragment);
    (*wrapper).data.destructor();
    // Drop the user data
    core::ptr::drop_in_place(&mut (*wrapper).data);
}

#[cfg(feature = "alloc")]
unsafe extern "C" fn attached_cb<T: FragmentImpl>(fragment: *mut neo_lvgl_sys::lv_fragment_t) {
    let wrapper = get_wrapper::<T>(fragment);
    (*wrapper).data.attached();
}

#[cfg(feature = "alloc")]
unsafe extern "C" fn detached_cb<T: FragmentImpl>(fragment: *mut neo_lvgl_sys::lv_fragment_t) {
    let wrapper = get_wrapper::<T>(fragment);
    (*wrapper).data.detached();
}

#[cfg(feature = "alloc")]
unsafe extern "C" fn create_obj_cb<T: FragmentImpl>(
    fragment: *mut neo_lvgl_sys::lv_fragment_t,
    container: *mut neo_lvgl_sys::lv_obj_t,
) -> *mut neo_lvgl_sys::lv_obj_t {
    let wrapper = get_wrapper::<T>(fragment);
    let container_obj = Obj::from_raw(container);

    if let Some(container) = container_obj {
        if let Some(obj) = (*wrapper).data.create_obj(&container) {
            return obj.raw();
        }
    }
    core::ptr::null_mut()
}

#[cfg(feature = "alloc")]
unsafe extern "C" fn obj_created_cb<T: FragmentImpl>(
    fragment: *mut neo_lvgl_sys::lv_fragment_t,
    obj: *mut neo_lvgl_sys::lv_obj_t,
) {
    let wrapper = get_wrapper::<T>(fragment);
    if let Some(obj) = Obj::from_raw(obj) {
        (*wrapper).data.obj_created(&obj);
    }
}

#[cfg(feature = "alloc")]
unsafe extern "C" fn obj_will_delete_cb<T: FragmentImpl>(
    fragment: *mut neo_lvgl_sys::lv_fragment_t,
    obj: *mut neo_lvgl_sys::lv_obj_t,
) {
    let wrapper = get_wrapper::<T>(fragment);
    if let Some(obj) = Obj::from_raw(obj) {
        (*wrapper).data.obj_will_delete(&obj);
    }
}

#[cfg(feature = "alloc")]
unsafe extern "C" fn obj_deleted_cb<T: FragmentImpl>(
    fragment: *mut neo_lvgl_sys::lv_fragment_t,
    _obj: *mut neo_lvgl_sys::lv_obj_t,
) {
    let wrapper = get_wrapper::<T>(fragment);
    (*wrapper).data.obj_deleted();
}

#[cfg(feature = "alloc")]
unsafe extern "C" fn event_cb<T: FragmentImpl>(
    fragment: *mut neo_lvgl_sys::lv_fragment_t,
    code: i32,
    userdata: *mut c_void,
) -> bool {
    let wrapper = get_wrapper::<T>(fragment);
    (*wrapper).data.on_event(code, userdata)
}

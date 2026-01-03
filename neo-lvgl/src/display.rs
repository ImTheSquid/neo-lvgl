//! Display management

use core::ptr::NonNull;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// LVGL display
pub struct Display {
    raw: NonNull<neo_lvgl_sys::lv_display_t>,
}

impl Display {
    /// Create a new display with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - Horizontal resolution in pixels
    /// * `height` - Vertical resolution in pixels
    pub fn new(width: i32, height: i32) -> Option<Self> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_display_create(width, height);
            NonNull::new(ptr).map(|raw| Self { raw })
        }
    }

    /// Get the raw pointer to the display
    #[inline]
    pub fn raw(&self) -> *mut neo_lvgl_sys::lv_display_t {
        self.raw.as_ptr()
    }

    /// Set this display as the default
    pub fn set_default(&self) {
        unsafe {
            neo_lvgl_sys::lv_display_set_default(self.raw.as_ptr());
        }
    }

    /// Get the currently active screen for this display
    pub fn active_screen(&self) -> crate::widgets::Screen<'_> {
        unsafe {
            let ptr = neo_lvgl_sys::lv_display_get_screen_active(self.raw.as_ptr());
            crate::widgets::Screen::from_raw(ptr)
        }
    }

    /// Set the display buffers for rendering.
    ///
    /// # Safety
    ///
    /// The buffers must remain valid for the lifetime of the display.
    pub unsafe fn set_buffers(
        &self,
        buf1: &'static mut [u8],
        buf2: Option<&'static mut [u8]>,
        render_mode: RenderMode,
    ) {
        let buf2_ptr = match &buf2 {
            Some(b) => b.as_ptr() as *mut _,
            None => core::ptr::null_mut(),
        };

        neo_lvgl_sys::lv_display_set_buffers(
            self.raw.as_ptr(),
            buf1.as_mut_ptr() as *mut _,
            buf2_ptr,
            buf1.len() as u32,
            render_mode.to_raw(),
        );
    }

    /// Set the flush callback for the display.
    ///
    /// This callback is invoked when LVGL needs to send pixels to the display hardware.
    pub fn set_flush_cb(&self, cb: FlushCb) {
        unsafe {
            neo_lvgl_sys::lv_display_set_flush_cb(self.raw.as_ptr(), Some(cb));
        }
    }

    /// Signal that the flush operation is complete.
    ///
    /// Call this from your flush callback when the transfer is done.
    pub fn flush_ready(&self) {
        unsafe {
            neo_lvgl_sys::lv_display_flush_ready(self.raw.as_ptr());
        }
    }

    /// Get display width
    pub fn width(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_display_get_horizontal_resolution(self.raw.as_ptr()) }
    }

    /// Get display height
    pub fn height(&self) -> i32 {
        unsafe { neo_lvgl_sys::lv_display_get_vertical_resolution(self.raw.as_ptr()) }
    }

    /// Set the color format for this display.
    ///
    /// This determines how pixels are encoded in the display buffers.
    /// The default is ARGB8888.
    pub fn set_color_format(&self, format: ColorFormat) {
        unsafe {
            neo_lvgl_sys::lv_display_set_color_format(self.raw.as_ptr(), format.to_raw());
        }
    }

    /// Get the current color format for this display.
    pub fn color_format(&self) -> ColorFormat {
        let raw = unsafe { neo_lvgl_sys::lv_display_get_color_format(self.raw.as_ptr()) };
        ColorFormat::from_raw(raw)
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            neo_lvgl_sys::lv_display_delete(self.raw.as_ptr());
        }
    }
}

/// Display render mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RenderMode {
    /// Partial rendering - only changed areas are redrawn
    #[default]
    Partial,
    /// Direct rendering - buffer represents the full screen
    Direct,
    /// Full refresh - entire screen is redrawn each frame
    Full,
}

impl RenderMode {
    fn to_raw(self) -> neo_lvgl_sys::lv_display_render_mode_t {
        match self {
            RenderMode::Partial => {
                neo_lvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_PARTIAL
            }
            RenderMode::Direct => {
                neo_lvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_DIRECT
            }
            RenderMode::Full => neo_lvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_FULL,
        }
    }
}

/// Display color format
///
/// Determines how pixels are encoded in display buffers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ColorFormat {
    /// 8-bit grayscale
    L8,
    /// 1-bit alpha
    A1,
    /// 2-bit alpha
    A2,
    /// 4-bit alpha
    A4,
    /// 8-bit alpha
    A8,
    /// 1-bit indexed color (palette)
    I1,
    /// 2-bit indexed color (palette)
    I2,
    /// 4-bit indexed color (palette)
    I4,
    /// 8-bit indexed color (palette)
    I8,
    /// 16-bit RGB (5-6-5)
    Rgb565,
    /// 16-bit RGB (5-6-5) with byte-swapped pixels
    Rgb565Swapped,
    /// 24-bit RGB (8-8-8)
    Rgb888,
    /// 32-bit ARGB (8-8-8-8)
    #[default]
    Argb8888,
    /// 32-bit XRGB (8-8-8-8, alpha ignored)
    Xrgb8888,
}

impl ColorFormat {
    fn to_raw(self) -> neo_lvgl_sys::lv_color_format_t {
        match self {
            ColorFormat::L8 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_L8,
            ColorFormat::A1 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A1,
            ColorFormat::A2 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A2,
            ColorFormat::A4 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A4,
            ColorFormat::A8 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A8,
            ColorFormat::I1 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I1,
            ColorFormat::I2 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I2,
            ColorFormat::I4 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I4,
            ColorFormat::I8 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I8,
            ColorFormat::Rgb565 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565,
            ColorFormat::Rgb565Swapped => {
                neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565_SWAPPED
            }
            ColorFormat::Rgb888 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB888,
            ColorFormat::Argb8888 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_ARGB8888,
            ColorFormat::Xrgb8888 => neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_XRGB8888,
        }
    }

    fn from_raw(raw: neo_lvgl_sys::lv_color_format_t) -> Self {
        match raw {
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_L8 => ColorFormat::L8,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A1 => ColorFormat::A1,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A2 => ColorFormat::A2,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A4 => ColorFormat::A4,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_A8 => ColorFormat::A8,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I1 => ColorFormat::I1,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I2 => ColorFormat::I2,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I4 => ColorFormat::I4,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I8 => ColorFormat::I8,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565 => ColorFormat::Rgb565,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565_SWAPPED => {
                ColorFormat::Rgb565Swapped
            }
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB888 => ColorFormat::Rgb888,
            neo_lvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_XRGB8888 => ColorFormat::Xrgb8888,
            _ => ColorFormat::Argb8888, // Default fallback
        }
    }

    /// Get the number of bytes per pixel for this format.
    ///
    /// Note: For sub-byte formats (A1, A2, A4, I1, I2, I4), this returns 1
    /// as the minimum addressable unit, though actual bits per pixel is less.
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            ColorFormat::A1 | ColorFormat::I1 => 1,
            ColorFormat::A2 | ColorFormat::I2 => 1,
            ColorFormat::A4 | ColorFormat::I4 => 1,
            ColorFormat::L8 | ColorFormat::A8 | ColorFormat::I8 => 1,
            ColorFormat::Rgb565 | ColorFormat::Rgb565Swapped => 2,
            ColorFormat::Rgb888 => 3,
            ColorFormat::Argb8888 | ColorFormat::Xrgb8888 => 4,
        }
    }
}

/// Flush callback type
pub type FlushCb = unsafe extern "C" fn(
    disp: *mut neo_lvgl_sys::lv_display_t,
    area: *const neo_lvgl_sys::lv_area_t,
    px_map: *mut u8,
);

/// Display driver trait for custom display implementations
pub trait DisplayDriver {
    /// Get display dimensions (width, height)
    fn size(&self) -> (i32, i32);

    /// Flush pixels to the display
    ///
    /// # Arguments
    ///
    /// * `area` - The rectangular area to update
    /// * `pixels` - The pixel data in the display's color format
    fn flush(&mut self, area: &Area, pixels: &[u8]);

    /// Called when flush is complete (optional)
    fn flush_ready(&mut self) {}
}

/// Rectangular area
#[derive(Clone, Copy, Debug)]
pub struct Area {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}

impl Area {
    /// Create from raw LVGL area
    pub fn from_raw(raw: &neo_lvgl_sys::lv_area_t) -> Self {
        Self {
            x1: raw.x1 as i16,
            y1: raw.y1 as i16,
            x2: raw.x2 as i16,
            y2: raw.y2 as i16,
        }
    }

    /// Get area width
    #[inline]
    pub fn width(&self) -> i16 {
        self.x2 - self.x1 + 1
    }

    /// Get area height
    #[inline]
    pub fn height(&self) -> i16 {
        self.y2 - self.y1 + 1
    }

    /// Get total pixel count
    #[inline]
    pub fn pixel_count(&self) -> usize {
        (self.width() as usize) * (self.height() as usize)
    }
}

/// A display that owns its driver and manages the flush callback automatically.
///
/// This is the recommended way to use custom display drivers. It handles all the
/// unsafe callback wiring internally.
///
/// # Example
///
/// ```ignore
/// struct MyDriver {
///     // your display hardware state
/// }
///
/// impl DisplayDriver for MyDriver {
///     fn size(&self) -> (i32, i32) {
///         (320, 240)
///     }
///
///     fn flush(&mut self, area: &Area, pixels: &[u8]) {
///         // Send pixels to your display hardware
///     }
/// }
///
/// // Easy way - let ManagedDisplay create and manage buffers
/// let driver = MyDriver { /* ... */ };
/// let display = ManagedDisplay::with_buffers(
///     driver,
///     ColorFormat::Rgb565,
///     RenderMode::Partial,
///     false, // single buffer
/// ).unwrap();
///
/// // Or with static buffers you manage yourself
/// let display = unsafe {
///     ManagedDisplay::from_static_buffers(driver, &mut buf1, None, RenderMode::Partial)
/// };
/// ```
#[cfg(feature = "alloc")]
pub struct ManagedDisplay<D: DisplayDriver> {
    display: Display,
    // Boxed so we have a stable address for the pointer stored in user_data
    driver: Box<D>,
    // Optional owned buffers (when using with_buffers constructor)
    _buf1: Option<Vec<u8>>,
    _buf2: Option<Vec<u8>>,
}

#[cfg(feature = "alloc")]
impl<D: DisplayDriver> ManagedDisplay<D> {
    /// Create a managed display that allocates and owns its buffers.
    ///
    /// This is the easiest way to create a display - just provide your driver
    /// and the display will manage everything else.
    ///
    /// # Arguments
    ///
    /// * `driver` - Your display driver implementation
    /// * `color_format` - The pixel format for the buffers
    /// * `render_mode` - The rendering mode to use
    /// * `double_buffer` - Whether to use double buffering (smoother but uses 2x memory)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let display = ManagedDisplay::with_buffers(
    ///     my_driver,
    ///     ColorFormat::Rgb565,
    ///     RenderMode::Partial,
    ///     false,
    /// ).unwrap();
    /// ```
    pub fn with_buffers(
        driver: D,
        color_format: ColorFormat,
        render_mode: RenderMode,
        double_buffer: bool,
    ) -> Option<Self> {
        let (width, height) = driver.size();
        let display = Display::new(width, height)?;

        // Calculate buffer size
        let buf_size = (width as usize) * (height as usize) * color_format.bytes_per_pixel();

        // Allocate buffers
        let mut buf1 = alloc::vec![0u8; buf_size];
        let mut buf2 = if double_buffer {
            Some(alloc::vec![0u8; buf_size])
        } else {
            None
        };

        // Box the driver so it has a stable address
        let driver = Box::new(driver);

        // Store pointer to driver in display's user_data
        let driver_ptr = &*driver as *const D as *mut core::ffi::c_void;
        unsafe {
            neo_lvgl_sys::lv_display_set_user_data(display.raw(), driver_ptr);
        }

        // Set up buffers - we pass raw pointers since we're keeping the Vecs alive
        let buf2_ptr = buf2
            .as_mut()
            .map(|b| b.as_mut_ptr())
            .unwrap_or(core::ptr::null_mut());
        unsafe {
            neo_lvgl_sys::lv_display_set_buffers(
                display.raw(),
                buf1.as_mut_ptr() as *mut _,
                buf2_ptr as *mut _,
                buf_size as u32,
                render_mode.to_raw(),
            );
        }

        // Set color format
        display.set_color_format(color_format);

        // Set up the flush callback trampoline
        unsafe {
            neo_lvgl_sys::lv_display_set_flush_cb(display.raw(), Some(Self::flush_trampoline));
        }

        Some(Self {
            display,
            driver,
            _buf1: Some(buf1),
            _buf2: buf2,
        })
    }

    /// Create a managed display with static buffers you provide.
    ///
    /// Use this when you need precise control over buffer allocation,
    /// such as placing buffers in specific memory regions (DMA-capable, etc.).
    ///
    /// # Arguments
    ///
    /// * `driver` - Your display driver implementation
    /// * `buf1` - Primary render buffer
    /// * `buf2` - Optional secondary buffer for double-buffering
    /// * `render_mode` - The rendering mode to use
    ///
    /// # Safety
    ///
    /// The buffers must remain valid for the lifetime of the display.
    pub unsafe fn from_static_buffers(
        driver: D,
        buf1: &'static mut [u8],
        buf2: Option<&'static mut [u8]>,
        render_mode: RenderMode,
    ) -> Option<Self> {
        let (width, height) = driver.size();
        let display = Display::new(width, height)?;

        // Box the driver so it has a stable address
        let driver = Box::new(driver);

        // Store pointer to driver in display's user_data
        let driver_ptr = &*driver as *const D as *mut core::ffi::c_void;
        neo_lvgl_sys::lv_display_set_user_data(display.raw(), driver_ptr);

        // Set up buffers
        display.set_buffers(buf1, buf2, render_mode);

        // Set up the flush callback trampoline
        neo_lvgl_sys::lv_display_set_flush_cb(display.raw(), Some(Self::flush_trampoline));

        Some(Self {
            display,
            driver,
            _buf1: None,
            _buf2: None,
        })
    }

    /// Create a managed display with static buffers and a specific color format.
    ///
    /// # Safety
    ///
    /// The buffers must remain valid for the lifetime of the display.
    pub unsafe fn from_static_buffers_with_format(
        driver: D,
        buf1: &'static mut [u8],
        buf2: Option<&'static mut [u8]>,
        render_mode: RenderMode,
        color_format: ColorFormat,
    ) -> Option<Self> {
        let managed = Self::from_static_buffers(driver, buf1, buf2, render_mode)?;
        managed.display.set_color_format(color_format);
        Some(managed)
    }

    /// Get a reference to the underlying display.
    pub fn display(&self) -> &Display {
        &self.display
    }

    /// Get a reference to the driver.
    pub fn driver(&self) -> &D {
        &self.driver
    }

    /// Get a mutable reference to the driver.
    pub fn driver_mut(&mut self) -> &mut D {
        &mut self.driver
    }

    /// Set this display as the default.
    pub fn set_default(&self) {
        self.display.set_default();
    }

    /// Get the currently active screen for this display.
    pub fn active_screen(&self) -> crate::widgets::Screen<'_> {
        self.display.active_screen()
    }

    /// Get display width.
    pub fn width(&self) -> i32 {
        self.display.width()
    }

    /// Get display height.
    pub fn height(&self) -> i32 {
        self.display.height()
    }

    /// Set the color format.
    pub fn set_color_format(&self, format: ColorFormat) {
        self.display.set_color_format(format);
    }

    /// Get the current color format.
    pub fn color_format(&self) -> ColorFormat {
        self.display.color_format()
    }

    /// The C trampoline that calls our Rust driver
    unsafe extern "C" fn flush_trampoline(
        disp: *mut neo_lvgl_sys::lv_display_t,
        area: *const neo_lvgl_sys::lv_area_t,
        px_map: *mut u8,
    ) {
        // Get driver pointer from user_data
        let driver_ptr = neo_lvgl_sys::lv_display_get_user_data(disp) as *mut D;
        if driver_ptr.is_null() {
            return;
        }

        let driver = &mut *driver_ptr;
        let area_ref = &*area;
        let rust_area = Area::from_raw(area_ref);

        // Calculate buffer size based on area and color format
        let color_format = ColorFormat::from_raw(neo_lvgl_sys::lv_display_get_color_format(disp));
        let pixel_count = rust_area.pixel_count();
        let buf_size = pixel_count * color_format.bytes_per_pixel();

        let pixels = core::slice::from_raw_parts(px_map, buf_size);

        // Call the Rust driver
        driver.flush(&rust_area, pixels);

        // Signal flush complete
        neo_lvgl_sys::lv_display_flush_ready(disp);
    }
}

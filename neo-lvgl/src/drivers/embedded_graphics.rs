//! embedded-graphics integration
//!
//! This module provides integration with the embedded-graphics ecosystem,
//! allowing LVGL to render to any display that implements `DrawTarget`.

use crate::display::{Area, DisplayDriver};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    pixelcolor::{
        raw::RawU16, Bgr565, Bgr666, Bgr888, Gray8, PixelColor, Rgb565, Rgb666, Rgb888, RgbColor,
    },
    Pixel,
};

/// Trait for converting LVGL's RGB565 pixel data to other color formats.
///
/// LVGL outputs pixels in RGB565 format by default. This trait allows
/// the embedded-graphics driver to support displays with different color depths.
pub trait FromLvglRgb565: Sized {
    /// Convert from LVGL's little-endian RGB565 raw value.
    fn from_rgb565_raw(raw: u16) -> Self;
}

impl FromLvglRgb565 for Rgb565 {
    #[inline]
    fn from_rgb565_raw(raw: u16) -> Self {
        Rgb565::from(RawU16::new(raw))
    }
}

impl FromLvglRgb565 for Bgr565 {
    #[inline]
    fn from_rgb565_raw(raw: u16) -> Self {
        let rgb = Rgb565::from(RawU16::new(raw));
        Bgr565::new(rgb.r(), rgb.g(), rgb.b())
    }
}

impl FromLvglRgb565 for Rgb888 {
    #[inline]
    fn from_rgb565_raw(raw: u16) -> Self {
        let rgb = Rgb565::from(RawU16::new(raw));
        // Expand 5/6/5 bits to 8/8/8 bits
        let r = (rgb.r() << 3) | (rgb.r() >> 2);
        let g = (rgb.g() << 2) | (rgb.g() >> 4);
        let b = (rgb.b() << 3) | (rgb.b() >> 2);
        Rgb888::new(r, g, b)
    }
}

impl FromLvglRgb565 for Bgr888 {
    #[inline]
    fn from_rgb565_raw(raw: u16) -> Self {
        let rgb = Rgb565::from(RawU16::new(raw));
        // Expand 5/6/5 bits to 8/8/8 bits
        let r = (rgb.r() << 3) | (rgb.r() >> 2);
        let g = (rgb.g() << 2) | (rgb.g() >> 4);
        let b = (rgb.b() << 3) | (rgb.b() >> 2);
        Bgr888::new(r, g, b)
    }
}

impl FromLvglRgb565 for Rgb666 {
    #[inline]
    fn from_rgb565_raw(raw: u16) -> Self {
        let rgb = Rgb565::from(RawU16::new(raw));
        // Expand 5/6/5 bits to 6/6/6 bits
        let r = (rgb.r() << 1) | (rgb.r() >> 4);
        let g = rgb.g(); // Already 6 bits
        let b = (rgb.b() << 1) | (rgb.b() >> 4);
        Rgb666::new(r, g, b)
    }
}

impl FromLvglRgb565 for Bgr666 {
    #[inline]
    fn from_rgb565_raw(raw: u16) -> Self {
        let rgb = Rgb565::from(RawU16::new(raw));
        // Expand 5/6/5 bits to 6/6/6 bits
        let r = (rgb.r() << 1) | (rgb.r() >> 4);
        let g = rgb.g(); // Already 6 bits
        let b = (rgb.b() << 1) | (rgb.b() >> 4);
        Bgr666::new(r, g, b)
    }
}

impl FromLvglRgb565 for Gray8 {
    #[inline]
    fn from_rgb565_raw(raw: u16) -> Self {
        let rgb = Rgb565::from(RawU16::new(raw));
        // Convert to grayscale using standard luminance formula
        // Y = 0.299*R + 0.587*G + 0.114*B (approximated with integer math)
        let r = (rgb.r() << 3) | (rgb.r() >> 2);
        let g = (rgb.g() << 2) | (rgb.g() >> 4);
        let b = (rgb.b() << 3) | (rgb.b() >> 2);
        let luma = ((r as u16 * 77) + (g as u16 * 150) + (b as u16 * 29)) >> 8;
        Gray8::new(luma as u8)
    }
}

/// Wrapper to use an embedded-graphics `DrawTarget` as an LVGL display driver.
///
/// This wrapper supports any color format that implements [`FromLvglRgb565`],
/// including RGB565, RGB888, BGR variants, and grayscale.
///
/// # Example
///
/// ```ignore
/// use lvgl::drivers::EmbeddedGraphicsDisplay;
/// use ili9341::Ili9341;  // RGB565 display
///
/// let display = Ili9341::new(spi, dc, cs).unwrap();
/// let lvgl_display = EmbeddedGraphicsDisplay::new(display);
/// ```
///
/// ```ignore
/// use lvgl::drivers::EmbeddedGraphicsDisplay;
/// use st7789::ST7789;  // RGB666 display
///
/// let display = ST7789::new(spi, dc, rst).unwrap();
/// let lvgl_display = EmbeddedGraphicsDisplay::new(display);
/// ```
pub struct EmbeddedGraphicsDisplay<T> {
    target: T,
}

impl<T> EmbeddedGraphicsDisplay<T> {
    /// Create a new embedded-graphics display wrapper.
    pub fn new(target: T) -> Self {
        Self { target }
    }

    /// Get a reference to the underlying display.
    pub fn inner(&self) -> &T {
        &self.target
    }

    /// Get a mutable reference to the underlying display.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.target
    }

    /// Consume the wrapper and return the underlying display.
    pub fn into_inner(self) -> T {
        self.target
    }
}

impl<T, C> DisplayDriver for EmbeddedGraphicsDisplay<T>
where
    T: DrawTarget<Color = C> + Dimensions,
    C: FromLvglRgb565 + PixelColor,
{
    fn size(&self) -> (i32, i32) {
        let size = self.target.bounding_box().size;
        (size.width as i32, size.height as i32)
    }

    fn flush(&mut self, area: &Area, pixels: &[u8]) {
        let width = area.width() as usize;

        // Convert LVGL RGB565 pixels to target color format and draw
        for y in area.y1..=area.y2 {
            for x in area.x1..=area.x2 {
                let idx = ((y - area.y1) as usize * width + (x - area.x1) as usize) * 2;
                if idx + 1 < pixels.len() {
                    // LVGL uses little-endian RGB565
                    let raw = u16::from_le_bytes([pixels[idx], pixels[idx + 1]]);
                    let color = C::from_rgb565_raw(raw);

                    // Ignore draw errors (embedded-graphics pattern)
                    let _ = self
                        .target
                        .draw_iter([Pixel(Point::new(x as i32, y as i32), color)]);
                }
            }
        }
    }
}

/// Extension trait for creating LVGL-compatible displays from embedded-graphics targets.
pub trait IntoLvglDisplay<C>: DrawTarget<Color = C> + Dimensions + Sized
where
    C: FromLvglRgb565 + PixelColor,
{
    /// Convert this display into an LVGL-compatible display driver.
    fn into_lvgl_display(self) -> EmbeddedGraphicsDisplay<Self> {
        EmbeddedGraphicsDisplay::new(self)
    }
}

impl<T, C> IntoLvglDisplay<C> for T
where
    T: DrawTarget<Color = C> + Dimensions,
    C: FromLvglRgb565 + PixelColor,
{
}

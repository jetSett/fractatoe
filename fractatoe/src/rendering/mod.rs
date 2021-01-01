use crate::image::Image;

pub mod flame_rendering;
pub mod gaussian_rendering;
pub mod mandelbrot_rendering;

use crate::fractals::histogram::Histogram;

pub trait HistogramRendering {
    fn render_image(self, histogram: Histogram) -> Image;
}

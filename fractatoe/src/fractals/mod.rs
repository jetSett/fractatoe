use crate::rendering::Histogram;

pub mod flame;
pub mod julia;
pub mod mandelbrot;

pub trait HistogramGeneration {
    fn build_histogram(self) -> Histogram;
}

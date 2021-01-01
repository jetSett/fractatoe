pub mod flame;
pub mod histogram;
pub mod julia;
pub mod mandelbrot;

pub trait HistogramGeneration {
    fn build_histogram(self, builder: histogram::HistogramBuilder) -> histogram::Histogram;
}

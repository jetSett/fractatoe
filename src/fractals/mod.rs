use serde_derive::{Deserialize, Serialize};

use crate::rendering::Histogram;

pub mod flame;
pub mod julia;
pub mod mandelbrot;

pub trait HistogramGeneration {
    fn build_histogram(self: Self) -> Histogram;
}

use julia::JuliaConf;
use mandelbrot::MandelbrotConf;

#[derive(Serialize, Deserialize)]
pub enum FractalConf {
    Mandelbrot(MandelbrotConf),
    Julia(JuliaConf),
    RenderingOnly(String),
}

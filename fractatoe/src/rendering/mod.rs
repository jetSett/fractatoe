use crate::image::Image;
use serde_derive::{Deserialize, Serialize};

pub mod flame_rendering;
pub mod gaussian_rendering;
pub mod mandelbrot_rendering;

pub type F64Color = f64;
pub type HistogramCell = (f64, F64Color);

#[derive(Deserialize, Serialize)]
pub enum FrequencyAggregationType {
    Linear,
    Logarithmic,
}

#[derive(Deserialize, Serialize)]
pub struct Histogram {
    width: usize,
    height: usize,
    resolution: usize,
    data: Vec<HistogramCell>,
}

impl Histogram {
    pub fn new(width: usize, height: usize, resolution: usize) -> Self {
        let mut data: Vec<HistogramCell> = Vec::new();
        data.resize(width * height * resolution * resolution, (0., 0.));
        Histogram {
            width,
            height,
            resolution,
            data,
        }
    }
    pub fn get_cell(&self, x: usize, y: usize) -> HistogramCell {
        self.data[x + self.width * self.resolution * y]
    }
    pub fn set_cell(&mut self, x: usize, y: usize, cell: HistogramCell) {
        self.data[x + self.width * self.resolution * y] = cell;
    }
    fn reduce_resolution(&mut self, freq_agreg_type: FrequencyAggregationType) {
        let mut pixel_cumul: Vec<HistogramCell> = vec![];

        // Accumulation tab for the pixels
        pixel_cumul.resize(self.width * self.height, (0., 0.));

        // for each virtual pixel
        for x in 0..(self.width * self.resolution) {
            for y in 0..(self.height * self.resolution) {
                // Take the frequence + color for the current virtual pixel
                let (freq, color) = self.data[x + self.width * self.resolution * y];

                // Find the associated real pixel (just divide each coordinate by resolution)
                let avg_point = (x / self.resolution, y / self.resolution);

                // sum with existing
                let (mut freq_sum, mut color_sum) =
                    pixel_cumul[avg_point.0 + avg_point.1 * self.width];

                freq_sum += freq;
                color_sum += color;

                pixel_cumul[avg_point.0 + avg_point.1 * self.width] = (freq_sum, color_sum)
            }
        }

        self.resolution = 1;

        // Now make the average for every pixels and compute the maximal frequency
        let mut max_freq: f64 = 0.;
        for x in 0..(self.width) {
            for y in 0..(self.height) {
                let index = x + y * self.width;
                let (freq_sum, mut color_sum) = pixel_cumul[index];
                let resolution_sq = (self.resolution * self.resolution) as f64;
                color_sum /= resolution_sq;

                if max_freq < freq_sum as f64 {
                    max_freq = freq_sum as f64;
                }

                pixel_cumul[index] = (freq_sum, color_sum);
            }
        }
        println!("Max freq : {}", max_freq);
        // Aggregate the frequences
        for x in 0..(self.width) {
            for y in 0..(self.height) {
                let index = x + y * self.width;
                let (mut freq, color) = pixel_cumul[index];
                freq = match freq_agreg_type {
                    FrequencyAggregationType::Linear => (freq / max_freq),
                    FrequencyAggregationType::Logarithmic => (freq.log(2.) / max_freq.log(2.)),
                };
                pixel_cumul[index] = (freq, color);
            }
        }
        self.data = pixel_cumul;
    }
}

pub trait HistogramRendering {
    fn render_image(self, histogram: Histogram) -> Image;
}

pub use flame_rendering::FlameRendererConf;
pub use gaussian_rendering::GaussianRendererConf;
pub use mandelbrot_rendering::GreyscaleRendererConf;
pub use mandelbrot_rendering::MandelbrotRendererConf;

#[derive(Serialize, Deserialize)]
pub enum RenderingConf {
    MandelbrotRendering(MandelbrotRendererConf),
    FlameRendering(FlameRendererConf),
    GaussianRendering(GaussianRendererConf),
    GreyscaleRendering(GreyscaleRendererConf),
}

use crate::window::{Image, Pix};
use num::complex::Complex;

use super::{FrequencyAggregationType, Histogram, HistogramGeneration, HistogramRendering};

pub struct Mandelbrot {
    pub width: usize,
    pub height: usize,
    pub scaling: f64,
    pub resolution: usize,
    pub bound: f64,
    pub iterations: usize,
}

fn mandelbrot_divergence(x: f64, y: f64, bound: f64, iterations: usize) -> usize {
    let mut z = Complex::new(0., 0.);

    let c = Complex::new(x, y);

    for i in 0..iterations {
        if z.norm() > bound {
            return i;
        }
        z = z * z + c;
    }
    return 0;
}
impl HistogramGeneration for Mandelbrot {
    fn build_histogram(self) -> Histogram {
        let mut histogram = Histogram::new(self.width, self.height, self.resolution);

        for i in 0..(self.width * self.resolution) {
            for j in 0..(self.height * self.resolution) {
                let x = self.scaling / (self.resolution as f64)
                    * ((i as f64) - (self.width as f64) / 2.);
                let y = self.scaling / (self.resolution as f64)
                    * ((j as f64) - (self.height as f64) / 2.);

                let div = mandelbrot_divergence(x, y, self.bound, self.iterations);
                histogram.set_cell(i, j, (div as f64, (255., 255., 255.)))
            }
        }
        histogram
    }
}

pub struct MandelbrotRenderer {
    pub r: usize,
    pub g: usize,
    pub b: usize,
    pub gamma: f64,
}

impl HistogramRendering for MandelbrotRenderer {
    fn render_image(self: Self, mut histogram: Histogram) -> Image {
        histogram.reduce_resolution(FrequencyAggregationType::Linear);
        let mut image = Image::new(histogram.width, histogram.height);

        for x in 0..histogram.width {
            for y in 0..histogram.height {
                let (freq, _) = histogram.get_cell(x, y);
                let pix = Pix {
                    r: ((self.r as f64) * (freq as f64).powf(self.gamma)) as u8,
                    g: ((self.g as f64) * (freq as f64).powf(self.gamma)) as u8,
                    b: ((self.b as f64) * (freq as f64).powf(self.gamma)) as u8,
                    alpha: 0xff,
                };
                image.set_pixel(x, y, pix);
            }
        }

        image
    }
}

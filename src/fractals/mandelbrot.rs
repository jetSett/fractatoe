use num::complex::Complex;
use serde_derive::{Deserialize, Serialize};

use super::{Histogram, HistogramGeneration};

#[derive(Serialize, Deserialize)]
pub struct MandelbrotConf {
    pub width: usize,
    pub height: usize,
    pub scaling: f64,
    pub resolution: usize,
    pub bound: f64,
    pub iterations: usize,
}

impl MandelbrotConf {
    pub fn build(self) -> Mandelbrot {
        Mandelbrot {
            width: self.width,
            height: self.height,
            scaling: self.scaling,
            resolution: self.resolution,
            bound: self.bound,
            iterations: self.iterations,
        }
    }
}

pub struct Mandelbrot {
    width: usize,
    height: usize,
    scaling: f64,
    resolution: usize,
    bound: f64,
    iterations: usize,
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

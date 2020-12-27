use num::complex::Complex;
use serde_derive::{Deserialize, Serialize};

use super::{Histogram, HistogramGeneration};

#[derive(Serialize, Deserialize)]
pub struct JuliaConf {
    pub width: usize,
    pub height: usize,
    pub c: (f64, f64),
    pub scaling: f64,
    pub resolution: usize,
    pub bound: f64,
    pub iterations: usize,
}

impl JuliaConf {
    pub fn build(self) -> Julia {
        Julia {
            width: self.width,
            height: self.height,
            scaling: self.scaling,
            c: Complex::new(self.c.0, self.c.1),
            resolution: self.resolution,
            bound: self.bound,
            iterations: self.iterations,
        }
    }
}

pub struct Julia {
    width: usize,
    height: usize,
    c: Complex<f64>,
    scaling: f64,
    resolution: usize,
    bound: f64,
    iterations: usize,
}

fn julia_divergence(x: f64, y: f64, c: Complex<f64>, bound: f64, iterations: usize) -> f64 {
    let mut z = Complex::new(x, y);

    for i in 0..iterations {
        if z.norm() > bound {
            return i as f64;
        }
        z = z * z + c;
    }
    0.
}

impl HistogramGeneration for Julia {
    fn build_histogram(self) -> Histogram {
        let mut histogram = Histogram::new(self.width, self.height, self.resolution);

        for i in 0..(self.width * self.resolution) {
            for j in 0..(self.height * self.resolution) {
                let x_float =
                    (2. * (i as f64 / ((self.width * self.resolution) as f64)) - 1.) / self.scaling;
                let y_float = (2. * (j as f64 / ((self.height * self.resolution) as f64)) - 1.)
                    / self.scaling;

                let div = julia_divergence(x_float, y_float, self.c, self.bound, self.iterations);
                histogram.set_cell(i, j, (div as f64, 0.))
            }
        }
        histogram
    }
}

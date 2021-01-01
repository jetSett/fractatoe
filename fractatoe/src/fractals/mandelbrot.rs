use num::complex::Complex;
use serde_derive::{Deserialize, Serialize};

use super::histogram::{Histogram, HistogramBuilder};
use super::HistogramGeneration;

#[derive(Serialize, Deserialize)]
pub struct Mandelbrot {
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
    0
}
impl HistogramGeneration for Mandelbrot {
    fn build_histogram(self, builder: HistogramBuilder) -> Histogram {
        let mut histogram =
            Histogram::new(builder.width_px, builder.height_px, builder.resolution_px);

        for ((i, j), (x_float, y_float)) in builder.iter_over_pixels() {
            let div = mandelbrot_divergence(x_float, y_float, self.bound, self.iterations);
            histogram.set_cell(i, j, (div as f64, 0.))
        }

        histogram
    }
}

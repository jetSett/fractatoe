use super::{
    histogram::{Histogram, HistogramBuilder},
    HistogramGeneration,
};
use num::complex::Complex;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Julia {
    c: (f64, f64),
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
    fn build_histogram(self, builder: HistogramBuilder) -> Histogram {
        let mut histogram =
            Histogram::new(builder.width_px, builder.height_px, builder.resolution_px);
        let c = Complex::new(self.c.0, self.c.1);
        for ((i, j), (x_float, y_float)) in builder.iter_over_pixels() {
            let div = julia_divergence(x_float, y_float, c, self.bound, self.iterations);
            histogram.set_cell(i, j, (div as f64, 0.))
        }

        histogram
    }
}

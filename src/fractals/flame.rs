use serde_derive::{Deserialize, Serialize};

use rand::distributions::weighted::WeightedIndex;
use rand::{Rng, SeedableRng};

use super::HistogramGeneration;
use crate::rendering::{F64Color, Histogram};

type FlameRng = rand::rngs::StdRng;

fn float_point_to_int_point(
    (x, y): (f64, f64),
    width: usize,
    height: usize,
    resolution: usize,
) -> (usize, usize) {
    (
        (x.min(1.) * (width * resolution) as f64).max(1.) as usize - 1,
        (y.min(1.) * (height * resolution) as f64).max(1.) as usize - 1,
    )
}

pub type FlameDistribution = WeightedIndex<u8>;
// a_j, b_j, c_j, d_j, e_j, f_j j=1...n
type CoefFlame = (f64, f64, f64, f64, f64, f64);

type FlamePoint = ((f64, f64), F64Color);

type FlameFunction = Box<dyn Fn(f64, f64) -> (f64, f64)>;

fn bisin(x: f64, y: f64) -> (f64, f64) {
    (x.sin(), y.sin())
}

fn linear(x: f64, y: f64) -> (f64, f64) {
    (x, y)
}

fn spherical(x: f64, y: f64) -> (f64, f64) {
    let norm = x * x + y * y;
    (x / norm, y / norm)
}

#[derive(Serialize, Deserialize)]
pub enum VariationFunction {
    Bisin,
    Linear,
    Spherical,
}

#[derive(Serialize, Deserialize)]
pub struct FlameConf {
    variation_functions: Vec<VariationFunction>,
    flame_distribution: Vec<u8>,
    weight_variation: Vec<f64>,
    coefs_inside: Vec<CoefFlame>,
    width: usize,
    height: usize,
    resolution: usize,
    number_points: usize,
    number_iterations: usize,

    seed: [u8; 32],
}
impl FlameConf {
    pub fn build(self) -> FlameAlgorithm {
        let mut variation_functions = vec![];

        let rng = FlameRng::from_seed(self.seed);

        for funct in self.variation_functions {
            let funct: FlameFunction = match funct {
                VariationFunction::Bisin => box bisin,
                VariationFunction::Linear => box linear,
                VariationFunction::Spherical => box spherical,
            };
            variation_functions.push(funct);
        }

        let flame_distribution = FlameDistribution::new(&self.flame_distribution)
            .expect("Flame function distribution not computable");

        FlameAlgorithm {
            variation_functions,
            flame_distribution: flame_distribution,
            weight_variation: self.weight_variation,
            coefs_inside: self.coefs_inside,
            width: self.width,
            height: self.height,
            resolution: self.resolution,
            number_points: self.number_points,
            number_iterations: self.number_iterations,

            rng,
        }
    }
}

pub struct FlameAlgorithm {
    variation_functions: Vec<FlameFunction>,
    flame_distribution: FlameDistribution,
    weight_variation: Vec<f64>,
    coefs_inside: Vec<CoefFlame>,

    number_points: usize,
    number_iterations: usize,

    width: usize,
    height: usize,
    resolution: usize,

    rng: FlameRng,
}

impl HistogramGeneration for FlameAlgorithm {
    fn build_histogram(mut self) -> Histogram {
        let mut histogram = Histogram::new(self.width, self.height, self.resolution);
        for _ in 0..self.number_points {
            let mut point: FlamePoint = self.rng.gen();
            for _ in 0..self.number_iterations {
                self.add_point_to_histogram(point, &mut histogram);
                point = self.one_round(point);
            }
        }
        histogram
    }
}

impl FlameAlgorithm {
    fn one_round(&mut self, point: FlamePoint) -> FlamePoint {
        let (mut x_current, mut y_current) = (0., 0.);
        let (x, y) = point.0;
        let color = point.1;

        let j = self.rng.sample(&self.flame_distribution);

        let m = self.variation_functions.len();

        for k in 0..(m) {
            let (a, b, c, d, e, f) = self.coefs_inside[j];
            let (x_translate, y_translate) =
                self.variation_functions[k](a * x + b * y + c, d * x + e * y + f);

            x_current += self.weight_variation[k] * x_translate;
            y_current += self.weight_variation[k] * y_translate;
        }

        ((x_current, y_current), color)
    }

    fn add_point_to_histogram(&mut self, point: FlamePoint, histogram: &mut Histogram) {
        let (x, y) = float_point_to_int_point(point.0, self.width, self.height, self.resolution);
        let (mut freq, (mut r, mut g, mut b)) = histogram.get_cell(x, y);
        freq += 1.;
        if r == 0. {
            r = (point.1).0;
        } else {
            r = (r + (point.1).0) / 2.;
        }
        if g == 0. {
            g = (point.1).0;
        } else {
            g = (g + (point.1).0) / 2.;
        }
        if b == 0. {
            b = (point.1).0;
        } else {
            b = (b + (point.1).0) / 2.;
        }
        histogram.set_cell(x, y, (freq, (r, g, b)));
    }
}

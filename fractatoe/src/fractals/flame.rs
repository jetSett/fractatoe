use serde_derive::{Deserialize, Serialize};

use rand_seeder::Seeder;

use rand::distributions::weighted::WeightedIndex;
use rand::Rng;

use super::HistogramGeneration;
use crate::rendering::{F64Color, Histogram};

type FlameRng = rand::rngs::StdRng;

fn float_point_to_int_point(
    (x, y): (f64, f64),
    width: usize,
    height: usize,
    resolution: usize,
) -> (i32, i32) {
    (
        (x * (width * resolution) as f64) as i32 - 1,
        (y * (height * resolution) as f64) as i32 - 1,
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

fn julia(x: f64, y: f64) -> (f64, f64) {
    let theta: f64 = (x / y).atan();
    let sqrt_r = (x * x + y * y).sqrt().sqrt();
    (sqrt_r * (theta / 2.).cos(), sqrt_r * (theta / 2.).sin())
}

fn swirl(x: f64, y: f64) -> (f64, f64) {
    let r_sq: f64 = x * x + y * y;
    (
        x * (r_sq).sin() - y * (r_sq).cos(),
        x * (r_sq).cos() + y * (r_sq).sin(),
    )
}
#[derive(Serialize, Deserialize)]
pub enum VariationFunction {
    Bisin,
    Linear,
    Spherical,
    Julia,
    Swirl,
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
    iteration_offset: usize,
    number_iterations: usize,

    seed: String,
}
impl FlameConf {
    pub fn build(self) -> FlameAlgorithm {
        let mut variation_functions = vec![];

        let rng = Seeder::from(self.seed).make_rng();

        for funct in self.variation_functions {
            let funct: FlameFunction = match funct {
                VariationFunction::Bisin => box bisin,
                VariationFunction::Linear => box linear,
                VariationFunction::Spherical => box spherical,
                VariationFunction::Julia => box julia,
                VariationFunction::Swirl => box swirl,
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
            iteration_offset: self.iteration_offset,

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
    iteration_offset: usize,

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
            for _ in 0..self.iteration_offset {
                point = self.one_round(point);
            }
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
        if 0 <= x
            && x < (self.width * self.resolution) as i32
            && 0 <= y
            && y < (self.height * self.resolution) as i32
        {
            let (mut freq, mut color) = histogram.get_cell(x as usize, y as usize);
            freq += 1.;
            color = (color + point.1) / 2.;

            histogram.set_cell(x as usize, y as usize, (freq, color));
        }
    }
}

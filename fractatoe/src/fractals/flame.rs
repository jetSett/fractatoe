use serde_derive::{Deserialize, Serialize};

use rand_seeder::Seeder;

use rand::distributions::weighted::WeightedIndex;
use rand::Rng;

use super::HistogramGeneration;
use crate::fractals::histogram::{F64Color, Histogram, HistogramBuilder};

type FlameRng = rand::rngs::StdRng;

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

    number_points: usize,
    iteration_offset: usize,
    number_iterations: usize,

    seed: String,
}
impl FlameConf {
    pub fn build(self) -> FlameAlgorithm {
        let mut variation_functions = vec![];

        let rng: FlameRng = Seeder::from(self.seed).make_rng();

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
            flame_distribution,
            weight_variation: self.weight_variation,
            coefs_inside: self.coefs_inside,
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

    rng: FlameRng,
}

impl HistogramGeneration for FlameAlgorithm {
    fn build_histogram(mut self, builder: HistogramBuilder) -> Histogram {
        let (x0, y0) = builder.point_top_left();
        let uniform_distrib_x =
            rand::distributions::uniform::Uniform::new(x0, x0 + builder.width_real);
        let uniform_distrib_y =
            rand::distributions::uniform::Uniform::new(y0, y0 + builder.height_real);
        let mut histogram =
            Histogram::new(builder.width_px, builder.height_px, builder.resolution_px);
        for _ in 0..self.number_points {
            // Sample a new point in the window
            let mut point: FlamePoint = (
                (
                    self.rng.sample(uniform_distrib_x),
                    self.rng.sample(uniform_distrib_y),
                ),
                self.rng.gen(),
            );
            // Make a few iteration to make an offset
            for _ in 0..self.iteration_offset {
                point = self.one_round(point);
            }

            for _ in 0..self.number_iterations {
                self.add_point_to_histogram(point, &mut histogram, &builder);
                point = self.one_round(point);
            }
        }
        histogram
    }
}

impl FlameAlgorithm {
    fn one_round(&mut self, point: FlamePoint) -> FlamePoint {
        let (mut x_current, mut y_current) = (0., 0.);
        let (x_point, y_point) = point.0;
        let color = point.1;

        let transformation_index = self.rng.sample(&self.flame_distribution);
        let coefs = self.coefs_inside[transformation_index];

        for (weight, variation_function) in self
            .weight_variation
            .iter()
            .zip(self.variation_functions.iter())
        {
            let (x_translate, y_translate) = variation_function(
                coefs.0 * x_point + coefs.1 * y_point + coefs.2,
                coefs.3 * x_point + coefs.4 * y_point + coefs.5,
            );

            x_current += weight * x_translate;
            y_current += weight * y_translate;
        }

        ((x_current, y_current), color)
    }

    fn add_point_to_histogram(
        &mut self,
        point: FlamePoint,
        histogram: &mut Histogram,
        builder: &HistogramBuilder,
    ) {
        let point_px = builder.real_to_pixel(point.0 .0, point.0 .1);
        if let Some((x, y)) = point_px {
            let (mut freq, mut color) = histogram.get_cell(x as usize, y as usize);
            freq += 1.;
            color = (color + point.1) / 2.;

            histogram.set_cell(x, y, (freq, color));
        }
    }
}

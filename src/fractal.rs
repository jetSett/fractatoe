pub mod mandelbrot {
    use crate::window::{Image, Pix};
    use num::complex::Complex;

    fn divergence(x: f64, y: f64, bound: f64, iterations: usize) -> f64 {
        let mut z = Complex::new(0., 0.);

        let c = Complex::new(x, y);

        for i in 0..iterations {
            if z.norm() > bound {
                return 1. / (i as f64).powf(0.3);
            }
            z = z * z + c;
        }
        return 0.;
    }

    pub fn create_image(
        width: usize,
        height: usize,
        scaling: f64,
        bound: f64,
        iterations: usize,
    ) -> Image {
        let mut image = Image::new(width, height);

        for i in 0..(width - 1) {
            for j in 0..(height - 1) {
                let x = scaling * ((i as f64) - (width as f64) / 2.);
                let y = scaling * ((j as f64) - (height as f64) / 2.);

                let div = divergence(x, y, bound, iterations);

                let pix = Pix {
                    r: (20. * div) as u8,
                    g: (0. * div) as u8,
                    b: (255. * div) as u8,
                    alpha: 0xff,
                };
                image.set_pixel(i as usize, j as usize, pix);
            }
        }
        image
    }
}

pub mod flame {
    use log::debug;

    use crate::window::{Image, Pix};
    use rand::distributions::weighted::WeightedIndex;
    use rand::Rng;

    fn float_point_to_int_point(
        (x, y): (f64, f64),
        width: usize,
        height: usize,
        resolution: usize,
    ) -> (usize, usize) {
        (
            (x * (width * resolution) as f64) as usize,
            (y * (height * resolution) as f64) as usize,
        )
    }

    pub type FlameDistribution = WeightedIndex<u8>;
    // a_j, b_j, c_j, d_j, e_j, f_j j=1...n
    type CoefFlame = (f64, f64, f64, f64, f64, f64);

    // Histogram cell : (frequency, color)
    // Should be u8 but will be summed at some point so u32 to prevent overflow
    type Color = (u32, u32, u32);
    type HistogramCell = (u32, Color);

    type FlamePoint = ((f64, f64), Color);

    pub type FlameFunction = Box<dyn Fn(f64, f64) -> (f64, f64)>;

    pub struct FlameAlgorithm {
        pub variation_functions: Vec<FlameFunction>,
        pub flame_distribution: FlameDistribution,
        pub weight_variation: Vec<f64>,
        pub coefs_inside: Vec<CoefFlame>,

        width: usize,
        height: usize,
        resolution: usize,

        histogram: Vec<HistogramCell>,
    }

    impl FlameAlgorithm {
        pub fn new(
            width: usize,
            height: usize,
            variation_functions: Vec<FlameFunction>,
            flame_distribution: FlameDistribution,
            weight_variation: Vec<f64>,
            coefs_inside: Vec<CoefFlame>,
            resolution: usize,
        ) -> Self {
            let mut histogram = vec![];
            histogram.resize(
                width * height * resolution * resolution,
                (0, (0x00, 0x00, 0x00)),
            );
            FlameAlgorithm {
                width,
                height,
                variation_functions,
                flame_distribution,
                weight_variation,
                coefs_inside,
                histogram,
                resolution,
            }
        }

        fn one_turn<RAND: Rng>(&self, point: FlamePoint, rng: &mut RAND) -> FlamePoint {
            let (mut x_current, mut y_current) = (0., 0.);
            let (x, y) = point.0;
            let color: Color = point.1;

            let j = rng.sample(&self.flame_distribution);

            let m = self.variation_functions.len();

            for k in 0..(m) {
                let (a, b, c, d, e, f) = self.coefs_inside[j];
                let (x_translate, y_translate) =
                    self.variation_functions[k](a * x + b * y + c, d * x + e * y + f);
                debug!("Translation : {:?}", (x_translate, y_translate));
                x_current += self.weight_variation[k] * x_translate;
                y_current += self.weight_variation[k] * y_translate;
            }

            ((x_current, y_current), color)
        }

        fn add_point_to_histogram(&mut self, point: FlamePoint) {
            debug!("Float points : {:?}", point.0);
            let (x, y) =
                float_point_to_int_point(point.0, self.width, self.height, self.resolution);
            debug!("integral points : {}, {}", x, y);
            let index_histogram = x + y * self.width * self.resolution;
            let (mut freq, (mut r, mut g, mut b)) = self.histogram[index_histogram];
            freq += 1;
            r = (r + (point.1).0) / 2;
            g = (g + (point.1).1) / 2;
            b = (b + (point.1).2) / 2;
            self.histogram[index_histogram] = (freq, (r, g, b));
        }

        pub fn compute_histogram<RAND: Rng>(&mut self, number_iterations: usize, mut rng: RAND) {
            let mut point: FlamePoint = (
                rng.gen(),
                (
                    rng.gen::<u32>() % 256,
                    rng.gen::<u32>() % 256,
                    rng.gen::<u32>() % 256,
                ),
            );
            for _ in 0..number_iterations {
                self.add_point_to_histogram(point);
                point = self.one_turn(point, &mut rng);
            }
        }

        pub fn render_image(self, gamma: f64) -> Image {
            let mut pixel_cumul: Vec<HistogramCell> = vec![];

            // Accumulation tab for the pixels
            pixel_cumul.resize(self.width * self.height, (0, (0x00, 0x00, 0x00)));

            // for each virtual pixel
            for x in 0..(self.width * self.resolution) {
                for y in 0..(self.height * self.resolution) {
                    // Take the frequence + color for the current virtual pixel
                    let (freq, (r, g, b)) = self.histogram[x + self.width * self.resolution * y];

                    // Find the associated real pixel (just divide each coordinate by resolution)
                    let avg_point = (x / self.resolution, y / self.resolution);

                    // sum with existing
                    let (mut freq_sum, (mut r_sum, mut g_sum, mut b_sum)) =
                        pixel_cumul[avg_point.0 + avg_point.1 * self.width];

                    freq_sum += freq;
                    r_sum += r;
                    g_sum += g;
                    b_sum += b;
                    pixel_cumul[avg_point.0 + avg_point.1 * self.width] =
                        (freq_sum, (r_sum, g_sum, b_sum))
                }
            }

            // Now make the average for every pixels and compute the maximal frequency
            let mut max_freq: f64 = 0.;
            for x in 0..(self.width) {
                for y in 0..(self.height) {
                    let index = x + y * self.width;
                    let (mut freq_sum, (mut r_sum, mut g_sum, mut b_sum)) = pixel_cumul[index];
                    let resolution_sq = (self.resolution * self.resolution) as u32;
                    freq_sum /= resolution_sq;
                    r_sum /= resolution_sq;
                    g_sum /= resolution_sq;
                    b_sum /= resolution_sq;

                    if max_freq < freq_sum as f64 {
                        max_freq = freq_sum as f64;
                    }

                    pixel_cumul[index] = (freq_sum, (r_sum, g_sum, b_sum));
                }
            }

            let max_freq_log = max_freq.log(2.);
            let mut image = Image::new(self.width, self.height);

            for x in 0..(self.width - 1) {
                for y in 0..(self.height - 1) {
                    let index = x + y * self.width;
                    let (freq, (r, g, b)) = pixel_cumul[index];
                    let r = r as u8;
                    let g = g as u8;
                    let b = b as u8;

                    let alpha = (((freq as f64).log(2.) / max_freq_log).powf(gamma) * 255.) as u8;

                    let pix = Pix { r, g, b, alpha };

                    image.set_pixel(x, y, pix)
                }
            }

            image
        }
    }
}

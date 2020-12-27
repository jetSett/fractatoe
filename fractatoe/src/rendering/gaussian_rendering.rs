use serde_derive::{Deserialize, Serialize};

use super::{FrequencyAggregationType, Histogram, HistogramRendering};

use crate::image::{Image, Pix};

#[derive(Serialize, Deserialize)]
pub struct GaussianColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,

    pub mean: f64,
    pub std_deviation: f64,

    pub scaling_factor: f64,
}

impl GaussianColor {
    fn error_function(_x: f64) -> f64 {
        1.
    }

    fn color_from_freq(&self, freq: f64) -> (f64, f64, f64) {
        let y = (freq - self.mean) / self.std_deviation;
        let exponent = -y * y;
        let reduction_factor = GaussianColor::error_function(1. / self.std_deviation)
            * std::f64::consts::PI.sqrt()
            * self.std_deviation
            / 2.;
        let gauss = exponent.exp() / reduction_factor;
        // dbg!(gauss);
        // assert!(gauss <= 1. && gauss >= 0.);
        let factor = gauss * self.scaling_factor;

        (factor * self.r, factor * self.g, factor * self.b)
    }
}

#[derive(Serialize, Deserialize)]
pub struct GaussianRendererConf {
    pub frequency_agreg_type: FrequencyAggregationType,
    pub gaussian_colors: Vec<GaussianColor>,

    pub gamma: f64,
}

impl GaussianRendererConf {
    pub fn build(self) -> GaussianRenderer {
        GaussianRenderer {
            frequency_agreg_type: self.frequency_agreg_type,
            gaussian_colors: self.gaussian_colors,
            gamma: self.gamma,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GaussianRenderer {
    frequency_agreg_type: FrequencyAggregationType,
    gaussian_colors: Vec<GaussianColor>,

    pub gamma: f64,
}

impl HistogramRendering for GaussianRenderer {
    fn render_image(self, mut histogram: Histogram) -> Image {
        histogram.reduce_resolution(self.frequency_agreg_type);
        let mut image = Image::new(histogram.width, histogram.height);

        for x in 0..(histogram.width - 1) {
            for y in 0..(histogram.height - 1) {
                let (mut freq, _) = histogram.get_cell(x, y);

                freq = freq.powf(self.gamma);

                let (mut r, mut g, mut b): (f64, f64, f64) = (0., 0., 0.);

                for gaussian_color in self.gaussian_colors.iter() {
                    let (r_gauss, g_gauss, b_gauss) = gaussian_color.color_from_freq(freq);
                    r += r_gauss;
                    g += g_gauss;
                    b += b_gauss;
                }

                let pix = Pix {
                    r: r.min(255.).max(0.) as u8,
                    g: g.min(255.).max(0.) as u8,
                    b: b.min(255.).max(0.) as u8,
                    alpha: 0xff,
                };

                image.set_pixel(x, y, pix)
            }
        }
        image
    }
}

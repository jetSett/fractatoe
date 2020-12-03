use serde_derive::{Deserialize, Serialize};

use super::{FrequencyAggregationType, Histogram, HistogramRendering};

use crate::window::{Image, Pix};

#[derive(Serialize, Deserialize)]
pub struct MandelbrotRendererConf {
    pub r: usize,
    pub g: usize,
    pub b: usize,
    pub gamma: f64,

    pub frequency_agreg_type: FrequencyAggregationType,
}

impl MandelbrotRendererConf {
    pub fn build(self) -> MandelbrotRenderer {
        MandelbrotRenderer {
            r: self.r,
            g: self.g,
            b: self.b,
            gamma: self.gamma,
            frequency_agreg_type: self.frequency_agreg_type,
        }
    }
}
pub struct MandelbrotRenderer {
    r: usize,
    g: usize,
    b: usize,
    gamma: f64,

    frequency_agreg_type: FrequencyAggregationType,
}

impl HistogramRendering for MandelbrotRenderer {
    fn render_image(self: Self, mut histogram: Histogram) -> Image {
        histogram.reduce_resolution(self.frequency_agreg_type);
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

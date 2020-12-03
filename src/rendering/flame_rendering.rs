use serde_derive::{Deserialize, Serialize};

use super::{FrequencyAggregationType, Histogram, HistogramRendering};

use crate::window::{Image, Pix};

#[derive(Serialize, Deserialize)]
pub struct FlameRendererConf {
    pub frequency_agreg_type: FrequencyAggregationType,
    pub gamma: f64,
}

impl FlameRendererConf {
    pub fn build(self) -> FlameRenderer {
        FlameRenderer {
            frequency_agreg_type: self.frequency_agreg_type,
            gamma: self.gamma,
        }
    }
}
pub struct FlameRenderer {
    frequency_agreg_type: FrequencyAggregationType,
    gamma: f64,
}

impl HistogramRendering for FlameRenderer {
    fn render_image(self, mut histogram: Histogram) -> Image {
        histogram.reduce_resolution(self.frequency_agreg_type);

        let mut image = Image::new(histogram.width, histogram.height);

        for x in 0..(histogram.width - 1) {
            for y in 0..(histogram.height - 1) {
                let (mut freq, (r, g, b)) = histogram.get_cell(x, y);

                freq = freq.powf(self.gamma);

                let r = ((r * freq) * 255.) as u8;
                let g = ((g * freq) * 255.) as u8;
                let b = ((b * freq) * 255.) as u8;

                let pix = Pix {
                    r,
                    g,
                    b,
                    alpha: 0xff,
                };

                image.set_pixel(x, y, pix)
            }
        }

        image
    }
}

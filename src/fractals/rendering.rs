use super::{FrequencyAggregationType, Histogram, HistogramRendering};
use crate::window::{Image, Pix};

pub struct MandelbrotRenderer {
    pub r: usize,
    pub g: usize,
    pub b: usize,
    pub gamma: f64,
}

impl HistogramRendering for MandelbrotRenderer {
    fn render_image(self: Self, mut histogram: Histogram) -> Image {
        histogram.reduce_resolution(FrequencyAggregationType::Linear);
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

use serde_derive::{Deserialize, Serialize};

use fractatoe::fractals::flame::FlameConf;
use fractatoe::fractals::histogram::HistogramBuilder;
use fractatoe::fractals::julia::Julia;
use fractatoe::fractals::mandelbrot::Mandelbrot;

#[derive(Serialize, Deserialize)]
pub enum FractalConf {
    Mandelbrot(Mandelbrot),
    Julia(Julia),
    Flame(FlameConf),
}

#[derive(Serialize, Deserialize)]
pub struct GenerationConf {
    pub histogram_conf: HistogramBuilder,
    pub fractal_conf: FractalConf,
}

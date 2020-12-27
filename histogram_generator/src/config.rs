use serde_derive::{Deserialize, Serialize};

use fractatoe::fractals::flame::FlameConf;
use fractatoe::fractals::julia::JuliaConf;
use fractatoe::fractals::mandelbrot::Mandelbrot;

#[derive(Serialize, Deserialize)]
pub enum FractalConf {
    Mandelbrot(Mandelbrot),
    Julia(JuliaConf),
    Flame(FlameConf),
    RenderingOnly(String),
}

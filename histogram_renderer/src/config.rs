use serde_derive::{Deserialize, Serialize};

pub use fractatoe::rendering::flame_rendering::FlameRendererConf;
pub use fractatoe::rendering::gaussian_rendering::GaussianRendererConf;
pub use fractatoe::rendering::mandelbrot_rendering::GreyscaleRendererConf;
pub use fractatoe::rendering::mandelbrot_rendering::MandelbrotRendererConf;

#[derive(Serialize, Deserialize)]
pub enum RenderingConf {
    MandelbrotRendering(MandelbrotRendererConf),
    FlameRendering(FlameRendererConf),
    GaussianRendering(GaussianRendererConf),
    GreyscaleRendering(GreyscaleRendererConf),
}

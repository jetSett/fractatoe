#![forbid(unsafe_code)]

use log::info;
use winit::dpi::PhysicalSize;

use argh::FromArgs;
#[derive(FromArgs)]
/// Greet
struct Args {
    #[argh(option, description = "scaling of the fractal")]
    scaling: Option<f64>,
}

mod fractal;
mod window;

fn main() -> Result<(), pixels::Error> {
    env_logger::init();

    let args: Args = argh::from_env();

    info!("Computing the image");
    let image = fractal::mandelbrot::create_image(600, 800, args.scaling.unwrap_or(0.01), 50., 100);

    info!("Starting main loop");

    window::show_image(PhysicalSize::new(600, 800), image)
}

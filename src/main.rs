#![forbid(unsafe_code)]
#![feature(trait_alias)]
#![feature(box_syntax)]

use argh::FromArgs;
use log::info;
use rand::{rngs::StdRng, SeedableRng};
use winit::dpi::PhysicalSize;
#[derive(FromArgs)]
/// Greet
struct Args {
    #[argh(positional, description = "width of the fractal")]
    width: usize,
    #[argh(positional, description = "height of the fractal")]
    height: usize,
    #[argh(positional, description = "number of iterations for the flame point")]
    number_iteration: usize,
    #[argh(positional, description = "resolution of the flame supersampling")]
    resolution: usize,
    #[argh(option, description = "resolution of the flame supersampling")]
    seed: Option<u64>,
    #[argh(switch, description = "resolution of the flame supersampling")]
    display: bool,
}

mod fractals;
mod window;

fn bisin(x: f64, y: f64) -> (f64, f64) {
    (y.sin(), x.sin())
}

fn bicos(x: f64, y: f64) -> (f64, f64) {
    (y.cos(), x.cos())
}

fn main() -> Result<(), &'static str> {
    use fractals::flame::FlameBuilder;
    use fractals::flame::FlameDistribution;

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let args: Args = argh::from_env();

    info!("Initializing the flame algorithm");

    let distribution = FlameDistribution::new(&[1, 1]).unwrap();

    let (width, height) = (args.width, args.height);

    let mut flame_algo = FlameBuilder::new()
        .with_size(width, height)
        .with_resolution(args.resolution)
        .with_variation_functions(vec![box bisin, box bicos])
        .with_flame_distribution(distribution)
        .with_weight_variation(vec![0.5, 0.5])
        .with_coefs_inside(vec![
            (1., 0.2, 0., 0.2, 1., 0.),
            (1., 1., 0.2, 2.3, 0.3, 0.),
        ])
        .build()?;

    info!("Computing the histogram");
    let rng = StdRng::seed_from_u64(args.seed.unwrap_or(4242));
    flame_algo.compute_histogram(args.number_iteration, rng);

    info!("Generating the image");
    let image = flame_algo.render_image(1.);

    info!("Starting main loop");
    if args.display {
        window::show_image(PhysicalSize::new(width as f32, height as f32), image).unwrap();
    }
    Ok(())
}

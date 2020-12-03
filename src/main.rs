#![forbid(unsafe_code)]
#![feature(trait_alias)]
#![feature(box_syntax)]

/*
    TODO :
    - Get params for a config file (toml) -> serdes
    - Allow to save histograms in order to make multiple rendering -> serdes
*/
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
    #[argh(positional, description = "number of points lauched for flame")]
    number_points: usize,
    #[argh(positional, description = "number of iterations for the flame point")]
    number_iterations: usize,
    #[argh(positional, description = "resolution of the flame supersampling")]
    resolution: usize,
    #[argh(option, description = "gamma reduction")]
    gamma: Option<f64>,
    #[argh(option, description = "resolution of the flame supersampling")]
    seed: Option<u64>,
    #[argh(switch, description = "resolution of the flame supersampling")]
    display: bool,
}

mod fractals;
mod window;

use fractals::{HistogramGeneration, HistogramRendering};

fn main() -> Result<(), &'static str> {
    use fractals::flame::FlameBuilder;
    use fractals::flame::FlameDistribution;
    use fractals::flame::VariationFunction;

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let args: Args = argh::from_env();

    info!("Initializing the flame algorithm");

    // let distribution = FlameDistribution::new(&[1, 1, 1]).unwrap();

    let (width, height) = (args.width, args.height);

    // let mut flame_algo = FlameBuilder::new()
    //     .with_size(width, height)
    //     .with_resolution(args.resolution)
    //     .with_variation_functions(vec![VariationFunction::Bisin, VariationFunction::Spherical])
    //     .with_flame_distribution(distribution)
    //     .with_weight_variation(vec![0.5, 0.5])
    //     .with_coefs_inside(vec![
    //         (0.9, 0.1, 0., 0.1, 0.9, 0.2),
    //         (0.9, 0.1, 0.5, 0.1, 0.9, 0.5),
    //         (1., 1., -0.3, 1., 1., 0.3),
    //     ])
    //     .build()?;

    // info!("Computing the histogram");
    // let rng = StdRng::seed_from_u64(args.seed.unwrap_or(4242));
    // flame_algo.compute_histogram(args.number_points, args.number_iterations, rng);

    info!("Generating the image");
    // let image = flame_algo.render_image(args.gamma.unwrap_or(1.));

    let mandelbroot_algo = fractals::mandelbrot::Mandelbrot {
        width: width,
        height: height,
        scaling: 0.01,
        resolution: 1,
        bound: 50.,
        iterations: 1000,
    };

    let histogram = mandelbroot_algo.build_histogram();

    let image = fractals::mandelbrot::MandelbrotRenderer {
        r: 255,
        g: 200,
        b: 10,
        gamma: 0.5,
    }
    .render_image(histogram);

    info!("Starting main loop");
    if args.display {
        window::show_image(PhysicalSize::new(width as f32, height as f32), image).unwrap();
    }
    Ok(())
}

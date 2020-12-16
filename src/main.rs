#![forbid(unsafe_code)]
#![feature(trait_alias)]
#![feature(box_syntax)]

use std::fs;
use std::io::Write;

use crate::fractals::HistogramGeneration;
use argh::FromArgs;
use serde_derive::{Deserialize, Serialize};
use winit::dpi::PhysicalSize;

use fractals::*;
use rendering::{Histogram, HistogramRendering, RenderingConf};

mod fractals;
mod image;
mod png_save;
mod rendering;
mod window;

use image::Image;

#[derive(FromArgs)]
/// Greet
struct Args {
    #[argh(positional)]
    config_filename: String,
    #[argh(option, description = "save the histogram to a file")]
    save_histogram: Option<String>,
    #[argh(option, description = "save the image to a file")]
    save_image: Option<String>,
    #[argh(switch, description = "do not show on screen")]
    no_show: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AppConf {
    fractal_conf: FractalConf,
    rendering_conf: rendering::RenderingConf,
}

fn get_histogram_from_fractal_conf(fractal_conf: fractals::FractalConf) -> Histogram {
    match fractal_conf {
        FractalConf::Mandelbrot(conf) => conf.build().build_histogram(),
        FractalConf::Julia(conf) => conf.build().build_histogram(),
        FractalConf::Flame(conf) => conf.build().build_histogram(),
        FractalConf::RenderingOnly(histo_filename) => {
            let histo_data =
                fs::read_to_string(histo_filename).expect("Could not read histogram file");
            serde_json::from_str(histo_data.as_str()).expect("Error in the histogram data")
        }
    }
}

fn render_image(rendering_conf: rendering::RenderingConf, histogram: Histogram) -> Image {
    match rendering_conf {
        RenderingConf::MandelbrotRendering(conf) => conf.build().render_image(histogram),
        RenderingConf::FlameRendering(conf) => conf.build().render_image(histogram),
        RenderingConf::GaussianRendering(conf) => conf.build().render_image(histogram),
    }
}

fn main() -> Result<(), &'static str> {
    env_logger::Builder::from_env(env_logger::Env::default()).init();

    let args: Args = argh::from_env();
    let json_config = fs::read_to_string(args.config_filename).expect("Could not read config file");

    let app_config: AppConf =
        serde_json::from_str(json_config.as_str()).expect("Error in the config file");

    let histogram = get_histogram_from_fractal_conf(app_config.fractal_conf);

    if let Some(filename) = args.save_histogram {
        let mut outfile = fs::File::create(filename).expect("Unable to open the histogram file");
        outfile
            .write(
                &serde_json::to_vec_pretty(&histogram)
                    .expect("Unable to convert the histogram in json file"),
            )
            .expect("Unable to write the histogram file");
    }

    if args.save_image.is_some() || !args.no_show {
        let image = render_image(app_config.rendering_conf, histogram);

        if let Some(image_path) = args.save_image {
            png_save::save_image(&image, image_path);
        }

        if !args.no_show {
            window::show_image(
                PhysicalSize::new(image.width as f32, image.height as f32),
                image,
            )
            .unwrap();
        }
    }

    Ok(())
}

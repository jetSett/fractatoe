#![forbid(unsafe_code)]
#![feature(trait_alias)]
#![feature(box_syntax)]

use std::fs;
use std::io::Write;

use argh::FromArgs;
use fractatoe::{fractals::HistogramGeneration, rendering::Histogram};

mod config;

#[derive(FromArgs)]
/// Arguments
struct Args {
    #[argh(positional)]
    config_filename: String,
    #[argh(positional, description = "save the histogram to a file")]
    output_histogram: String,
}

use config::FractalConf;

fn get_histogram_from_fractal_conf(fractal_conf: FractalConf) -> Histogram {
    match fractal_conf {
        FractalConf::Mandelbrot(generator) => generator.build_histogram(),
        FractalConf::Julia(conf) => conf.build().build_histogram(),
        FractalConf::Flame(conf) => conf.build().build_histogram(),
        FractalConf::RenderingOnly(histo_filename) => {
            let histo_data =
                fs::read_to_string(histo_filename).expect("Could not read histogram file");
            serde_json::from_str(histo_data.as_str()).expect("Error in the histogram data")
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default()).init();

    let args: Args = argh::from_env();
    let fractal_config =
        fs::read_to_string(args.config_filename).map(|x| serde_json::from_str(x.as_str()))??;

    let histogram = get_histogram_from_fractal_conf(fractal_config);

    fs::File::create(args.output_histogram)?
        .write_all(
            &serde_json::to_vec_pretty(&histogram)
                .expect("Unable to convert the histogram in json file"),
        )
        .expect("Unable to write the histogram file");

    Ok(())
}

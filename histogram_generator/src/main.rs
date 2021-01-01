#![forbid(unsafe_code)]
#![feature(trait_alias)]
#![feature(box_syntax)]

use std::fs;
use std::io::Write;

use argh::FromArgs;
use fractatoe::fractals::{histogram::Histogram, HistogramGeneration};

mod config;

#[derive(FromArgs)]
/// Arguments
struct Args {
    #[argh(positional)]
    config_filename: String,
    #[argh(positional, description = "save the histogram to a file")]
    output_histogram: String,
}

use config::{FractalConf, GenerationConf};

fn get_histogram_from_gen_conf(gen_conf: GenerationConf) -> Histogram {
    let histogram_conf = gen_conf.histogram_conf;
    match gen_conf.fractal_conf {
        FractalConf::Mandelbrot(generator) => generator.build_histogram(histogram_conf),
        FractalConf::Julia(generator) => generator.build_histogram(histogram_conf),
        FractalConf::Flame(generator) => generator.build().build_histogram(histogram_conf),
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default()).init();

    let args: Args = argh::from_env();
    let gen_conf: GenerationConf =
        fs::read_to_string(args.config_filename).map(|x| serde_json::from_str(x.as_str()))??;

    let histogram = get_histogram_from_gen_conf(gen_conf);

    fs::File::create(args.output_histogram)?
        .write_all(
            &serde_json::to_vec_pretty(&histogram)
                .expect("Unable to convert the histogram in json file"),
        )
        .expect("Unable to write the histogram file");

    Ok(())
}

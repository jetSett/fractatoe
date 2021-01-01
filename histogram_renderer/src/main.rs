use argh::FromArgs;
use std::fs;

#[derive(FromArgs)]
/// Arguments
struct Args {
    #[argh(positional, description = "histogram to render")]
    histogram_filename: String,
    #[argh(positional, description = "rendering configuration")]
    config_filename: String,
    #[argh(option, description = "save the image to a png file", short = 'o')]
    output_image: Option<String>,
    #[argh(switch, description = "do not show on screen")]
    no_show: bool,
}

use fractatoe::fractals::histogram::Histogram;
use fractatoe::image::Image;
use fractatoe::rendering::HistogramRendering;

use winit::dpi::PhysicalSize;

mod config;
mod png_save;
mod window;

use config::RenderingConf;

fn render_image(rendering_conf: RenderingConf, histogram: Histogram) -> Image {
    match rendering_conf {
        RenderingConf::MandelbrotRendering(conf) => conf.build().render_image(histogram),
        RenderingConf::FlameRendering(conf) => conf.build().render_image(histogram),
        RenderingConf::GaussianRendering(conf) => conf.build().render_image(histogram),
        RenderingConf::GreyscaleRendering(conf) => conf.build().render_image(histogram),
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default()).init();

    let args: Args = argh::from_env();

    if args.output_image.is_none() && args.no_show {
        return Ok(());
    }

    let rendering_conf = fs::read_to_string(args.config_filename)
        .map(|x| serde_json::from_str::<RenderingConf>(x.as_str()))??;

    let histogram = fs::read_to_string(args.histogram_filename)
        .map(|x| serde_json::from_str::<Histogram>(x.as_str()))??;

    let image = render_image(rendering_conf, histogram);

    if let Some(image_path) = args.output_image {
        png_save::save_image(&image, image_path)?;
    }

    if !args.no_show {
        window::show_image(
            PhysicalSize::new(image.width as f32, image.height as f32),
            image,
        )?;
    }

    Ok(())
}

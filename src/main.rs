#![forbid(unsafe_code)]

use log::{debug, error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use argh::FromArgs;
#[derive(FromArgs)]
/// Greet
struct Args {
    #[argh(positional, description = "scaling of the fractal")]
    scaling: f64,
}

#[derive(Default, Clone)]
struct Pix {
    r: u8,
    g: u8,
    b: u8,
    alpha: u8,
}

impl From<[u8; 4]> for Pix {
    fn from(x: [u8; 4]) -> Self {
        Pix {
            r: x[0],
            g: x[1],
            b: x[2],
            alpha: x[3],
        }
    }
}

impl Into<[u8; 4]> for Pix {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.alpha]
    }
}

struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pix>,
    changed: bool,
    scaling: f64,
}

fn does_diverge(x: f64, y: f64) -> bool {
    use num::complex::Complex;
    let mut z = Complex::new(0., 0.);

    const BOUND: f64 = 1000.;

    let c = Complex::new(x, y);

    for _ in 0..500 {
        if z.norm() > BOUND {
            return true;
        }
        z = z * z + c;
    }
    return false;
}
fn main() -> Result<(), pixels::Error> {
    env_logger::init();

    let args: Args = argh::from_env();

    info!("Starting the program");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    info!("Window created");
    let size = window.inner_size();
    let width: u32 = size.width;
    let height: u32 = size.height;

    let mut image = Image::new(width as usize, height as usize, args.scaling);

    let surface_texture = SurfaceTexture::new(width, height, &window);

    let mut pixels = Pixels::new(width, height, surface_texture)?;

    info!("Starting main loop");

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                image.compute_if_changed();
                image.draw(pixels.get_frame());
                if pixels
                    .render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::Resized(_) => {
                    let size = window.inner_size();
                    pixels.resize(size.width, size.height);
                    // image.resize(size.width as usize, size.height as usize);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    })
}

impl Image {
    fn new(width: usize, height: usize, scaling: f64) -> Self {
        let mut pixels_vec: Vec<Pix> = Vec::new();
        pixels_vec.resize_with(width * height, Default::default);
        Image {
            width,
            height,
            pixels: pixels_vec,
            changed: true,
            scaling,
        }
    }

    fn compute_if_changed(&mut self) {
        if self.changed {
            self.compute_image();
            self.changed = false;
        }
    }
    fn compute_image(&mut self) {
        debug!("{} - {}", self.width, self.height);
        for i in 0..(self.width - 1) {
            for j in 0..(self.height - 1) {
                let x = self.scaling * ((i as f64) - (self.width as f64) / 2.);
                let y = self.scaling * ((j as f64) - (self.height as f64) / 2.);

                let pix = if does_diverge(x, y) {
                    Pix {
                        r: 0xff,
                        g: 0xff,
                        b: 0xff,
                        alpha: 0xff,
                    }
                } else {
                    Pix {
                        r: 0x00,
                        g: 0xff,
                        b: 0x0,
                        alpha: 0xff,
                    }
                };
                self.set_pixel(i as usize, j as usize, pix);
            }
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, pix: Pix) {
        self.pixels[x + y * self.width] = pix;
    }

    fn set_scaling(&mut self, scaling: f64) {
        self.scaling = scaling;
        self.changed = true;
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel[0] = self.pixels[i].r;
            pixel[1] = self.pixels[i].g;
            pixel[2] = self.pixels[i].b;
            pixel[3] = self.pixels[i].alpha;
        }
    }
    // fn resize(&mut self, width: usize, height: usize) {
    //     self.width = width;
    //     self.height = height;
    //     self.pixels
    //         .resize_with((width * height) as usize, Default::default);
    // }
}

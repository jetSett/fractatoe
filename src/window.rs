use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Default, Clone)]
pub struct Pix {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub alpha: u8,
}

impl Into<[u8; 4]> for Pix {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.alpha]
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pix>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels_vec: Vec<Pix> = Vec::new();
        pixels_vec.resize_with(width * height, Default::default);
        Image {
            width,
            height: height,
            pixels: pixels_vec,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pix: Pix) {
        self.pixels[x + y * self.width] = pix;
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel[0] = self.pixels[i].r;
            pixel[1] = self.pixels[i].g;
            pixel[2] = self.pixels[i].b;
            pixel[3] = self.pixels[i].alpha;
        }
    }
}

pub fn show_image<Size: Into<winit::dpi::Size>>(
    size: Size,
    image: Image,
) -> Result<(), pixels::Error> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();
    let width: u32 = size.width;
    let height: u32 = size.height;

    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(width, height, surface_texture)?;
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
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
    });
}

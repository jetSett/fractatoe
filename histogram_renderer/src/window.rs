use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use fractatoe::image::Image;

pub fn show_image<Size: Into<winit::dpi::Size>>(size: Size, image: Image) -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(size)
        .build(&event_loop)?;

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

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

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel[0] = self.pixels[i].r;
            pixel[1] = self.pixels[i].g;
            pixel[2] = self.pixels[i].b;
            pixel[3] = self.pixels[i].alpha;
        }
    }
}

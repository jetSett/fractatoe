pub mod mandelbrot {
    use crate::window::{Image, Pix};
    use num::complex::Complex;

    fn does_diverge(x: f64, y: f64, bound: f64, iterations: usize) -> bool {
        let mut z = Complex::new(0., 0.);

        let c = Complex::new(x, y);

        for _ in 0..iterations {
            if z.norm() > bound {
                return true;
            }
            z = z * z + c;
        }
        return false;
    }

    pub fn create_image(
        width: usize,
        height: usize,
        scaling: f64,
        bound: f64,
        iterations: usize,
    ) -> Image {
        let mut image = Image::new(600, 800);

        for i in 0..(width - 1) {
            for j in 0..(height - 1) {
                let x = scaling * ((i as f64) - (width as f64) / 2.);
                let y = scaling * ((j as f64) - (height as f64) / 2.);

                let pix = if does_diverge(x, y, bound, iterations) {
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
                image.set_pixel(i as usize, j as usize, pix);
            }
        }
        image
    }
}

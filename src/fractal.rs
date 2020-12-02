pub mod mandelbrot {
    use crate::window::{Image, Pix};
    use num::complex::Complex;

    fn divergence(x: f64, y: f64, bound: f64, iterations: usize) -> f64 {
        let mut z = Complex::new(0., 0.);

        let c = Complex::new(x, y);

        for i in 0..iterations {
            if z.norm() > bound {
                return 1. / (i as f64).powf(0.3);
            }
            z = z * z + c;
        }
        return 0.;
    }

    pub fn create_image(
        width: usize,
        height: usize,
        scaling: f64,
        bound: f64,
        iterations: usize,
    ) -> Image {
        let mut image = Image::new(width, height);

        for i in 0..(width - 1) {
            for j in 0..(height - 1) {
                let x = scaling * ((i as f64) - (width as f64) / 2.);
                let y = scaling * ((j as f64) - (height as f64) / 2.);

                let div = divergence(x, y, bound, iterations);

                let pix = Pix {
                    r: (20. * div) as u8,
                    g: (0. * div) as u8,
                    b: (255. * div) as u8,
                    alpha: 0xff,
                };
                image.set_pixel(i as usize, j as usize, pix);
            }
        }
        image
    }
}

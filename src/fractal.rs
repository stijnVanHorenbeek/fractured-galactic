extern crate num_complex;
use num_complex::Complex;

pub(crate) const MAX_ITER: u32 = 100;

pub fn mandelbrot(c: Complex<f64>) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    MAX_ITER
}

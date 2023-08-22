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

pub fn julia(z: Complex<f64>, c: Complex<f64>) -> u32 {
    let mut z = z;
    for i in 0..MAX_ITER {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    MAX_ITER
}

pub async fn compute_mandelbrot(
    center_x: f64,
    center_y: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<u32>> {
    let rows: Vec<_> = (0..height)
        .map(|y| {
            tokio::spawn(async move {
                let mut row = Vec::with_capacity(width);
                for x in 0..width {
                    let cx = ((x as f64 / width as f64) * 4.0 - 2.0) + center_x;
                    let cy = ((y as f64 / height as f64) * 4.0 - 2.0) + center_y;
                    let c = num_complex::Complex::new(cx, cy);
                    let val = mandelbrot(c);
                    row.push(val);
                }
                row
            })
        })
        .collect();

    let mut result = Vec::with_capacity(height);
    for row_fut in rows {
        result.push(row_fut.await.unwrap());
    }
    result
}

pub async fn compute_julia_set(
    c: Complex<f64>,
    center_x: f64,
    center_y: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<u32>> {
    let rows: Vec<_> = (0..height)
        .map(|y| {
            tokio::spawn(async move {
                let mut row = Vec::with_capacity(width);
                for x in 0..width {
                    let zx = ((x as f64 / width as f64) * 4.0 - 2.0) + center_x;
                    let zy = ((y as f64 / height as f64) * 4.0 - 2.0) + center_y;
                    let z = num_complex::Complex::new(zx, zy);
                    let val = julia(z, c);
                    row.push(val);
                }
                row
            })
        })
        .collect();

    let mut result = Vec::with_capacity(height);
    for row_fut in rows {
        result.push(row_fut.await.unwrap());
    }
    result
}

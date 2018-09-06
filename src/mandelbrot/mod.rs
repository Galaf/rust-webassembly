use Configuration;

const RGBA_PIXELS_BYTE_COUNT: usize = 4;
const RGBA_FULL_OPAQUE: u8 = 255;

fn initialize_vector(width: u32, height: u32) -> Vec<u8> {
    let max_capacity: usize = RGBA_PIXELS_BYTE_COUNT * (width * height) as usize;
    let mut v: Vec<u8> = Vec::with_capacity(max_capacity);

    v.resize(max_capacity, 0);

    return v;
}

fn iterate_mandel(cx: f64, cy: f64, iterations: usize) -> usize {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut xx: f64 = 0.0;
    let mut yy: f64 = 0.0;
    let mut xy: f64;

    let mut i: usize = iterations;

    while i > 0 && xx + yy <= 4.0 {
        xy = x * y;
        xx = x * x;
        yy = y * y;
        x = xx - yy + cx;
        y = xy + xy + cy;

        i -= 1;
    }

    return iterations - i;
}

#[inline]
fn fill_rgb(data: &mut Vec<u8>, position: usize, r: u8, g: u8, b: u8) {
    data[position] = r;
    data[position + 1] = g;
    data[position + 2] = b;
    data[position + 3] = RGBA_FULL_OPAQUE;
}

pub fn get_mandelbrot_set(configuration: &Configuration) -> Vec<u8> {
    let iterations: usize = configuration.iterations;
    let width = configuration.width;
    let height = configuration.height;

    let mut data: Vec<u8> = initialize_vector(width, height);

    let xmin: f64 = configuration.xmin;
    let xmax: f64 = configuration.xmax;
    let ymin: f64 = configuration.ymin;
    let ymax: f64 = configuration.ymax;

    for ix in 0..width {
        for iy in 0..height {
            //let x: f64 = width as f64 / (xmax - xmin);
            let x: f64 = xmin + (xmax - xmin) * ix as f64 / (width - 1) as f64;
            let y: f64 = ymin + (ymax - ymin) * iy as f64 / (height - 1) as f64;
            //let y: f64 = ymin + (ymax - ymin) * iy as f64 / (height - 1) as f64;
            let i: usize = iterate_mandel(x, y, iterations);
            let ppos: usize = (4 * (width * iy + ix)) as usize;

            if i > iterations {
                fill_rgb(&mut data, ppos, 0, 0, 0);
            } else {
                let c: u8 = (3 as f64 * (i as f64).ln() / (iterations as f64 - 1.0).ln()) as u8;

                match c {
                    c if c < 1 => {
                        fill_rgb(&mut data, ppos, 255 * c, 0, 0);
                    },
                    c if c < 2 => {
                        fill_rgb(&mut data, ppos, 255, 255 * (c - 1), 0);
                    },
                    _ => {
                        fill_rgb(&mut data, ppos, 255, 255, 255 * (c - 2));
                    },
                }
            }
        }
    }

    return data;
}

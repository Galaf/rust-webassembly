fn initialize_vector(width: u32, height: u32) -> Vec<u8> {
    let max_capacity: usize = (4 * width * height) as usize;
    let mut v = Vec::with_capacity(max_capacity);

    v.resize_default(max_capacity);

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

pub fn get_mandelbrot_set(width: u32, height: u32) -> Vec<u8> {
    let mut data: Vec<u8> = initialize_vector(width, height);

    let xmin: f64 = -2.0;
    let xmax: f64 = 1.0;
    let ymin: f64 = -1.0;
    let ymax: f64 = 1.0;
    let iterations: usize = 1000;

    for ix in 0..width {
        for iy in 0..height {
            let x: f64 = xmin + (xmax - xmin) * ix as f64 / (width - 1) as f64;
            let y: f64 = ymin + (ymax - ymin) * iy as f64 / (height - 1) as f64;
            let i: usize = iterate_mandel(x, y, iterations);
            let ppos: usize = (4 * (width * iy + ix)) as usize;

            if i > iterations {
                data[ppos] = 0;
                data[ppos + 1] = 0;
                data[ppos + 2] = 0;
            } else {
                let c: u8 = (3 as f64 * (i as f64).ln() / (iterations as f64 - 1.0).ln()) as u8;

                if c < 1 {
                    data[ppos] = 255 * c;
                    data[ppos + 1] = 0;
                    data[ppos + 2] = 0;
                } else if c < 2 {
                    data[ppos] = 255;
                    data[ppos + 1] = 255 * (c - 1);
                    data[ppos + 2] = 0;
                } else {
                    data[ppos] = 255;
                    data[ppos + 1] = 255;
                    data[ppos + 2] = 255 * (c - 2);
                }
            }

            data[ppos + 3] = 255;
        }
    }

    return data;
}

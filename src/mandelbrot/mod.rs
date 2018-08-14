fn initialize_vector(width: u32, height: u32) -> Vec<u8> {
    let mut v = Vec::new();
    let max_capacity = 4 * width * height;

    for _ in 0..max_capacity {
        v.push(200);
    }

    return v;
}

fn iterate_mandel(cx: f64, cy: f64, iterations: usize) -> usize {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut xx: f64 = 0.0;
    let mut yy: f64 = 0.0;
    let mut xy: f64 = 0.0;

    let mut i = iterations;

    while i > 0 && xx + yy <= 4f64 {
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
            let x = xmin + (xmax - xmin) * ix as f64 / (width - 1) as f64;
            let y = ymin + (ymax - ymin) * iy as f64 / (height - 1) as f64;
            let i = iterate_mandel(x, y, iterations);
            let ppos = 4 * (width * iy + ix);

            if i > iterations {
                data[ppos as usize] = 0;
                data[(ppos + 1) as usize] = 0;
                data[(ppos + 2) as usize] = 0;
            } else {
                let c = 3 as f64 * (i as f64).ln() / (iterations as f64 - 1.0).ln();

                if c < 1.0 {
                    data[ppos as usize] = 255 * c as u8;
                    data[(ppos + 1) as usize] = 0;
                    data[(ppos + 2) as usize] = 0;
                } else if c < 2.0 {
                    data[ppos as usize] = 255;
                    data[(ppos + 1) as usize] = 255 * (c as u8 - 1);
                    data[(ppos + 2) as usize] = 0;
                } else {
                    data[ppos as usize] = 255;
                    data[(ppos + 1) as usize] = 255;
                    data[(ppos + 2) as usize] = 255 * (c as u8 - 2);
                }
            }

            data[(ppos + 3) as usize] = 255;
        }
    }

    return data;
}

use rand::{thread_rng, Rng};

pub fn get_mandelbrot_set(width: u32, height: u32) -> Vec<u8> {
    let mut data: Vec<u8> = vec![];

    let limit = 4 * width * height;
    //let mut rng = thread_rng();

    for _ in 0..limit {
        data.push(135);
    }

    return data;
}

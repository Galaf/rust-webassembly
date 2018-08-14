pub fn get_mandelbrot_set(width: u32, height: u32) -> Vec<u8> {
    let mut data: Vec<u8> = vec![];

    for x in 0..width * 4 {
        for y in 0..height * 4 {
            data.push(255);
        }
    }

    return data;
}

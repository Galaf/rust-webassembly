fn belongs_in_set(x: usize, y: usize) -> bool {
    let mut real_component = x;
    let mut imaginary_component = y;

    for _i in 0..10 {
        let temp_real_component =
            real_component * real_component - imaginary_component * imaginary_component + x;
        let temp_imaginary_component = 2 * real_component * imaginary_component + y;

        real_component = temp_real_component;
        imaginary_component = temp_imaginary_component;
    }

    if real_component * imaginary_component < 5 {
        return true;
    }

    return false;
}

const MAGNIFICATION_FACTOR: usize = 600;
const PAN_X: usize = 0;
const PAN_Y: usize = 0;

pub fn draw(width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut canvas: Vec<Vec<usize>> = Vec::new();
    for x in 0..width {
        canvas[x] = Vec::new();
        for y in 0..height {
            let belongs = belongs_in_set(
                x / MAGNIFICATION_FACTOR - PAN_X,
                y / MAGNIFICATION_FACTOR - PAN_Y,
            );

            canvas[x][y] = {
                if belongs {
                    1
                } else {
                    0
                }
            }
        }
    }

    return canvas;
}

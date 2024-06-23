use rtiow::color::Color;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let c = Color::new(
                i as f64 / (IMAGE_HEIGHT - 1) as f64,
                j as f64 / (IMAGE_WIDTH - 1) as f64,
                0.0,
            );
            println!("{}", c);
        }
    }
}

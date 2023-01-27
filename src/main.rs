const IMAGE_WIDTH: u64 = 256;
const IMAGE_HEIGHT: u64 = 256;
use std::io::{stderr, Write};

use ray_tracing_in_one_weekend::Color;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                0.25,
            );

            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("\nDone.");
}

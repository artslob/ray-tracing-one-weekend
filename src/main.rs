use std::ops;
use std::ops::Add;

#[derive(Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn origin() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    const BRIGHTNESS: i32 = 255;

    // header of ppm image file
    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, BRIGHTNESS);

    for j in (0..IMAGE_HEIGHT).rev() {
        // eprintln!("Processing {} rows. Remains {}", IMAGE_HEIGHT, j + 1);
        for i in 0..IMAGE_WIDTH {
            let red = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let green = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let blue: f64 = 0.25;

            const ROUNDING: f64 = 255.0;

            let red = (ROUNDING * red) as i32;
            let green = (ROUNDING * green) as i32;
            let blue = (ROUNDING * blue) as i32;

            print!("{} {} {} ", red, green, blue);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_sum_origins() {
        let sum = Vec3::origin() + Vec3::origin();
        assert_eq!(sum.x, 0.0);
        assert_eq!(sum.y, 0.0);
        assert_eq!(sum.z, 0.0);
    }
}

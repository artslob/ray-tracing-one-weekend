use std::cmp;
use std::ops;

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

impl cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        compare_floats(self.x, other.x)
            && compare_floats(self.y, other.y)
            && compare_floats(self.z, other.z)
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

fn compare_floats_eps(left: f64, right: f64, epsilon: f64) -> bool {
    return (left - right).abs() < epsilon;
}

fn compare_floats(left: f64, right: f64) -> bool {
    return compare_floats_eps(left, right, 0.0000001);
}

#[cfg(test)]
mod tests {
    use crate::compare_floats;
    use crate::Vec3;

    #[test]
    fn test_sum_origins() {
        let sum = Vec3::origin() + Vec3::origin();
        assert_eq!(sum.x, 0.0);
        assert_eq!(sum.y, 0.0);
        assert_eq!(sum.z, 0.0);
    }

    #[test]
    fn test_sum_different_origin() {
        let sum = Vec3::new(0.3, 0.2, 1.5) + Vec3::origin();
        assert_eq!(sum.x, 0.3);
        assert_eq!(sum.y, 0.2);
        assert_eq!(sum.z, 1.5);
    }

    #[test]
    fn test_sum_different() {
        let sum = Vec3::new(100.0, 55.97, -7.7) + Vec3::new(-0.0, -327.12, 7.1);
        assert_eq!(sum.x, 100.0);
        assert_eq!(sum.y, -271.15);
        assert!(compare_floats(sum.z, -0.6));
    }

    #[test]
    fn test_sum_eq() {
        let sum = Vec3::new(100.0, 55.97, -7.13) + Vec3::new(-0.0, -327.12, -3.135);
        let result = Vec3::new(100.0, -271.15, -10.265);
        assert_eq!(sum, result);
    }

    #[test]
    fn test_ne() {
        assert_ne!(
            Vec3::new(100.0, 55.97, -7.130001),
            Vec3::new(100.0, 55.97, -7.13)
        );
    }
}

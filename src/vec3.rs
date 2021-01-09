use crate::utils;
use std::cmp;
use std::ops;

#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Copy for Vec3 {}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Vec3 {
    pub fn origin() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn random(min: f64, max: f64) -> Self {
        Self {
            x: utils::random_double_range(min, max),
            y: utils::random_double_range(min, max),
            z: utils::random_double_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let random_sphere = Self::random(-1., 1.);
            if random_sphere.length_squared() >= 1. {
                continue;
            }
            return random_sphere;
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Self {
                x: utils::random_double_range(-1., 1.),
                y: utils::random_double_range(-1., 1.),
                z: 0.0,
            };
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn write_color(color: Color, samples_per_pixel: i32) {
        let scale = 1. / samples_per_pixel as f64;

        let red = Self::color_value(color.x, scale);
        let green = Self::color_value(color.y, scale);
        let blue = Self::color_value(color.z, scale);

        print!("{} {} {} ", red, green, blue);
    }

    fn color_value(value: f64, scale: f64) -> i32 {
        const ROUNDING: f64 = 256.0;

        // gamma-correct for gamma=2.0
        let value = (value * scale).sqrt();
        // Write the translated [0,255] value of each color component.
        (ROUNDING * utils::clamp(value, 0., 0.999)) as i32
    }

    pub fn near_zero(&self) -> bool {
        utils::compare_floats(self.x, 0.0)
            && utils::compare_floats(self.y, 0.0)
            && utils::compare_floats(self.z, 0.0)
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

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        utils::compare_floats(self.x, other.x)
            && utils::compare_floats(self.y, other.y)
            && utils::compare_floats(self.z, other.z)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

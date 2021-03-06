use crate::hittable::Hittable;
use crate::vec3::{Color, Point3, Vec3};
use crate::world::World;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(&self, world: &World, depth: i32) -> Color {
        const BLACK: Vec3 = Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        if depth <= 0 {
            return BLACK;
        }

        if let Some(record) = world.hit(self, 0.001, std::f64::INFINITY) {
            if let Some(scatter_data) = record.material.scatter(self, &record) {
                let attenuation = scatter_data.attenuation;
                let scattered = scatter_data.scattered;
                return attenuation * Self::ray_color(&scattered, world, depth - 1);
            }
            return BLACK;
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        let white = Color::new(1.0, 1.0, 1.0);
        // lerp: linear blend
        // blendedValue = (1−t)⋅startValue + t⋅endValue
        (1.0 - t) * white + t * Color::new(0.5, 0.7, 1.0)
    }
}

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

    pub fn ray_color(&self, world: &World) -> Color {
        if let Some(record) = world.hit(self, 0., std::f64::INFINITY) {
            let color = Color {
                x: 1.,
                y: 1.,
                z: 1.,
            };
            return 0.5 * (record.normal + color);
        }
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        let white = Color::new(1.0, 1.0, 1.0);
        // lerp: linear blend
        // blendedValue = (1−t)⋅startValue + t⋅endValue
        (1.0 - t) * white + t * Color::new(0.5, 0.7, 1.0)
    }
}

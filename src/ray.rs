use crate::vec3::{Color, Point3, Vec3};

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

    pub fn ray_color(&self) -> Color {
        let t = self.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5);
        if let Some(t) = t {
            let n = (self.at(t)
                - Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                })
            .unit_vector();
            return 0.5
                * Color {
                    x: n.x + 1.0,
                    y: n.y + 1.0,
                    z: n.z + 1.0,
                };
        }
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        let white = Color::new(1.0, 1.0, 1.0);
        // lerp: linear blend
        // blendedValue = (1−t)⋅startValue + t⋅endValue
        (1.0 - t) * white + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn hit_sphere(&self, center: Point3, radius: f64) -> Option<f64> {
        let oc: Vec3 = Vec3::origin() - center;
        let a = self.direction.length_squared();
        let half_b = oc.dot(&self.direction);
        let c = oc.length_squared() - radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            None
        } else {
            Some((-half_b - discriminant.sqrt()) / a)
        }
    }
}

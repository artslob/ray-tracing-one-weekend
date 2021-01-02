use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        let mut root = (-half_b - sqrt_discriminant) / a;

        if root < min || root > max {
            root = (-half_b + sqrt_discriminant) / a;
            if root < min || root > max {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let front_face = ray.direction.dot(&outward_normal) < 0.0;

        Some(HitRecord {
            point,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            t: root,
            front_face,
        })
    }
}

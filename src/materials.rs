use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterData>;
}

pub struct ScatterData {
    attenuation: Color,
    scattered: Ray,
}

pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<ScatterData> {
        let scatter_direction = record.normal + Vec3::random_unit_vector();
        let scatter_direction = if scatter_direction.near_zero() {
            record.normal
        } else {
            scatter_direction
        };
        let scattered = Ray::new(record.point, scatter_direction);
        Some(ScatterData {
            attenuation: self.albedo,
            scattered,
        })
    }
}

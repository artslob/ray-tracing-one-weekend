use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterData>;
}

pub struct ScatterData {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
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

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }

    fn reflect(&self, v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(&n) * n
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterData> {
        let reflected = self.reflect(ray.direction.unit_vector(), record.normal);
        let attenuation = self.albedo;
        let scattered = Ray::new(record.point, reflected);
        if scattered.direction.dot(&record.normal) > 0. {
            Some(ScatterData {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }
}

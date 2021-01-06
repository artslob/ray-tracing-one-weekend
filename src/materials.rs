use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils;
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
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterData> {
        let reflected = reflect(ray.direction.unit_vector(), record.normal);
        let attenuation = self.albedo;
        let scattered = Ray::new(
            record.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
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

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn refract(unit_direction: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -unit_direction.dot(&normal);
        let cos_theta = cos_theta.min(1.0);
        let r_out_perp = etai_over_etat * (unit_direction + cos_theta * normal);
        let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt() * (-normal);
        r_out_perp + r_out_parallel
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterData> {
        let attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflect_value = reflectance(cos_theta, refraction_ratio);
        let has_reflectance = reflect_value > utils::random_double();
        let direction = if cannot_refract || has_reflectance {
            reflect(unit_direction, record.normal)
        } else {
            Dielectric::refract(unit_direction, record.normal, refraction_ratio)
        };
        let scattered = Ray::new(record.point, direction);

        Some(ScatterData {
            attenuation,
            scattered,
        })
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // use Schlick's approximation for reflectance
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0.powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

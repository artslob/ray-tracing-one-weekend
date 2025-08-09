use crate::hittable::{HitRecord, Hittable};
use crate::materials;
use crate::ray::Ray;
use crate::rng::Random;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3};

type ThreadHittable = dyn Hittable + Sync + Send;

pub struct World {
    list: Vec<Box<ThreadHittable>>,
    pub random: Random,
}

impl World {
    pub fn new(random: Random) -> Self {
        Self {
            list: vec![],
            random,
        }
    }

    pub fn add(&mut self, value: Box<ThreadHittable>) {
        self.list.push(value)
    }

    pub fn with_items(mut self) -> Self {
        let material_ground = materials::Lambertian::new(Color {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        });
        self.add(Box::new(Sphere::new(
            Point3 {
                x: 0.0,
                y: -1000.,
                z: 0.0,
            },
            1000.,
            Box::new(material_ground),
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = self.random.random_f64();
                let center = Point3 {
                    x: a as f64 + 0.9 * self.random.random_f64(),
                    y: 0.2,
                    z: b as f64 + 0.9 * self.random.random_f64(),
                };
                let another_point = Point3 {
                    x: 4.,
                    y: 0.2,
                    z: 0.,
                };

                if (center - another_point).length() <= 0.9 {
                    continue;
                }

                let sphere_material: Box<dyn materials::Material + Send + Sync> =
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Color::random(&self.random) * Color::random(&self.random);
                        Box::new(materials::Lambertian::new(albedo))
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Color::random_range(&self.random, 0.5, 1.);
                        let fuzz = self.random.random_f64_in_range(0., 0.5);
                        Box::new(materials::Metal::new(albedo, fuzz))
                    } else {
                        // glass
                        Box::new(materials::Dielectric::new(1.5))
                    };

                self.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }

        self.add(Box::new(Sphere::new(
            Point3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            1.,
            Box::new(materials::Dielectric::new(1.5)),
        )));
        self.add(Box::new(Sphere::new(
            Point3 {
                x: -4.0,
                y: 1.0,
                z: 0.0,
            },
            1.,
            Box::new(materials::Lambertian::new(Color {
                x: 0.4,
                y: 0.2,
                z: 0.1,
            })),
        )));
        self.add(Box::new(Sphere::new(
            Point3 {
                x: 4.0,
                y: 1.0,
                z: 0.0,
            },
            1.,
            Box::new(materials::Metal::new(
                Color {
                    x: 0.7,
                    y: 0.6,
                    z: 0.5,
                },
                0.1,
            )),
        )));

        self
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        let mut closest = max;
        let mut result: Option<HitRecord> = None;

        for hittable in self.list.iter() {
            if let Some(record) = hittable.hit(ray, min, closest) {
                closest = record.t;
                result = Some(record);
            }
        }
        result
    }
}

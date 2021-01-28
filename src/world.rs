use crate::hittable::{HitRecord, Hittable};
use crate::materials;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils;
use crate::vec3::{Color, Point3};

type ThreadHittable = dyn Hittable + Sync + Send;

pub struct World {
    list: Vec<Box<ThreadHittable>>,
}

impl World {
    pub fn new(list: Vec<Box<ThreadHittable>>) -> Self {
        Self { list }
    }

    pub fn add(&mut self, value: Box<ThreadHittable>) {
        self.list.push(value)
    }

    pub fn with_items() -> Self {
        let mut the_world = Self::new(vec![]);

        let material_ground = materials::Lambertian::new(Color {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        });
        the_world.add(Box::new(Sphere::new(
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
                let choose_mat = utils::random_double();
                let center = Point3 {
                    x: a as f64 + 0.9 * utils::random_double(),
                    y: 0.2,
                    z: b as f64 + 0.9 * utils::random_double(),
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
                        let albedo = Color::random() * Color::random();
                        Box::new(materials::Lambertian::new(albedo))
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Color::random_range(0.5, 1.);
                        let fuzz = utils::random_double_range(0., 0.5);
                        Box::new(materials::Metal::new(albedo, fuzz))
                    } else {
                        // glass
                        Box::new(materials::Dielectric::new(1.5))
                    };

                the_world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }

        the_world.add(Box::new(Sphere::new(
            Point3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            1.,
            Box::new(materials::Dielectric::new(1.5)),
        )));
        the_world.add(Box::new(Sphere::new(
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
        the_world.add(Box::new(Sphere::new(
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

        the_world
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

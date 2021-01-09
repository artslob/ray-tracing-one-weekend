use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};
use rand::Rng;

mod camera;
mod hittable;
mod materials;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod world;

fn main() {
    const ASPECT_RATIO: f64 = 3.0 / 2.0;

    // image
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const BRIGHTNESS: i32 = 255;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    let the_world = make_world();

    let lookfrom = Point3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let lookat = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let camera = camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let mut rng = rand::thread_rng();

    // header of ppm image file
    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, BRIGHTNESS);

    // rendering from left upper corner to right lower corner
    for j in (0..IMAGE_HEIGHT).rev() {
        // eprintln!("Processing {} rows. Remains {}", IMAGE_HEIGHT, j + 1);
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0., 0., 0.);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                color += camera.get_ray(u, v).ray_color(&the_world, MAX_DEPTH);
            }

            Vec3::write_color(color, SAMPLES_PER_PIXEL);
        }
        println!();
    }
}

fn make_world() -> world::World {
    let mut the_world = world::World::new(vec![]);

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

            let sphere_material: Box<dyn materials::Material> = if choose_mat < 0.8 {
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

#[cfg(test)]
mod tests {
    use crate::utils::compare_floats;
    use crate::vec3::Vec3;

    #[test]
    fn test_sum_origins() {
        let sum = Vec3::origin() + Vec3::origin();
        assert_eq!(sum.x, 0.0);
        assert_eq!(sum.y, 0.0);
        assert_eq!(sum.z, 0.0);
    }

    #[test]
    fn test_sum_different_origin() {
        let sum = Vec3::new(0.3, 0.2, 1.5) + Vec3::origin();
        assert_eq!(sum.x, 0.3);
        assert_eq!(sum.y, 0.2);
        assert_eq!(sum.z, 1.5);
    }

    #[test]
    fn test_sum_different() {
        let sum = Vec3::new(100.0, 55.97, -7.7) + Vec3::new(-0.0, -327.12, 7.1);
        assert_eq!(sum.x, 100.0);
        assert_eq!(sum.y, -271.15);
        assert!(compare_floats(sum.z, -0.6));
    }

    #[test]
    fn test_sub() {
        let result = Vec3::new(100.0, 55.97, -7.7) - Vec3::new(-1.0, -327.12, 7.1);
        assert_eq!(result, Vec3::new(101.0, 383.09, -14.8));
    }

    #[test]
    fn test_sum_eq() {
        let sum = Vec3::new(100.0, 55.97, -7.13) + Vec3::new(-0.0, -327.12, -3.135);
        let result = Vec3::new(100.0, -271.15, -10.265);
        assert_eq!(sum, result);
    }

    #[test]
    fn test_ne() {
        assert_ne!(
            Vec3::new(100.0, 55.97, -7.130001),
            Vec3::new(100.0, 55.97, -7.13)
        );
    }

    #[test]
    fn test_ng_origin() {
        let origin = Vec3::origin();
        let result = -origin;
        assert_eq!(result, Vec3::origin());
    }

    #[test]
    fn test_ng_some() {
        let origin = Vec3::new(3.4, 5.5, -7.0);
        let result = -origin;
        assert_eq!(result, Vec3::new(-3.4, -5.5, 7.0));
    }

    #[test]
    fn test_add_assign() {
        let mut result = Vec3::new(3.2, 4.5, -6.0);
        result += Vec3::new(3.0, -2.5, 1.0);
        assert_eq!(result, Vec3::new(6.2, 2.0, -5.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut result = Vec3::new(3.2, 4.5, -6.0);
        result *= 3.0;
        assert_eq!(result, Vec3::new(9.6, 13.5, -18.0));
    }

    #[test]
    fn test_mul() {
        let a1 = Vec3::new(3.2, 4.5, -6.0);
        let a2 = Vec3::new(7.0, 1.5, 3.0);
        assert_eq!(a1 * a2, Vec3::new(22.4, 6.75, -18.0));
    }

    #[test]
    fn test_mul_f64() {
        let v = Vec3::new(3.2, 4.5, -6.0);
        assert_eq!(v * 3.0, Vec3::new(9.6, 13.5, -18.0));
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(3.3, 4.5, -6.0);
        assert_eq!(v / 3.0, Vec3::new(1.1, 1.5, -2.0));
    }

    #[test]
    fn test_f64_mul_vec() {
        let v = Vec3::new(3.2, 4.5, -6.0);
        assert_eq!(3.0 * v, Vec3::new(9.6, 13.5, -18.0));
    }

    #[test]
    fn test_div_assign() {
        let mut result = Vec3::new(3.2, 4.5, -6.0);
        result /= 2.0;
        assert_eq!(result, Vec3::new(1.6, 2.25, -3.0));
    }

    #[test]
    fn test_length() {
        let vec = Vec3::new(3.2, 4.5, -6.0);
        let result = vec.length();
        let expected = 8.154140052758477;
        assert!(compare_floats(result, expected), "result is {}", result);
        assert!(compare_floats(vec.x, 3.2), "x is {}", vec.x);
    }

    #[test]
    fn test_fmt() {
        let vec = Vec3::new(3.2, 4.5, -6.0);
        assert_eq!(format!("{}", vec), "3.2 4.5 -6")
    }
}

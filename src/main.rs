use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use rand::{thread_rng, Rng};

use crate::vec3::{Color, Point3, Vec3};
use clap::Parser;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

mod camera;
mod cli;
mod hittable;
mod materials;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod world;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const BRIGHTNESS: i32 = 255;
const SAMPLES_PER_PIXEL: i32 = 500;
const MAX_DEPTH: i32 = 50;

fn main() {
    let the_world = Arc::new(world::World::with_items());

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
    let camera = Arc::new(camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    ));

    let args = cli::Args::parse();

    // header of ppm image file
    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, BRIGHTNESS);

    if args.single_thread {
        eprintln!("use single thread");
        single_thread(&camera, &the_world);
    } else {
        eprintln!("use multiple threads");
        multiple_threads(&camera, &the_world);
    }
}

fn multiple_threads(camera: &Arc<camera::Camera>, the_world: &Arc<world::World>) {
    let thread_count = match ::num_cpus::get() {
        0..=1 => 1,
        n => n - 1,
    };
    eprintln!("running on {} threads", thread_count);

    let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(thread_count + 1);
    let (tx, rx) = mpsc::channel::<(usize, i32)>();
    let rx = Arc::new(Mutex::new(rx));

    let (row_tx, row_rx) = mpsc::channel::<Row>();

    threads.push(thread::spawn(move || {
        for (enumerator, j) in (0..IMAGE_HEIGHT).rev().enumerate() {
            tx.send((enumerator, j)).unwrap();
        }
    }));

    for _ in 0..thread_count {
        let the_world = Arc::clone(&the_world);
        let camera = Arc::clone(&camera);
        let rx = Arc::clone(&rx);
        let row_tx = row_tx.clone();

        threads.push(thread::spawn(move || {
            loop {
                let (enumerator, j) = match rx.lock().unwrap().recv() {
                    Ok((enumerator, j)) => (enumerator, j),
                    Err(_) => {
                        // eprintln!("exiting thread: {}", e);
                        return;
                    }
                };
                let start = std::time::Instant::now();
                let mut colors = Vec::with_capacity(IMAGE_WIDTH as usize);

                for i in 0..IMAGE_WIDTH {
                    let color = calc_color(&camera, &the_world, i, j);
                    colors.push(color);
                }
                row_tx.send(Row { colors, enumerator }).unwrap();
                eprintln!("{}", format_elapsed(start, j));
            }
        }));
    }

    drop(row_tx);

    let mut heap = BinaryHeap::new();
    let mut heap_cursor = 0;

    for row in row_rx {
        heap.push(row);

        while let Some(row) = heap.peek() {
            if row.enumerator != heap_cursor {
                break;
            }
            if let Some(row) = heap.pop() {
                for color in row.colors {
                    Vec3::write_color(color, SAMPLES_PER_PIXEL);
                }
            }
            heap_cursor += 1;
        }
    }

    assert_eq!(heap.len(), 0);

    for handle in threads {
        handle.join().unwrap();
    }
}

fn single_thread(camera: &camera::Camera, the_world: &world::World) {
    // rendering from left upper corner to right lower corner
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Processing {} rows. Remains {}", IMAGE_HEIGHT, j + 1);
        let start = std::time::Instant::now();

        for i in 0..IMAGE_WIDTH {
            let color = calc_color(camera, the_world, i, j);
            Vec3::write_color(color, SAMPLES_PER_PIXEL);
        }
        eprintln!("{}", format_elapsed(start, j));
        println!();
    }
}

fn calc_color(camera: &camera::Camera, the_world: &world::World, i: i32, j: i32) -> Color {
    let mut color = Color::new(0., 0., 0.);

    for _ in 0..SAMPLES_PER_PIXEL {
        let u = (i as f64 + thread_rng().gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
        let v = (j as f64 + thread_rng().gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

        color += camera.get_ray(u, v).ray_color(&the_world, MAX_DEPTH);
    }

    color
}

struct Row {
    colors: Vec<Color>,
    enumerator: usize,
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse order: from smallest to biggest
        other.enumerator.cmp(&self.enumerator)
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.enumerator.eq(&other.enumerator)
    }
}

impl Eq for Row {}

fn format_elapsed(start: Instant, j: i32) -> String {
    let elapsed = start.elapsed();
    format!(
        "time elapsed on {} {:?} {:?}",
        j,
        elapsed,
        elapsed.as_nanos(),
    )
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

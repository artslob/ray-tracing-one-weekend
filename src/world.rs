use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct World {
    list: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        Self { list }
    }

    pub fn add(&mut self, value: Box<dyn Hittable>) {
        self.list.push(value)
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

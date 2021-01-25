use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

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

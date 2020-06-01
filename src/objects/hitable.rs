use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;

#[derive(Debug, Default)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(l: Vec<Box<dyn Hitable>>) -> Self {
        HitableList { list: l }
    }

    pub fn add_hitable(hitable: &dyn Hitable) {

    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hitable in self.list.iter() {
            if hitable.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }

        hit_anything
    }
}
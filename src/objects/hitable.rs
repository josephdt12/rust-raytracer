use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::materials::Material;

#[derive(Debug, Default)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl HitRecord {
    pub fn material(&self) -> Material { self.material }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord);
}

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(l: Vec<Box<dyn Hitable>>) -> Self {
        HitableList { list: l }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let mut hit_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hitable in self.list.iter() {
            let (is_hit, temp_rec) = hitable.hit(r, t_min, closest_so_far);

            if is_hit {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                hit_record.t = temp_rec.t;
                hit_record.p = temp_rec.p;
                hit_record.normal = temp_rec.normal;
                hit_record.material = temp_rec.material;
            }
        }

        (hit_anything, hit_record)
    }
}
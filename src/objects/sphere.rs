use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::objects::hitable::Hitable;
use crate::objects::hitable::HitRecord;
use crate::materials::Material;

#[derive(Debug, Default)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(cen: &Vec3, r: f64, material: Material) -> Sphere {
        Sphere { center: *cen, radius: r, material }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let oc = *r.origin() - self.center;
        let a = Vec3::dot(r.direction(), r.direction());
        let b: f64 = Vec3::dot(&oc, r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let mut hit_record: HitRecord = HitRecord::default();
                hit_record.t = temp;
                hit_record.p = r.point_at_parameter(hit_record.t);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                hit_record.material = self.material;
                return (true, hit_record);
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let mut hit_record: HitRecord = HitRecord::default();
                hit_record.t = temp;
                hit_record.p = r.point_at_parameter(hit_record.t);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                hit_record.material = self.material;
                return (true, hit_record);
            }
        }
        (false, HitRecord::default())
    }
}
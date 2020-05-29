use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::objects::hitable::Hit;
use crate::objects::hitable::HitRecord;

#[derive(Debug, Default)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(cen: &Vec3, r: f32) -> Sphere {
        Sphere { center: *cen, radius: r }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = *r.origin() - self.center;
        let a = Vec3::dot(r.direction(), r.direction());
        let b: f32 = Vec3::dot(&oc, r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}
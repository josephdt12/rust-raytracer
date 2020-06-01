use rand::Rng;
use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::objects::hitable::HitRecord;
use crate::materials::Scatterable;

#[derive(Debug, Default, Clone, Copy)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo } 
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Ray, Vec3) {
        let target: Vec3 = hit_record.p + hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, target - hit_record.p);
        let attentuation = self.albedo;
        (true, scattered, attentuation)
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p: Vec3 = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) -
                    Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 { return p; }
    }
}
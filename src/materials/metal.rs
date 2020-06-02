use rand::Rng;
use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::materials::Scatterable;
use crate::objects::hitable::HitRecord;

#[derive(Debug, Clone, Copy, Default)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f64) -> Self {
        let fuzz: f64;
        if f < 1.0 { fuzz = f; } else { fuzz = 1.0; }
        Metal {
            albedo, fuzz
        }
    }
}

impl Scatterable for Metal {
    /// For metal surfaces, rays are scattered if the dot product of the
    /// scattered ray's direction and the surface normal is greater than 0.0.
    /// A value less than 0.0 would be pointing into the surface.
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Ray, Vec3) {
        let reflected: Vec3 = reflect(&Vec3::unit_vector(&ray_in.direction()), &hit_record.normal);

        // Reflections will be a bit fuzzy
        let scattered: Ray = Ray::new(hit_record.p, reflected + random_point_in_unit_sphere() * self.fuzz);

        let attentuation: Vec3 = self.albedo;
        (Vec3::dot(scattered.direction(), &hit_record.normal) > 0.0, scattered, attentuation)
    }
}

/// Return a vector for a vector reflected across a surface normal.
pub fn reflect(incoming: &Vec3, normal: &Vec3) -> Vec3 {
    *incoming - *normal * 2.0 * Vec3::dot(incoming, normal)
}

fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p: Vec3 = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) -
                    Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 { return p; }
    }
}
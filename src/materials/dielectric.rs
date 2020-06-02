use rand::Rng;
use crate::materials::Scatterable;
use crate::structures::vec3::Vec3;
use crate::structures::ray::Ray;
use crate::objects::hitable::HitRecord;

/// Dielectric materials are materials that will both reflect and
/// refracy incoming rays.
#[derive(Copy, Debug, Clone, Default)]
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Ray, Vec3) {
        let outward_normal: Vec3;
        let cosine: f64;
        let reflected: Vec3 = reflect(ray_in.direction(), hit_record.normal());

        let refractive_index_ratio: f64;

        // Hard-coded to be a glass surface with attentuation 1.0,
        // meaning the surface absorbs nothing.
        let attentuation: Vec3 = Vec3::new(1.0, 1.0, 1.0);

        if Vec3::dot(ray_in.direction(), hit_record.normal()) > 0.0 {
            outward_normal = -*hit_record.normal();
            refractive_index_ratio = self.refractive_index;
            cosine = self.refractive_index * Vec3::dot(ray_in.direction(), hit_record.normal()) / ray_in.direction().length();
        } else {
            outward_normal = *hit_record.normal();
            refractive_index_ratio = 1.0 / self.refractive_index;
            cosine = -Vec3::dot(ray_in.direction(), hit_record.normal()) / ray_in.direction().length();
        }

        let (is_refracted, refracted_vector) = refract(
            ray_in.direction(),
            &outward_normal,
            refractive_index_ratio,
        );

        let scattered: Ray;
        let reflect_probability: f64;

        if is_refracted {
            reflect_probability = schlick(cosine, self.refractive_index);
        } else {
            reflect_probability = 1.0;
        }

        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflect_probability {
            scattered = Ray::new(hit_record.p, reflected);
        } else {
            scattered = Ray::new(hit_record.p, refracted_vector);
        }

        (true, scattered, attentuation)
    }
}

/// Return a vector for a vector reflected across a surface normal.
pub fn reflect(incoming: &Vec3, normal: &Vec3) -> Vec3 {
    *incoming - *normal * 2.0 * Vec3::dot(incoming, normal)
}

/// Determine if an incoming vector will be refracted, and if so, the refracted vector.
pub fn refract(incoming: &Vec3, normal: &Vec3, refractive_index_ratio: f64) -> (bool, Vec3) {
    let unit_vector: Vec3 = Vec3::unit_vector(incoming);
    let dt: f64 = Vec3::dot(&unit_vector, normal);
    let discriminant: f64 = 1.0 - refractive_index_ratio * refractive_index_ratio * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted: Vec3 = (unit_vector - *normal * dt) * refractive_index_ratio - *normal * discriminant.sqrt();
        return (true, refracted);
    } else {
        return (false, Vec3::default());
    }
}

/// Schlick's approximation for calculating the reflectivity of a surface at different angles.
pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0: f64 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
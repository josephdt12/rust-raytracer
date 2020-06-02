use crate::structures::ray::Ray;
use crate::objects::hitable::HitRecord;
use crate::structures::vec3::Vec3;

pub mod lambertian;
pub mod metal;

use self::lambertian::Lambertian;
use self::metal::Metal;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian(Lambertian::default())
    }
}

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Ray, Vec3) {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray_in, hit_record),
            Material::Metal(ref inner) => inner.scatter(ray_in, hit_record),
        }
    }
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Ray, Vec3);
}
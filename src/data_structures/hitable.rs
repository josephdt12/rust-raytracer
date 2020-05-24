use super::Vec3;
use super::Ray;

#[derive(Debug, Default)]
pub struct Hit_Record {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &Hit_Record);
}
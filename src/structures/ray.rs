use crate::structures::vec3::Vec3;

#[derive(Debug, Default)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a: a, b: b }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.a
    }

    pub fn direction(&self) -> &Vec3 {
        &self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + self.b * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_points() {
        let ray = Ray::new(
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
        );
        let pointed_vec = ray.point_at_parameter(2.0);
        let expected_vec = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(pointed_vec, expected_vec);
    }
}
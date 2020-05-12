use std::f32;
use std::ops;

#[derive(Debug, Default)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { 
            e: [e0, e1, e2]
        }
    }

    // Basic accessor functions
    pub fn x(&self) -> f32 { self.e[0] }
    pub fn y(&self) -> f32 { self.e[1] }
    pub fn z(&self) -> f32 { self.e[2] }
    pub fn r(&self) -> f32 { self.e[0] }
    pub fn g(&self) -> f32 { self.e[1] }
    pub fn b(&self) -> f32 { self.e[2] }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }
    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] +
        self.e[2] * self.e[2]
    }

    /// Converts this vector to a unit vector.
    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        for val in &mut self.e {
            *val /= k;
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 2 {
            panic!("Invalid array access");
        }
        &self.e[index]
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.e == other.e
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [ self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2] ]
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            e: [ self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2] ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn it_add_assigns() {
        let mut vec3 = Vec3::new(1.0, 1.0, 1.0);
        vec3 += Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(vec3, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn it_sub_assigns() {
        let mut vec3 = Vec3::new(1.0, 1.0, 1.0);
        vec3 -= Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(vec3, Vec3::new(0.0, 0.0, 0.0));
    }
}
use std::f64;
use std::ops;

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    elements: [f64; 3],
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { 
            elements: [e0, e1, e2]
        }
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
    }

    // Basic accessor functions
    pub fn x(&self) -> f64 { self.elements[0] }
    pub fn y(&self) -> f64 { self.elements[1] }
    pub fn z(&self) -> f64 { self.elements[2] }
    pub fn r(&self) -> f64 { self.elements[0] }
    pub fn g(&self) -> f64 { self.elements[1] }
    pub fn b(&self) -> f64 { self.elements[2] }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        self.elements[0] * self.elements[0] + self.elements[1] * self.elements[1] +
        self.elements[2] * self.elements[2]
    }

    /// Converts this vector to a unit vector.
    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        for val in &mut self.elements {
            *val /= k;
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 2 {
            panic!("Invalid array access");
        }
        &self.elements[index]
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            elements: [ self[0] + other[0], self[1] + other[1], self[2] + other[2] ]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            elements: [ self[0] - other[0], self[1] - other[1], self[2] - other[2] ]
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            elements: [ self.elements[0] * scalar, self.elements[1] * scalar, self.elements[2] * scalar ]
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        Self {
            elements: [
                self.elements[0] * rhs.elements[0],
                self.elements[1] * rhs.elements[1],
                self.elements[2] * rhs.elements[2],
            ]
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        if scalar == 0.0 {
            panic!("Cannot divide by zero");
        }

        Self {
            elements: [ self.elements[0] / scalar, self.elements[1] / scalar, self.elements[2] / scalar ] 
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            elements: [ self.elements[0] + other.elements[0], self.elements[1] + other.elements[1], self.elements[2] + other.elements[2] ]
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            elements: [ self.elements[0] - other.elements[0], self.elements[1] - other.elements[1], self.elements[2] - other.elements[2] ]
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        let k = 1.0 / rhs;
        self.elements[0] *= k;
        self.elements[1] *= k;
        self.elements[2] *= k;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            elements: [ -self.elements[0], -self.elements[1], -self.elements[2] ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn it_adds() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);
        let vec3 = vec1 + vec2;
        assert_eq!(vec3, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn it_muls() {
        let vec = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = vec * 2.0;
        assert_eq!(vec2, Vec3::new(2.0, 2.0, 2.0));
    }

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
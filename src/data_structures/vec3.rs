use std::f32;

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
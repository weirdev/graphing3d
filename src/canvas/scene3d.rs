use std::ops::Sub;

pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub enum Shape3D {
    Point3D(Vector3D)
}

impl Sub<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    fn sub(self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Vector3D {
    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}
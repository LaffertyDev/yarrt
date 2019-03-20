use std::ops::{Add, Div, Mul, Sub, AddAssign, DivAssign, SubAssign, MulAssign};

use rand::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Vec3 {
    e0: f32,
    e1: f32,
    e2: f32,
}


impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { e0: x, e1: y, e2: z}
    }

    pub fn magnitude(&self) -> f32 {
        return (self.e0.powi(2) + self.e1.powi(2) + self.e2.powi(2)).sqrt();
    }

    pub fn magnitude_squared(&self) -> f32 {
        return self.e0.powi(2) + self.e1.powi(2) + self.e2.powi(2);
    }

    pub fn unit_vector(vector: &Vec3) -> Vec3 {
        let mag = vector.magnitude();
        return vector / mag;
    }

    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f32 {
        return lhs.e0 * rhs.e0 + lhs.e1 * rhs.e1 + lhs.e2 * rhs.e2;
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            e0: self.e1 * other.e2 - self.e2 * other.e1,
            e1: -(self.e0 * other.e2 - self.e2 * other.e0),
            e2: self.e0 * other.e1 - other.e1 * self.e0
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut point: Vec3;
        loop {
            point = Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()) * 2.0 -1.0;

            if point.magnitude_squared() < 1.0 {
                break;
            }
        }

        return point;
    }
    
    pub fn x(&self) -> f32 {
        self.e0
    }
    
    pub fn y(&self) -> f32 {
        self.e1
    }
    
    pub fn z(&self) -> f32 {
        self.e2
    }
    
    pub fn r(&self) -> f32 {
        self.e0
    }
    
    pub fn g(&self) -> f32 {
        self.e1
    }
    
    pub fn b(&self) -> f32 {
        self.e2
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2
        }
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 * other.e0,
            e1: self.e1 * other.e1,
            e2: self.e2 * other.e2
        }
    }
}

impl Div for &Vec3 {
    type Output = Vec3;

    fn div(self, other: &Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 / other.e0,
            e1: self.e1 / other.e1,
            e2: self.e2 / other.e2
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2
        }
    }
}


impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 * other.e0,
            e1: self.e1 * other.e1,
            e2: self.e2 * other.e2
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 / other.e0,
            e1: self.e1 / other.e1,
            e2: self.e2 / other.e2
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2
        }
    }
}

impl Add<f32> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Vec3 {
        Vec3 {
            e0: self.e0 + other,
            e1: self.e1 + other,
            e2: self.e2 + other
        }
    }
}

impl Sub<f32> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: f32) -> Vec3 {
        Vec3 {
            e0: self.e0 - other,
            e1: self.e1 - other,
            e2: self.e2 - other
        }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            e0: self.e0 / other,
            e1: self.e1 / other,
            e2: self.e2 / other
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            e0: self.e0 * other,
            e1: self.e1 * other,
            e2: self.e2 * other
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Vec3 {
        Vec3 {
            e0: self.e0 + other,
            e1: self.e1 + other,
            e2: self.e2 + other
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f32) -> Vec3 {
        Vec3 {
            e0: self.e0 - other,
            e1: self.e1 - other,
            e2: self.e2 - other
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            e0: self.e0 / other,
            e1: self.e1 / other,
            e2: self.e2 / other
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            e0: self.e0 * other,
            e1: self.e1 * other,
            e2: self.e2 * other
        }
    }
}


impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e0 -= other.e0;
        self.e1 -= other.e1;
        self.e2 -= other.e2;
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e0 += other.e0;
        self.e1 += other.e1;
        self.e2 += other.e2;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e0 *= other.e0;
        self.e1 *= other.e1;
        self.e2 *= other.e2;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e0 /= other.e0;
        self.e1 /= other.e1;
        self.e2 /= other.e2;
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, other: f32) {
        self.e0 -= other;
        self.e1 -= other;
        self.e2 -= other;
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        self.e0 += other;
        self.e1 += other;
        self.e2 += other;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.e0 *= other;
        self.e1 *= other;
        self.e2 *= other;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.e0 /= other;
        self.e1 /= other;
        self.e2 /= other;
    }
}
use std::ops::{Add, Div, Mul, Sub, AddAssign, DivAssign, SubAssign, MulAssign};

#[derive(Debug, PartialEq)]
pub struct vec3 {
    e0: f32,
    e1: f32,
    e2: f32,
}


impl vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        vec3 { e0: x, e1: y, e2: z}
    }

    pub fn magnitude(&self) -> f32 {
        return (self.e0.powi(2) + self.e1.powi(2) + self.e2.powi(2)).sqrt();
    }

    pub fn unit_vector(&self) -> vec3 {
        let mag = self.magnitude();
        return self / mag;
    }

    pub fn dot(&self, other: &vec3) -> f32 {
        return self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2;
    }

    pub fn cross(&self, other: &vec3) -> vec3 {
        vec3 {
            e0: self.e1 * other.e2 - self.e2 * other.e1,
            e1: -(self.e0 * other.e2 - self.e2 * other.e0),
            e2: self.e0 * other.e1 - other.e1 * self.e0
        }
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

impl Add for &vec3 {
    type Output = vec3;

    fn add(self, other: &vec3) -> vec3 {
        vec3 {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2
        }
    }
}

impl Mul for &vec3 {
    type Output = vec3;

    fn mul(self, other: &vec3) -> vec3 {
        vec3 {
            e0: self.e0 * other.e0,
            e1: self.e1 * other.e1,
            e2: self.e2 * other.e2
        }
    }
}

impl Div for &vec3 {
    type Output = vec3;

    fn div(self, other: &vec3) -> vec3 {
        vec3 {
            e0: self.e0 / other.e0,
            e1: self.e1 / other.e1,
            e2: self.e2 / other.e2
        }
    }
}

impl Sub for &vec3 {
    type Output = vec3;

    fn sub(self, other: &vec3) -> vec3 {
        vec3 {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2
        }
    }
}

impl Div<f32> for &vec3 {
    type Output = vec3;

    fn div(self, other: f32) -> vec3 {
        vec3 {
            e0: self.e0 / other,
            e1: self.e1 / other,
            e2: self.e2 / other
        }
    }
}

impl Mul<f32> for &vec3 {
    type Output = vec3;

    fn mul(self, other: f32) -> vec3 {
        vec3 {
            e0: self.e0 * other,
            e1: self.e1 * other,
            e2: self.e2 * other
        }
    }
}


impl SubAssign for vec3 {
    fn sub_assign(&mut self, other: vec3) {
        self.e0 -= other.e0;
        self.e1 -= other.e1;
        self.e2 -= other.e2;
    }
}

impl AddAssign for vec3 {
    fn add_assign(&mut self, other: vec3) {
        self.e0 += other.e0;
        self.e1 += other.e1;
        self.e2 += other.e2;
    }
}

impl MulAssign for vec3 {
    fn mul_assign(&mut self, other: vec3) {
        self.e0 *= other.e0;
        self.e1 *= other.e1;
        self.e2 *= other.e2;
    }
}

impl DivAssign for vec3 {
    fn div_assign(&mut self, other: vec3) {
        self.e0 /= other.e0;
        self.e1 /= other.e1;
        self.e2 /= other.e2;
    }
}
use std::ops;

use rand::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector3 {
    e0: f32,
    e1: f32,
    e2: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { e0: x, e1: y, e2: z}
    }

    pub fn magnitude(&self) -> f32 {
        return (self.e0.powi(2) + self.e1.powi(2) + self.e2.powi(2)).sqrt();
    }

    pub fn magnitude_squared(&self) -> f32 {
        return self.e0.powi(2) + self.e1.powi(2) + self.e2.powi(2);
    }

    pub fn unit_vector(vector: &Vector3) -> Vector3 {
        let mag = vector.magnitude();
        return vector / mag;
    }

    pub fn dot(lhs: &Vector3, rhs: &Vector3) -> f32 {
        return lhs.e0 * rhs.e0 + lhs.e1 * rhs.e1 + lhs.e2 * rhs.e2;
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            e0: self.e1 * other.e2 - self.e2 * other.e1,
            e1: -(self.e0 * other.e2 - self.e2 * other.e0),
            e2: self.e0 * other.e1 - other.e1 * self.e0
        }
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        let mut point: Vector3;
        loop {
            point = Vector3::new(random::<f32>(), random::<f32>(), random::<f32>()) * 2.0 -1.0;

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

/// Generates the operations for vector methods. `let result = my_vec_3 + my_other_Vector3`
/// Handles `Vector3, Vector3`, `Vector3, &Vector3`, `&Vector3, Vector3`, `&Vector3, &Vector3`
/// `Vector3_Vector3_op(ops::AddAssign, add_assign)` (note the camelcase add_assign name)
macro_rules! Vector3_Vector3_op {
    ($($path:ident)::+, $fn:ident) => {
        impl $($path)::+<Vector3> for Vector3 {
            type Output = Vector3;

            fn $fn(self, other: Vector3) -> Self::Output {
                Vector3 {
                    e0: self.e0.$fn(other.e0),
                    e1: self.e1.$fn(other.e1),
                    e2: self.e2.$fn(other.e2),
                }
            }
        }

        impl $($path)::+<&Vector3> for &Vector3 {
            type Output = Vector3;

            fn $fn(self, other: &Vector3) -> Self::Output {
                Vector3 {
                    e0: self.e0.$fn(other.e0),
                    e1: self.e1.$fn(other.e1),
                    e2: self.e2.$fn(other.e2),
                }
            }
        }

        impl $($path)::+<&Vector3> for Vector3 {
            type Output = Vector3;

            fn $fn(self, other: &Vector3) -> Self::Output {
                Vector3 {
                    e0: self.e0.$fn(other.e0),
                    e1: self.e1.$fn(other.e1),
                    e2: self.e2.$fn(other.e2),
                }
            }
        }

        impl $($path)::+<Vector3> for &Vector3 {
            type Output = Vector3;

            fn $fn(self, other: Vector3) -> Self::Output {
                Vector3 {
                    e0: self.e0.$fn(other.e0),
                    e1: self.e1.$fn(other.e1),
                    e2: self.e2.$fn(other.e2),
                }
            }
        }
    };
}

/// Generates the operations for vector method assignment. `my_vec += my_other_vec`
/// Handles `Vector3, Vector3` and `Vector3, &Vector3`
/// `Vector3_Vector3_opassign(ops::AddAssign, add_assign)` (note the camelcase add_assign name)
macro_rules! Vector3_Vector3_opassign {
    ($($path:ident)::+, $fn:ident) => {
        impl $($path)::+<Vector3> for Vector3 {
            fn $fn(&mut self, other: Vector3) {
                self.e0.$fn(other.e0);
                self.e1.$fn(other.e1);
                self.e2.$fn(other.e2);
            }
        }

        impl $($path)::+<&Vector3> for Vector3 {
            fn $fn(&mut self, other: &Vector3) {
                self.e0.$fn(other.e0);
                self.e1.$fn(other.e1);
                self.e2.$fn(other.e2);
            }
        }
    };
}

/// Generates the operations for method assignment. `my_vec += f32`
/// `Vector3_opassign(ops:AddAssign, add_assign)` (note the camelcase add_assign name)
macro_rules! Vector3_opassign {
    ($($path:ident)::+, $fn:ident, $ty:ty) => {
        impl $($path)::+<$ty> for Vector3 {
            fn $fn(&mut self, other: $ty) {
                self.e0.$fn(other);
                self.e1.$fn(other);
                self.e2.$fn(other);
            }
        }
    }
}

/// Generates the operations for the method. `let result = my_vec + 4f32`
/// Handles `Vector3, T`, `T, Vector3`, `&Vector3, T`, `T, &Vector3`
/// `Vector3_op!(ops:Add, add, f32)`
macro_rules! Vector3_op {
    ($($path:ident)::+, $fn:ident, $ty:ty) => {
        // impl ops::Add::add for Vector3
        impl $($path)::+<$ty> for Vector3 {
            type Output = Vector3;

            // fn add(self, other: f32) -> Self::Output
            fn $fn(self, other: $ty) -> Self::Output {
                Vector3 {
                    // e0: self.e0.add(other)
                    e0: self.e0.$fn(other),
                    e1: self.e1.$fn(other),
                    e2: self.e2.$fn(other),
                }
            }
        }

        impl $($path)::+<$ty> for &Vector3 {
            type Output = Vector3;

            fn $fn(self, other: $ty) -> Self::Output {
                Vector3 {
                    e0: self.e0.$fn(other),
                    e1: self.e1.$fn(other),
                    e2: self.e2.$fn(other),
                }
            }
        }

        impl $($path)::+<Vector3> for $ty {
            type Output = Vector3;

            fn $fn(self, other: Vector3) -> Self::Output {
                Vector3 {
                    e0: self.$fn(other.e0),
                    e1: self.$fn(other.e1),
                    e2: self.$fn(other.e2),
                }
            }
        }

        impl $($path)::+<&Vector3> for $ty {
            type Output = Vector3;

            fn $fn(self, other: &Vector3) -> Self::Output {
                Vector3 {
                    e0: self.$fn(other.e0),
                    e1: self.$fn(other.e1),
                    e2: self.$fn(other.e2),
                }
            }
        }
    }
}

macro_rules! Vector3_op_for {
    ($ty: ty) => {
        Vector3_op!(ops::Add, add, $ty);
        Vector3_op!(ops::Sub, sub, $ty);
        Vector3_op!(ops::Mul, mul, $ty);
        Vector3_op!(ops::Div, div, $ty);
        Vector3_opassign!(ops::AddAssign, add_assign, $ty);
        Vector3_opassign!(ops::SubAssign, sub_assign, $ty);
        Vector3_opassign!(ops::MulAssign, mul_assign, $ty);
        Vector3_opassign!(ops::DivAssign, div_assign, $ty);
    };
}

Vector3_op_for!(f32);
Vector3_Vector3_op!(ops::Add, add);
Vector3_Vector3_op!(ops::Sub, sub);
Vector3_Vector3_op!(ops::Mul, mul);
Vector3_Vector3_op!(ops::Div, div);
Vector3_Vector3_opassign!(ops::AddAssign, add_assign);
Vector3_Vector3_opassign!(ops::SubAssign, sub_assign);
Vector3_Vector3_opassign!(ops::MulAssign, mul_assign);
Vector3_Vector3_opassign!(ops::DivAssign, div_assign);
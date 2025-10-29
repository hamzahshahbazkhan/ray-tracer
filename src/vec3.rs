use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Default)]

pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    // distance from origin
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

pub type Point3 = Vec3;

// Implement display trait
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

// -Vec3
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

// Vec3+Vec#
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

// Vec 3 -= Vec3
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

//Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        Self::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x(), self * other.y(), self * other.z())
    }
}

// Vec3 *= Vec3
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t
    }
}

// Vec3 / Vec3
impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        )
    }
}

// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        Self::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

// Vec3 /= f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t
    }
}

// dot product
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

// cross product
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}

// unit vector
pub fn unit_vector(vec: Vec3) -> Vec3 {
    vec / vec.length()
}

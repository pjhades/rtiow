use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit(&self) -> Self {
        *self / self.len()
    }

    pub fn sqr(&self) -> f64 {
        self.dot(self)
    }

    pub fn len(&self) -> f64 {
        self.sqr().sqrt()
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::{approx_eq, ApproxEq, F64Margin};

    impl ApproxEq for Vec3 {
        type Margin = F64Margin;
        fn approx_eq<M: Into<Self::Margin>>(self, other: Self, _margin: M) -> bool {
            approx_eq!(f64, self.x, other.x, ulps = 1)
                && approx_eq!(f64, self.y, other.y, ulps = 1)
                && approx_eq!(f64, self.z, other.z, ulps = 1)
        }
    }

    #[test]
    fn test_arithmetic() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(-1.0, -2.0, -3.0);
        approx_eq!(Vec3, -&v1, v2, ulps = 1);
        approx_eq!(Vec3, v1 + &v2, Vec3::zero(), ulps = 1);
        approx_eq!(Vec3, v1 - &v2, Vec3::new(2.0, 4.0, 6.0), ulps = 1);
        approx_eq!(Vec3, v1 * -1.0, v2, ulps = 1);
        approx_eq!(Vec3, v1 / -1.0, v2, ulps = 1);

        v1 += &v2;
        approx_eq!(Vec3, v1, Vec3::zero(), ulps = 1);
        v1 -= &v2;
        approx_eq!(Vec3, v1, Vec3::new(1.0, 2.0, 3.0), ulps = 1);
        v1 *= -1.0;
        approx_eq!(Vec3, v1, v2, ulps = 1);
        v1 /= -1.0;
        approx_eq!(Vec3, v1, Vec3::new(1.0, 2.0, 3.0), ulps = 1);

        approx_eq!(f64, v1.dot(&v2), -14.0, ulps = 1);
        approx_eq!(Vec3, v1.cross(&v2), Vec3::zero(), ulps = 1);

        let v = Vec3::new(3.0, 4.0, 0.0);
        approx_eq!(f64, v.len(), 5.0, ulps = 1);
        approx_eq!(f64, v.sqr(), 25.0, ulps = 1);
        approx_eq!(Vec3, v.unit(), Vec3::new(0.6, 0.8, 0.0), ulps = 1);
    }
}

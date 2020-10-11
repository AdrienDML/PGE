use crate::math::vec3d::Vec3D;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quat{
    pub s : f64,
    pub v : Vec3D,
}

impl Quat{
    pub fn null() -> Self {
        Self {
            s : 0_f64,
            v : Vec3D::null(),
        }
    }

    pub fn sq_norm(&self) -> f64 {
        self.s * self.s + self.v.sq_norm()
    }

    pub fn norm(&self) -> f64 {
        self.sq_norm().sqrt()
    }

    pub fn normalize(&mut self) {
        let k = 1_f64/self.norm();
        self.s *= k;
        self.v *= k;
    }

    pub fn conj(&self)-> Self {
        Self {
            s : self.s,
            v : -self.v
        }
    }

    pub fn conjugate(&mut self) {
        self.v = -self.v;
    }

    pub fn inv(&self) -> Self {
        self.conj() * (1_f64 / self.sq_norm())
    }

    pub fn inverse(&mut self) {
        *self = self.inv();
    }



}

impl std::ops::Add for Quat {
    type Output = Quat;
    fn add(self, other : Self) -> Self {
        Self{
            s : self.s + other.s,
            v : self.v + other.v,
        }
    }
}
impl std::ops::AddAssign for Quat {
    fn add_assign(&mut self, other : Self) {
        *self = Self{
            s : self.s + other.s,
            v : self.v + other.v,
        };
    }
}
impl std::ops::Sub for Quat {
    type Output = Quat;
    fn sub(self, other : Self) -> Self {
        Self{
            s : self.s - other.s,
            v : self.v - other.v,
        }
    }
}
impl std::ops::Neg for Quat {
    type Output = Quat;
    fn neg(self) -> Self {
        Self{
            s : -self.s,
            v : -self.v,
        }
    }
}
impl std::ops::Mul<f64> for Quat {
    type Output = Quat;
    fn mul(self, other : f64) -> Self {
        Self{
            s : self.s * other,
            v : self.v * other,
        }
    }
}
impl std::ops::MulAssign<f64> for Quat {
    fn mul_assign(&mut self, other : f64) {
        *self = Self{
            s : self.s * other,
            v : self.v * other,
        };
    }
}
impl std::ops::Mul<Quat> for Quat {
    type Output = Quat;
    fn mul(self, other : Quat) -> Self {
        Self{
            s : self.s * other.s - self.v.dot(&other.v),
            v : self.s * other.v + other.s * self.v + self.v.cross(&other.v),
        }
    }
}
impl std::ops::MulAssign<Quat> for Quat {
    fn mul_assign(&mut self, other : Quat) {
        *self = Self {
            s : self.s * other.s - self.v.dot(&other.v),
            v : self.s * other.v + other.s * self.v + self.v.cross(&other.v),
        };
    }
}

impl std::fmt::Display for Quat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{};{}]", self.s, self.v)
    }
}


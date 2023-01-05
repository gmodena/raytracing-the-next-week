use std::ops::{Neg, Add, Sub, Div, Mul};
use rand::{thread_rng, Rng};

pub fn random_double(min: Option<f32>, max: Option<f32>) -> f32 {
    let mut rng = thread_rng();

    let a = min.unwrap_or(0.0);
    let b = max.unwrap_or(1.0);

    rng.gen_range(a..=b)
}


pub fn random(min: Option<f32>, max: Option<f32>) -> Vec3 {
    Vec3(random_double(min, max), random_double(min, max), random_double(min, max))
}

// Hack that approximates a Lambertian pattern
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random(Some(-1.0), Some(1.0));
        if p.length_squared() < 1.0 {
            return p
        }
    }
}

// True Lambertian Reflection. Generates more uniform rays of light.
pub fn random_unit_vector() -> Vec3 {
    Vec3::unit_vector(random_in_unit_sphere())
}

// Naive method; reflection does not depend on the angle from normal.
// scatter direction if uniform for all angles away from the hit point,
pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();

    if in_unit_sphere.dot(normal) > 0.0 { // In the same hemisphere as the normal
        return in_unit_sphere;
    }
    return -in_unit_sphere
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = Vec3(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }

    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn x(&self) -> f32 { self.0 }
    pub fn y(&self) -> f32 { self.1 }
    pub fn z(&self) -> f32 { self.2 }

    pub fn length_squared(self) -> f32 {
        f32::powi(self.0, 2) + f32::powi(self.1, 2) + f32::powi(self.2, 2)
    }
    pub fn length(self) -> f32 {
        f32::sqrt(self.length_squared())
    }
    pub fn dot(self, v: Vec3) -> f32 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3(self.1 * v.2 - self.2 * v.1, 
             self.2 * v.0 - self.0 * v.2, 
             self.0 * v.1 - self.1 * v.0)
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        return f32::abs(self.0) < s && f32::abs(self.1) < s && f32::abs(self.2) < s 
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - n * self.dot(n) * 2.0
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min( (-self).dot(n), 1.0);
        let r_out_perp = (self + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * (- f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())));
        r_out_perp + r_out_parallel
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Vec3) -> Self::Output {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> Self::Output {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}


impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, t: f32) -> Self::Output {
        self * (1.0/t)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, t: f32) -> Self::Output {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, v: Vec3) -> Self::Output {
        Vec3(self.0 * v.0, self.1 * v.1, self.2 * v.2)
    }
}

pub type Color = Vec3;
pub type Point3D = Vec3;

use crate::vec3::{Vec3, Point3D};

#[derive(Clone, Copy, Debug, PartialEq)] 
pub struct Ray(pub Point3D, pub Vec3, pub f32);

impl Ray {
    pub fn origin(&self) -> Point3D { self.0 }
    pub fn direction(&self) -> Vec3 { self.1 }
    pub fn time(&self) -> f32 { self.2 }
    /// A function 𝐏(𝑡)=𝐀+𝑡𝐛. Here 𝐏 is a 3D position along a line in 3D. 
    /// 𝐀 is the ray origin and 𝐛 is the ray direction. The ray parameter 𝑡 is a real number (double in the code).
    /// Arguments
    /// * `t`: postion of a point along the ray 𝐏(𝑡). For positive 𝑡, you get only 
    /// the parts in front of 𝐀, and this is what is often called a half-line or ray.
    pub fn at(self, t: f32) -> Vec3 {
        return self.origin() + self.direction() * t
    }
}

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, Hittable};
use crate::material::Material;

pub struct MovingSphere {
    pub center_0: Vec3,
    pub center_1: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
    pub time_0: f32,
    pub time_1: f32,
}

impl Hittable for MovingSphere {
    fn hit<'a>(&'a self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>> {
        let center =  self.center_0 + (self.center_1 - self.center_0) * ((r.time() - self.time_1) / (self.time_1 - self.time_0));

        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        
        let p = r.at(root);
        let t = root;
        // TODO in the book the following logic is implemented as a 
        // setter in the HitRecord struct. Refactor if we
        // need this logic in more places.
        let outward_normal = (p - center) / self.radius;
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face { 
            outward_normal 
        }   else {
            - outward_normal
        };
       
        let material = &*self.material;
        let record = HitRecord {
            p,
            normal,
            front_face,
            t,
            material
        };

        return Some(record);
    }
}

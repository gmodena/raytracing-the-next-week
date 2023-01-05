use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f32,
    pub material: &'a dyn Material
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}


/// Implement hittable_list and hittable_list::hit. The book declares 
/// a class that delegates to a vector container. Here we just implement 
/// the Hittable trait for Rust's Vec.
/// TODO Do we need shared_ptr semantics (Rc, Arc) at all?
impl Hittable for Vec<Box<dyn Hittable>> {
        fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record: Option<HitRecord> = None;

        for object in self {
            let has_hit = object.hit(r, t_min, closest_so_far);
            match has_hit {
                None => hit_record = hit_record,
                Some(record) => { 
                    hit_record = Some(record);
                    closest_so_far = record.t;
                }
            }
        }
        return hit_record;
    }
}

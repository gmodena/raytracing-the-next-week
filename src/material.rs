use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::vec3::{self, Vec3, random_in_unit_sphere};
use rand::random;

pub struct Scatter {
    pub r: Ray,
    pub attenuation: Vec3
}

pub trait Material {
    fn scatter(&self, r_in: Ray, record: HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Vec3
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32
}

pub struct Dielectric {
    pub index_of_refraction: f32
}

impl Dielectric {
    /// Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 * r0 + (1.0 - r0 * r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, record: HitRecord) -> Option<Scatter> {
        let scatter_direction = if (record.normal + vec3::random_unit_vector()).near_zero() {
            record.normal
        } else {
            record.normal + vec3::random_unit_vector()
        };

        let scattered = Ray(record.p, scatter_direction, _r_in.time());
        let attenuation = self.albedo;

        Some(Scatter{r: scattered, attenuation})
    }
} 

impl Material for Metal {
    fn scatter(&self, r_in: Ray, record: HitRecord) -> Option<Scatter> {
        let reflected = r_in.direction().unit_vector().reflect(record.normal);
        let scattered = Ray(record.p, reflected + random_in_unit_sphere() * f32::min(self.fuzz, 1.0), r_in.time());
        let attenuation = self.albedo;

        if scattered.direction().dot(record.normal) > 0.0 {
            Some(Scatter{r: scattered, attenuation})
        } else {
            None
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, record: HitRecord) -> Option<Scatter> { 
        let attenuation = Vec3(1.0, 1.0, 1.0);
        let refraction_ratio = if record.front_face { 1.0 / self.index_of_refraction } else { self.index_of_refraction};
        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = f32::min( (-unit_direction).dot(record.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random::<f32>() {
            unit_direction.reflect(record.normal)
        } else {
            unit_direction.refract(record.normal, refraction_ratio)
        };

        let scattred = Ray(record.p, direction, r_in.time());

        Some(Scatter { r: scattred, attenuation })
    }
}

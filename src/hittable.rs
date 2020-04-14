use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ref_idx: f32 },
}

pub fn scatter(
    material: &Material,
    ray_in: &Ray,
    rec: &HitRecord,
    attentuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            *scattered = Ray::ray(rec.p, target - rec.p);
            *attentuation = albedo;
            return true;
        }
        &Material::Metal { albedo, fuzz } => {
            let mut f = 1.0;
            if fuzz < 1.0 {
                f = fuzz;
            }
            let reflected = reflect(Vec3::unit_vector(&ray_in.direction()), rec.normal);
            *scattered = Ray::ray(rec.p, reflected + fuzz * random_in_unit_sphere());
            *attentuation = albedo;
            return Vec3::dot(&scattered.direction(), &rec.normal) > 0.0;
        }
        &Material::Dielectric { ref_idx } => {
            let mut outward_normal = Vec3::default();
            let reflected = reflect(ray_in.direction(), rec.normal);
            let mut ni_over_nt = 0.0;
            *attentuation = Vec3::new(1.0, 1.0, 1.0);

            let mut refracted = Vec3::default();

            let mut reflect_prob = 0.0;
            let mut cosine = 0.0;

            if Vec3::dot(&ray_in.direction(), &rec.normal) > 0.0 {
                outward_normal = -rec.normal;
                ni_over_nt = ref_idx;
                cosine = ref_idx * Vec3::dot(&ray_in.direction(), &rec.normal)
                    / ray_in.direction().length();
            } else {
                outward_normal = rec.normal;
                ni_over_nt = 1.0 / ref_idx;
                cosine = -Vec3::dot(&ray_in.direction(), &rec.normal) / ray_in.direction().length();
            }

            if refract(
                &ray_in.direction(),
                &outward_normal,
                ni_over_nt,
                &mut refracted,
            ) {
                reflect_prob = schlick(cosine, ref_idx);
            } else {
                reflect_prob = 1.0;
            }

            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < reflect_prob {
                *scattered = Ray::ray(rec.p, reflected);
            } else {
                *scattered = Ray::ray(rec.p, refracted);
            }

            return true;
        }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(&v, &n) * n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt();
        return true;
    } else {
        return false;
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vec3::new(1.0, 1.0, 1.0);

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

// A simple ray tracer built by translating the tutorial
// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// to Rust

// TODO: pull these mod declarations out into a separate lib.rs
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use material::{scatter, Material};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

use rand::prelude::*;

fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        let mut scattered = Ray::ray(Vec3::default(), Vec3::default());
        let mut attentuation = Vec3::default();

        if depth < 50 && scatter(&rec.material, r, &rec, &mut attentuation, &mut scattered) {
            return attentuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

// Construct a ppm file for image data, e.g.
// P3
// 3 2
// 255
// 255 0 0    0 255 0  0 0 255
// 255 255 0  255 255  255 0 0 0
//
fn main() {
    //println!("A raytracer in Rust!");

    let width = 400;
    let height = 200;
    let samples = 100;
    let max_value = 255;

    // this is so helpful, https://stackoverflow.com/questions/46965867/rust-borrowed-value-must-be-valid-for-the-static-lifetime
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    // this is just the worst syntax
    list.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian {
            albedo: Vec3::new(0.1, 0.2, 0.5),
        },
    )));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        },
    )));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal {
            albedo: Vec3::new(0.8, 0.6, 0.2),
            fuzz: 0.3,
        },
    )));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Dielectric { ref_idx: 1.5 },
    )));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Material::Dielectric { ref_idx: 1.5 },
    )));
    let world = HittableList::new(list);

    let cam = Camera::camera();
    let mut rng = rand::thread_rng();

    // we use a plan txt ppm to start building images
    println!("P3\n{} {}\n{}", width, height, max_value);

    for j in (0..height).rev() {
        for i in 0..width {
            let mut col = Vec3::default();

            for _ in 0..samples {
                let u = (i as f32 + rng.gen::<f32>()) / width as f32;
                let v = (j as f32 + rng.gen::<f32>()) / height as f32;

                let r = &cam.get_ray(u, v);
                col = col + color(&r, &world, 0);
            }

            col = col / samples as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

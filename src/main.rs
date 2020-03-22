// A simple ray tracer built by translating the tutorial
// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// to Rust

// TODO: pull these mod declarations out into a separate lib.rs
mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

use rand::prelude::*;

fn color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();

    if world.hit(&r, 0.0, std::f32::MAX, &mut rec) {
        let target = rec.p() + rec.normal() + random_in_unit_sphere();
        return 0.5 * color(&Ray::ray(rec.p(), target - rec.p()), &world);
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    let mut rng = rand::thread_rng();

    loop {
        p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vec3::new(1.0, 1.0, 1.0);

        if p.squared_length() < 1.0 {
            return p;
        }
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

    let width = 200;
    let height = 100;
    let samples = 100;
    let max_value = 255;

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
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
                col = col + color(&r, &world);
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

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
use rayon::prelude::*;
use std::time;

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

    let width = 1600; // 1600;
    let height = 800; // 800;
    let samples = 100; //100;
    let max_value = 255;

    // this is so helpful, https://stackoverflow.com/questions/46965867/rust-borrowed-value-must-be-valid-for-the-static-lifetime
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    list.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let centre = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (centre - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    list.push(Box::new(Sphere::sphere(
                        centre,
                        0.2,
                        Material::Lambertian { albedo },
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_init(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    list.push(Box::new(Sphere::sphere(
                        centre,
                        0.2,
                        Material::Metal { albedo, fuzz },
                    )));
                } else {
                    list.push(Box::new(Sphere::sphere(
                        centre,
                        0.2,
                        Material::Dielectric { ref_idx: 1.5 },
                    )));
                }
            }
        }
    }
    list.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric { ref_idx: 1.5 },
    )));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    )));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    )));

    let world = HittableList::new(list);

    let aspect_ratio = width as f32 / height as f32;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = 10.0;
    let apeture = 0.1;

    let cam = Camera::camera(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        apeture,
        dist_to_focus,
    );

    // collect a vector of rgb tuples for each pixel (width * height)
    let mut screen = vec![(0u32, 0u32, 0u32); width * height];
    let start = time::Instant::now();

    screen
        .iter_mut()
        // .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| {
            let mut rng = rand::thread_rng();
            let column = index % width; // column is the 'count' within a row
            let row = height - index / width; // the row number

            // println!("Row: {}, column: {}", row, column);
            let mut col = Vec3::default();

            for _ in 0..samples {
                let u = (column as f32 + rng.gen::<f32>()) / width as f32;
                let v = (row as f32 + rng.gen::<f32>()) / height as f32;

                let r = &cam.get_ray(u, v);
                col = col + color(&r, &world, 0);
            }

            col = col / samples as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = (255.99 * col.r()) as u32;
            let ig = (255.99 * col.g()) as u32;
            let ib = (255.99 * col.b()) as u32;

            // no alpha, just 24 bit colour
            *pixel = (ir, ig, ib);
        });

    eprintln!("Number of pixels generated: {}", screen.len());

    // we use a plain txt ppm to start building images
    println!("P3\n{} {}\n{}", width, height, max_value);

    for (r, g, b) in screen {
        println!("{}, {}, {}", r, g, b);
    }

    let duration = time::Instant::now() - start;
    eprintln!("Render elapsed time: {:?}", duration);
}

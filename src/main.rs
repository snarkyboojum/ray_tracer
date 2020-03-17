// A simple ray tracer built by translating the tutorial
// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// to Rust

// TODO: pull these mod declarations out into a separate lib.rs
mod vec3;

use vec3::Vec3;

// Construct a ppm file for image data, e.g.
// P3
// 3 2
// 255
// 255 0 0    0 255 0  0 0 255
// 255 255 0  255 255  255 0 0 0
//
fn write_ppm(w: i32, h: i32, max_value: i32) {
    println!("P3\n{} {}\n{}", w, h, max_value);

    for j in (0..h).rev() {
        for i in 0..w {
            let r = i as f32 / w as f32;
            let g = j as f32 / h as f32;
            let b: f32 = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main() {
    //println!("A raytracer in Rust!");

    let width = 200;
    let height = 100;
    let max_value = 255;

    // we use a plan txt ppm to start building images
    write_ppm(width, height, max_value);

    let v1 = Vec3::new(1f32, 2f32, 6f32);
    let v2 = Vec3::new(2f32, 6f32, 8f32);

    let v3 = v1 + v2;
    println!("Added v1 and v2: {:?}", v3);
}

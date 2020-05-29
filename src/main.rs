use std::io::Write;
use std::fs::File;
use std::f64;
use rand::Rng;

use raytracer::structures::ray::Ray;
use raytracer::structures::vec3::Vec3;
use raytracer::objects::sphere::Sphere;
use raytracer::objects::hitable::{Hit, HitRecord, HitableList};
use raytracer::objects::camera::Camera;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p: Vec3 = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) -
                    Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 { return p; }
    }
}

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    let mut rec = HitRecord::default();

    if world.hit(r, 0.0001, f64::MAX, &mut rec) {
        let target: Vec3 = rec.p + rec.normal + random_in_unit_sphere();
        return color(&Ray::new(rec.p, target - rec.p), world) * 0.5;
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut file = File::create("test.ppm").unwrap();

    let nx = 200;
    let ny = 100;
    let ns: isize = 100;

    let header = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(header.as_bytes()).unwrap();

    let list = HitableList::new(vec![
        Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    let cam = Camera::new();

    for j in (0..ny - 1).rev() {
        if j % 30 == 0 { println!("Y: {}", j); }
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u: f64 = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v: f64 = (j as f64 + rng.gen::<f64>()) / ny as f64;

                let r = cam.get_ray(u, v);
                let _p: Vec3 = r.point_at_parameter(2.0);
                col += color(&r, &list);
            }

            col /= ns as f64;
            
            // Do gamma correction
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

            let ir = (255.99 * col[0]) as isize;
            let ig = (255.99 * col[1]) as isize;
            let ib = (255.99 * col[2]) as isize;

            let row = format!("{:.0} {:.0} {:.0}\n", ir, ig, ib);
            file.write(row.as_bytes()).unwrap();
        }
    }
}

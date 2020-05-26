use std::io::Write;
use std::fs::File;
use std::f32;

mod data_structures;
use data_structures::Ray;
use data_structures::Vec3;
use data_structures::Sphere;

use data_structures::Hit;
use data_structures::HitRecord;
use data_structures::HitableList;

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    let mut rec = HitRecord::default();

    if world.hit(r, 0.0, f32::MAX, &mut rec) {
        return Vec3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        ) * 0.5;
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn main() {
    let mut file = File::create("test.ppm").unwrap();

    let nx = 200;
    let ny = 100;

    let header = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(header.as_bytes()).unwrap();

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let list = HitableList::new(vec![
        Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            let col = color(&r, &list);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            let row = format!("{:.0} {:.0} {:.0}\n", ir, ig, ib);
            file.write(row.as_bytes()).unwrap();
        }
    }
}

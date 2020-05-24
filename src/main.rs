use std::io::Write;
use std::fs::File;

mod data_structures;
use data_structures::Ray;
use data_structures::Vec3;

fn color(r: &Ray) -> Vec3 {
    let center = Vec3::new(0.0, 0.0, -1.0);
    let radius = 0.5;

    let t = hit_sphere(&center, radius, r);
    if t > 0.0 {
        let normal = Vec3::unit_vector(&(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)));
        return Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
 
// Checks if a ray is hitting a given sphere
fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {
    let ray_to_center = *ray.origin() - *center;
    let a = Vec3::dot(ray.direction(), ray.direction());
    let b = 2.0 * Vec3::dot(&ray_to_center, ray.direction());
    let c = Vec3::dot(&ray_to_center, &ray_to_center) - radius * radius;
    let discriminant = b * b - a * 4.0 * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a)
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

    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            let col = color(&r);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            let row = format!("{:.0} {:.0} {:.0}\n", ir, ig, ib);
            file.write(row.as_bytes()).unwrap();
        }
    }
}

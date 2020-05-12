use std::io::Write;
use std::fs::File;

mod data_structures;

fn main() {
    let mut file = File::create("test.txt").unwrap();

    let nx = 200;
    let ny = 100;

    let header = format!("P3\n{} {}\n255\n", nx, ny);
    file.write(header.as_bytes()).unwrap();

    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let col = data_structures::Vec3::new(
                i as f32 / nx as f32,
                j as f32 / ny as f32,
                0.2
            );

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            let row = format!("{:.0} {:.0} {:.0}\n", ir, ig, ib);
            file.write(row.as_bytes()).unwrap();
        }
    }
}

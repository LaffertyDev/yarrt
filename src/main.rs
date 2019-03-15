mod vector;

fn main() {
    let num_rows = 200;
    let num_cols = 100;
    println!("P3\n{} {}\n255", num_rows, num_cols);
    for y in (0..num_cols).rev() {
        for x in 0..num_rows {
            let vec = vector::Vec3::new(
                x as f32 / num_rows as f32,
                y as f32 / num_cols as f32,
                0.2f32
            );
            let ir = (255.99 * vec.r()) as i32;
            let ig = (255.99 * vec.g()) as i32;
            let ib = (255.99 * vec.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main() {
    let num_rows = 200;
    let num_cols = 100;
    println!("P3\n{} {}\n255", num_rows, num_cols);
    for y in (0..num_cols).rev() {
        for x in 0..num_rows {
            let r = x as f32 / num_rows as f32;
            let g = y as f32 / num_cols as f32;
            let b = 0.2f32;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

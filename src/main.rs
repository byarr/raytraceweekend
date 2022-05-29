fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{image_width} {image_height}\n255");
    for j in (0..image_height).rev() {
        eprintln!("Scan lines remaining {j}");
        for i in 0..image_width {
            let r = i as f32 / (image_width as f32-1.0);
            let g = j as f32 / (image_height as f32-1.0);
            let b = 0.25;

            let r = (255.999 * r) as i32;
            let g = (255.999 * g) as i32;
            let b = (255.999 * b) as i32;
            println!("{r} {g} {b}");

        }
    }
    eprintln!("Done!")
}

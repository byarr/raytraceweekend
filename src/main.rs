use raytraceweekend::Colour;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{image_width} {image_height}\n255");
    for j in (0..image_height).rev() {
        eprintln!("Scan lines remaining {j}");
        for i in 0..image_width {


            // color pixel_color(double(i)/(image_width-1), double(j)/(image_height-1), 0.25);
            let r = i as f64 / (image_width as f64-1.0);
            let g = j as f64 / (image_height as f64-1.0);
            let b = 0.25;

            let colour = Colour::new(r, g, b);

            println!("{colour}");

        }
    }
    eprintln!("Done!")
}

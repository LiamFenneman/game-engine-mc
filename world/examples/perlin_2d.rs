fn main() {
    const SIZE: usize = 512;
    let perlin = ge_world::noise::Noise::new(0, 1.0, 1.0, 0.0);

    let mut samples = Vec::with_capacity(SIZE * SIZE);
    for y in 0..SIZE {
        for x in 0..SIZE {
            samples.push(perlin.sample_2d(cgmath::Vector2::new(x as f64, y as f64)));
        }
    }

    let buffer = samples
        .iter()
        .flat_map(|s| {
            let r = (s * 255.0) as u8;
            [r, r, r]
        })
        .collect::<Vec<_>>();

    image::save_buffer(
        "images/perlin_2d.png",
        &buffer,
        SIZE as u32,
        SIZE as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();

    println!("Saved image to 'images/perlin_2d.png'");
}

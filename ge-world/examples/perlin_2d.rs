use cgmath::vec2;
use ge_world::noise::NoiseField;

fn main() {
    const SIZE: usize = 512;
    let seed = rand::random();
    let noise_field = NoiseField::new(seed, 5, 1.0, 0.5, 2.0, 0.5);

    let mut samples = Vec::with_capacity(SIZE * SIZE);
    for y in 0..SIZE {
        for x in 0..SIZE {
            samples.push(noise_field.sample_2d(
                vec2(x as f64 - 256.0, y as f64 - 256.0),
                None,
                Some(vec2(SIZE as f64 / 2.0, SIZE as f64 / 2.0)),
            ));
        }
    }

    let buffer = samples
        .iter()
        .flat_map(|s| {
            let r = ((s + 1.0) / 2.0 * 255.0) as u8;
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

    println!("Seed: {seed}");
    println!("Saved image to 'images/perlin_2d.png'");
}

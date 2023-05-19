use ge_world::noise::Noise;

const FILE: &str = "images/perlin_2d.png";
const SIZE: usize = 512;

const OFFSET: f32 = -256.0;

fn main() {
    let noise = Noise::new(5, 3.0, 1.0, 2.0, 0.5);
    let mut samples = Vec::with_capacity(SIZE * SIZE);
    for y in 0..SIZE {
        for x in 0..SIZE {
            samples.push(noise.fbm(
                x as f32 / SIZE as f32 + OFFSET,
                y as f32 / SIZE as f32 + OFFSET,
                0.0,
            ));
        }
    }

    println!("Generated {} samples", samples.len());

    let buffer = samples
        .iter()
        .flat_map(|s| {
            let r = ((s + 1.0) / 2.0 * 255.0) as u8;
            [r, r, r]
        })
        .collect::<Vec<_>>();

    image::save_buffer(
        FILE,
        &buffer,
        SIZE as u32,
        SIZE as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();

    println!("Saved image to '{FILE}'");
}

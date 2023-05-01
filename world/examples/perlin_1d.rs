use ge_world::noise::Noise;
use plotters::prelude::*;

fn main() {
    let root = BitMapBackend::new("images/perlin_1d.png", (800, 600)).into_drawing_area();

    const MIN: i32 = -10;
    const MAX: i32 = 10;
    const SAMPLES: i32 = 100;
    let perlin = Noise::new(rand::random(), 5, 1.0, 0.5, 0.0);

    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d((MIN as f64)..(MAX as f64), -1f64..1f64)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let samples = ((MIN * SAMPLES)..=(MAX * SAMPLES))
        .map(|x| x as f64 / SAMPLES as f64)
        .map(|x| (x, perlin.sample_1d(x)))
        .collect::<Vec<_>>();

    chart.draw_series(LineSeries::new(samples, &RED)).unwrap();

    root.present().unwrap();
    println!("Saved image to 'images/perlin_1d.png'");
}

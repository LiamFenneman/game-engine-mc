use ge_world::noise::Noise;
use plotters::prelude::*;

fn main() {
    let root = BitMapBackend::new("images/perlin_1d.png", (640, 480)).into_drawing_area();

    const MIN: i32 = -10;
    const MAX: i32 = 10;
    const SAMPLES: i32 = 100;
    let perlin = Noise::new(0, 1, 1.0, 1.0, 0.0);
    let perlin2 = Noise::new(0, 2, 1.0, 1.25, 0.0);
    let perlin5 = Noise::new(0, 5, 1.0, 2.5, 0.0);

    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d((MIN as f64)..(MAX as f64), 0f64..1f64)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            ((MIN * SAMPLES)..=(MAX * SAMPLES))
                .map(|x| x as f64 / SAMPLES as f64)
                .map(|x| (x, perlin.sample_1d(x))),
            &BLACK,
        ))
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            ((MIN * SAMPLES)..=(MAX * SAMPLES))
                .map(|x| x as f64 / SAMPLES as f64)
                .map(|x| (x, perlin2.sample_1d(x))),
            &BLUE,
        ))
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            ((MIN * SAMPLES)..=(MAX * SAMPLES))
                .map(|x| x as f64 / SAMPLES as f64)
                .map(|x| (x, perlin5.sample_1d(x))),
            &RED,
        ))
        .unwrap();

    root.present().unwrap();
    println!("Saved image to 'images/perlin_1d.png'");
}

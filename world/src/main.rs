use cgmath::Vector3;
use ge_world::{
    gen::{self, WorldGenerator},
    noise::Noise,
    util, World,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;
    let root = BitMapBackend::new("images/0.png", (640, 480)).into_drawing_area();

    const MIN: i32 = -10;
    const MAX: i32 = 10;
    const SAMPLES: i32 = 10;
    let perlin = Noise::new(0, 1.0, 1.0, 0.0);

    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d((MIN as f64)..(MAX as f64), 0f64..1f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        ((MIN * SAMPLES)..=(MAX * SAMPLES))
            .map(|x| x as f64 / SAMPLES as f64)
            .map(|x| (x, perlin.sample(x))),
        &RED,
    ))?;

    root.present()?;

    Ok(())
}

#[allow(dead_code)]
fn world_gen_test() {
    let world_gen = gen::RandomWorldGenerator {
        world_size: WORLD_SIZE,
    };
    let renderer = TestRenderer::new(world_gen.generate());
    renderer.render(Some(1));
    // renderer.render(None);

    let p = Vector3::new(1, 0, 1);
    let i = util::pos_to_idx(p, WORLD_SIZE);
    let t = renderer.0.blocks[i].ty;
    println!("pos = {p:?} -> idx = {i:?} -> {t}");

    let i = (WORLD_SIZE * WORLD_SIZE - 1) as usize;
    let p = util::idx_to_pos(i, WORLD_SIZE);
    let t = renderer.0.blocks[i].ty;
    println!("idx = {i:?} -> pos = {p:?} -> {t}");
}

const WORLD_SIZE: u32 = 8;

#[derive(Debug, Clone)]
pub struct TestRenderer(World);

impl TestRenderer {
    #[must_use]
    pub fn new(world: World) -> Self {
        Self(world)
    }

    pub fn render(&self, limit_y: Option<u32>) {
        let limit_y = match limit_y {
            Some(limit_y) => limit_y,
            None => WORLD_SIZE,
        };

        for y in 0..limit_y.min(WORLD_SIZE) {
            for z in 0..WORLD_SIZE {
                for x in 0..WORLD_SIZE {
                    let block = self.0.blocks.iter().find(|block| {
                        block.position.x == x && block.position.y == y && block.position.z == z
                    });
                    match block {
                        Some(block) => print!("{}", block.ty),
                        None => print!(" "),
                    }
                }
                println!();
            }
            println!();
        }
    }
}

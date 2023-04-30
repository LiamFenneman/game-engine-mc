use cgmath::Vector3;
use ge_world::*;

fn main() {
    let world_gen = RandomWorldGenerator {
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

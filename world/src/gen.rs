use crate::{Block, World};
use cgmath::Vector3;
use rand::{Rng, SeedableRng};

/// A `WorldGenerator` is a trait that generates a `World`.
pub trait WorldGenerator {
    fn generate(&self) -> World;
}

pub struct RandomWorldGenerator {
    pub world_size: u32,
}

impl WorldGenerator for RandomWorldGenerator {
    #[allow(clippy::cast_precision_loss)]
    fn generate(&self) -> World {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(2);

        // generate a plane of grass
        let mut blocks = vec![];
        for y in 0..self.world_size {
            for z in 0..self.world_size {
                for x in 0..self.world_size {
                    blocks.push(Block {
                        ty: rng.gen(),
                        position: Vector3::new(x, y, z),
                    });
                }
            }
        }

        return World { blocks };
    }
}

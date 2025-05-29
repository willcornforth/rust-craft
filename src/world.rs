use bevy::prelude::*;

const CHUNK_SIZE: usize = 16;

enum BlockType {
    Air,
    Grass,
    Dirt,
}

#[derive(Clone, Copy, Component)]
struct Block {
 block_type: BlockType
}

impl Block {
    fn new(block_type: BlockType) -> Self {
        Self { block_type }
    }

    pub fn is_solid() -> bool {
        Self.block_type == BlockType::Air
    }
}

#[derive(Clone, Copy, Component)]
struct Chunk {
    // Flatmap for storing blocks
    blocks: [Block; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
    size: usize
}

impl Chunk {
    fn new(size: usize) -> Self {
        Self {
            blocks: [Block { block_type: BlockType::Air }; size * size * size],
            size
        }
    }

    // Getters and setters for blocks. Calculates index in the array using x, y, and z.
    fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[x + y * self.size + z * self.size * self.size]
    }
    fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        self.blocks[x + y * self.size + z * self.size * self.size] = block;
    }
    // Responsible for geenrating an efficient mesh of all the visible blocks
    // within the chunk.
}

#[derive(Component)]
struct World {
    // Hashmap for each chunk within the world.
    chunks: HashMap<(i32, i32), Chunk>
}

impl World {
    fn new() -> Self {
        Self {
            chunks: HashMap::new()
        }
    }
}
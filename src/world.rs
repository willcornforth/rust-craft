use std::collections::HashMap;
use bevy::prelude::*;

const CHUNK_SIZE: usize = 16;

enum BlockType {
  Air,
  Grass,
  Dirt,
}

#[derive(Component)]
struct Block {
  block_type: BlockType
}

#[derive(Component)]
struct Chunk {
  // Map of block coords to block in chunk.
  blocks: [Block; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
  size: i32,
}

#[derive(Component)]
struct World {
  // Map of world chunks, only need to store the x, y coords.
  chunks: HashMap<(i32, i32), Chunk>  // FLAT MAP INSTEAD.
}
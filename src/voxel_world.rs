use std::collections::HashMap;
use bevy::math::Vec3;
use bevy::prelude::{Cuboid, Mesh};


#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum VoxelType {
  Air =0,
  Stone =1,
  Grass =2,
}
const CHUNK_SIZE: usize = 16; // 16x16x16 voxels per chunk

pub(crate) struct VoxelWorld {
  // Store only active chunks
  chunks: HashMap<(i32, i32, i32), Chunk>,
}

impl VoxelWorld {
  pub(crate) fn new() -> Self {
    Self {
      chunks: HashMap::new(),
    }
  }
}

pub(crate) struct Chunk {
  // Flat array storage
  voxels: [VoxelType; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
  size: usize,
}

impl Chunk {
  pub(crate) fn new() -> Self {
    Self {
      size: CHUNK_SIZE,
      voxels: [VoxelType::Stone; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
    }
  }

  fn get_voxel(&self, x: usize, y: usize, z: usize) -> VoxelType {
    if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
      let index = x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE;
      self.voxels[index]
    } else {
      VoxelType::Air // Default for out of bounds
    }
  }

  fn set_voxel(&mut self, x: usize, y: usize, z: usize, voxel: VoxelType) {
    if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
      let index = x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE;
      self.voxels[index] = voxel;
    }
  }

  pub(crate) fn generate_mesh(&self) -> Vec<(Mesh, Vec3)> {
    let mut meshes = Vec::new();

    for x in 0..self.size {
      for y in 0..self.size {
        for z in 0..self.size {
          let voxel = self.get_voxel(x, y, z);

          // Skip air blocks
          if voxel == VoxelType::Air {
            continue;
          }

          meshes.push((
            Mesh::from( Cuboid::new(1.0,1.0,1.0) ), 
            Vec3::new(x as f32, y as f32, z as f32))
          );
        }
      }
    }
    meshes
  }
}
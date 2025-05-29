use world::*;
use bevy::prelude::*;

#[derive(Component)]
struct ChunkRenderer {
    mesh_handle: Option<Handle<Mesh>>,
    material_handle: Option<Handle<StandardMaterial>>,
    dirty: bool // Used for marking chunks for re-rendering.
}

impl ChunkRenderer {
    pub fn new(chunk: &Chunk) {
        dirty = true;
        generate_mesh(chunk)
    }
    pub fn generate_mesh(chunk: &Chunk) -> Option<Mesh> {
      // Generate a mesh using the chunk data.
      let mut positions = Vec::new();
      let mut normals = Vec::new();
      let mut uvs = Vec::new();
      let mut indices = Vec::new();

      let mut face_count = 0;

      for x in 0..chunk.size {
        for y in 0..chunk.size {
          for z in 0..chunk.size {
            let block = chunk.get_block(x, y, z);

            if(!block.is_solid()) { continue }


          }
        }
      }
    }
}

fn chunk_render_system(
    mut chunks: Query<(&Chunk, &mut ChunkRenderer)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
  for (chunk, mut renderer) in chunks.iter_mut() {
    if (!renderer.dirty) { continue }

    if let Some(mesh) = meshes.get_mut(&renderer.mesh_handle) {
      // If render is dirty, update the existing mesh with a newly generated one.
      *mesh = renderer.generate_mesh(chunk);
      renderer.dirty = false;
    } else if (renderer.mesh_handle.is_none()) {
      // No mesh has been created yet, so create one.
      renderer.mesh_handle = meshes.add(Some(renderer.generate_mesh(chunk)));

      if(renderer.material_handle.is_none()) {
        // Create new material from colour value using the Into trait.
        renderer.material_handle = materials.add(Color::rgb(0.8, 0.7, 0.6).into());
      }

    }
  }
}
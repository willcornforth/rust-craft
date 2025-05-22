mod voxel_world;
mod camera;

use voxel_world::{Chunk};
use bevy::prelude::*;
use rand::{rng, Rng, RngCore};
use crate::camera::GameCamera;

fn main() {
  println!("Starting application...");

  // Setup bevy
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

fn setup(
  mut commands: Commands,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>
) {
  // Create the camera and borrow the commands variable to spawn it.
  let camera = GameCamera::new(Vec3::new(25.0, 5.0, 50.0));
  camera.spawn(&mut commands);
  
  // Test chunk logic...
  let chunk = Chunk::new();
  let chunk_meshes = chunk.generate_mesh();

  // Add the meshes to the scene
  for (mesh, position) in chunk_meshes {
    let mesh_handle = meshes.add(mesh);

    let mut rng = rng();
    let random_r = rng.random_range(0.0..1.0);
    let random_g = rng.random_range(0.0..1.0);
    let random_b = rng.random_range(0.0..1.0);

    // Create an entity with the mesh
    commands.spawn((
      Mesh3d(mesh_handle),
      MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Color::linear_rgb(random_r, random_g, random_b),
        ..default()
      })),
      Transform::from_translation(position),
    ));
  }
}
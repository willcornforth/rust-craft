mod voxel_world;

use voxel_world::{Chunk, VoxelWorld};
use bevy::prelude::*;

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
  mut meshes: ResMut<Assets<Mesh>>,
) {
  let camera_transform = Transform::from_xyz(50.0, 5.0, 50.0)
    .looking_at(Vec3::ZERO, Vec3::Y);

  // Create a 3D camera.
  commands.spawn((
    Camera3d::default(),
    camera_transform
  ));

  // Setup a point light for the scene.
  commands.spawn((PointLight::default(), camera_transform));

  let chunk = Chunk::new();
  let chunk_meshes = chunk.generate_mesh();
  dbg!(&chunk_meshes.len()); // Print mesh size... (Currently, there is one mesh for every voxel.)

  // Add the meshes to the scene
  for mesh in chunk_meshes {
    let mesh_handle = meshes.add(mesh);

    // Create an entity with the mesh
    commands.spawn((
      Mesh3d(mesh_handle),
      MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Color::linear_rgb(1.0, 1.0, 1.0),
        ..default()
      }))
    ));
  }
}

fn create_cube_mesh() -> Mesh {
  Mesh::from(Cuboid::new(1.0, 1.0, 1.0))
}
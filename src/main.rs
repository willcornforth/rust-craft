mod voxel_world;
use voxel_world::{Chunk};

mod camera;
mod entity_components;

use camera::GameCamera;

use bevy::prelude::*;
use rand::{rng, Rng};

fn main() {
  println!("Starting application...");

  // Setup bevy
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, camera::mouse_look_system)
    .run();
}

fn test_chunk_logic(
  commands: &mut Commands,
  materials: &mut ResMut<Assets<StandardMaterial>>,
  meshes: &mut ResMut<Assets<Mesh>>
) {
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

fn setup(
  mut commands: Commands,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>
) {
  test_chunk_logic(&mut commands, &mut materials, &mut meshes);

  // Create the camera and borrow the commands variable to spawn it.
  let camera = GameCamera::new();

  camera.spawn(&mut commands, Transform::from_translation(Vec3::new(0.0, 0.0, 100.0))
    .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y));
}
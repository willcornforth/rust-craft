use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct GameCamera {
  position_transform: Transform
}

impl GameCamera {
  pub fn new(position: Vec3) -> Self {
    Self {
      position_transform: Transform::from_translation(position)
        .looking_at(Vec3::ZERO, Vec3::Y)
    }
    
  }
  
  pub fn spawn(&self, commands: &mut Commands) {

    let perspective_projection = PerspectiveProjection {
      fov: 1.57, // 90 degrees in radians
      aspect_ratio: 16.0 / 9.0,         // Adjust to your window's aspect ratio
      near: 0.1,                        // Near clipping plane
      far: 1000.0,                      // Far clipping plane
      ..default()                       // Use defaults for any other fields
    };
    let projection = Projection::Perspective(perspective_projection);

    // Create the camera entity.
    commands.spawn((
      Camera3d::default(),
      projection,
      self.position_transform,
      self.clone(),
      Visibility::default()
    ));

    // Create a point light that follows the camera.
    commands.spawn((
      PointLight {
        shadows_enabled: true,
        intensity: 10000.0,
        ..default()
      },
      self.position_transform
    ));
  }
}
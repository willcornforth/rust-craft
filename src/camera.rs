use std::f32::consts::PI;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
pub fn mouse_look_system(
  mut mouse_motion_events: EventReader<MouseMotion>,
  mut camera_query: Query<(&mut Transform, &mut GameCamera)>) {
  let sensitivity = 0.002; // TODO: Make this changeable in-game.
  let mut delta = Vec2::ZERO; // Delta between motion events.

  // Accumulate motion delta.
  for event in mouse_motion_events.read() {
    delta += event.delta;
  }

  // We haven't moved much, so don't update the camera.
  if delta == Vec2::ZERO {
    return;
  }

  // Iterate each camera and update its transform.
  for(mut transform, mut camera) in camera_query.iter_mut() {
    // Update view angles with the calculated delta.
    camera.view_angles -= delta * sensitivity;
    
    // View angles to [-PI, PI] (-180, 180 degrees)
    if camera.view_angles.x > PI {
      camera.view_angles.x = -PI;
    } else if camera.view_angles.x < -PI {
      camera.view_angles.x = PI;
    }
    
    // Clamp the view angle pitch to [-90, 90].
    camera.view_angles.y = camera.view_angles.y.clamp(-(PI / 2.0), PI / 2.0);
    println!("{:?}", camera.view_angles);
    
    // Update the camera's transform to the wish direction.
    let direction = Vec3::new(
      -(camera.view_angles.x.cos() * camera.view_angles.y.cos()),
      camera.view_angles.y.sin(),
      camera.view_angles.x.sin() * camera.view_angles.y.cos()
    );
    
    // Update transform's translation to the new direction.
    let translation = transform.translation;
    transform.look_at(translation + direction, Vec3::Y);
  }
  
}

#[derive(Component, Clone)]
pub struct GameCamera {
  view_angles: Vec2
}

impl GameCamera {
  pub fn new() -> Self {
    Self {
      view_angles: Vec2::new(0.0, 0.0)
    }
    
  }
  pub fn spawn(&self, commands: &mut Commands, initial_position: Vec3) {
    let perspective_projection = PerspectiveProjection {
      fov: 1.57, // 90 degrees in radians
      aspect_ratio: 16.0 / 9.0,         // Adjust to your window's aspect ratio
      near: 0.1,                        // Near clipping plane
      far: 1000.0,                      // Far clipping plane
      ..default()                       // Use defaults for any other fields
    };
    let projection = Projection::Perspective(perspective_projection);

    let position_transform = Transform::from_translation(initial_position)
      .looking_at(Vec3::ZERO, Vec3::Y);
    
    // Create the camera entity.
    commands.spawn((
      Camera3d::default(),
      projection,
      position_transform,
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
      position_transform
    ));
  }
}
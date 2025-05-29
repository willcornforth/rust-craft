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
   for (mut transform, camera) in camera_query.iter_mut() {
     // Update view angles with the calculated delta.
     let (mut yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

     // Apply mouse input delta.
     yaw -= delta.x * sensitivity;
     pitch -= delta.y * sensitivity;

     // Normalise view angles to [-PI, PI] (-180, 180 degrees)
     // 1. Shifts radians by PI to remove working with negatives. (Moving the range from [-PI, PI] to [0, 2PI])
     // 2. Takes a Euclidean remainder when divided by 2PI effectively wrapping the angle to this range.
     // 3. Shifts the range back to [-PI, PI] by subtracting PI.
     yaw = (yaw + PI).rem_euclid(2.0 * PI) - PI;

     // Clamp the view angle pitch to [-90, 90].
     pitch = pitch.clamp(-(PI / 2.0), PI / 2.0);
     println!("View: {:?}", (yaw, pitch));

     transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
   }
 }

 // Camera marker tag.
 #[derive(Component, Clone)]
  pub struct GameCamera;

 impl GameCamera {
   pub fn new() -> Self {
     Self {}
   }
   pub fn spawn(&self, commands: &mut Commands, transform: Transform) {
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
       transform,
       self.clone(),
       Visibility::default()
     ));

     // Create a point light that follows the camera.
     commands.spawn((
       PointLight {
         shadows_enabled: true,
         intensity: 100000.0,
         ..default()
       },
       transform
     ));
   }
 }

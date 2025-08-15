use std::{f32::consts::FRAC_PI_2, ops::Range};

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>().add_systems(Update, (draw_cursor, orbit));
    }
}

#[derive(Debug, Resource)]
pub struct CameraSettings {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub roll_speed: f32,
    pub yaw_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
         let pitch_limit = FRAC_PI_2 - 0.1;
         Self {
            orbit_distance: 20.0,
            pitch_speed: 0.003,
            // top hemisphere correseponds from -pitch limit to 0, bottom is from 0 to pitch limit
            pitch_range: -pitch_limit..-0.01,
            roll_speed: 1.0,
            yaw_speed: 0.004,
        }
    }
}

#[derive(Component)]
pub struct Ground;

pub fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    //mouse_buttons: Res<ButtonInput<MouseButton>>,
    ground: Single<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let Ok(windows) = windows.single() else {
        return;
    };

    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        Isometry3d::new(
            point + ground.up() * 0.01,
            Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
        ),
        0.2,
        Color::WHITE,
    );
}

pub fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    let delta = mouse_motion.delta;
    let mut delta_roll = 0.0;

   
    if mouse_buttons.pressed(MouseButton::Right) {
        // Mouse motion is one of the few inputs that should not be multiplied by delta time,
        // as we are already receiving the full movement since the last frame was rendered. Multiplying
        // by delta time here would make the movement slower that it should be.
        let delta_pitch = delta.y * camera_settings.pitch_speed;
        let delta_yaw = delta.x * camera_settings.yaw_speed;

        // Conversely, we DO need to factor in delta time for mouse button inputs.
        delta_roll *= camera_settings.roll_speed * time.delta_secs();

        // Obtain the existing pitch, yaw, and roll values from the transform.
        let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

        // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
        let pitch = (pitch + delta_pitch).clamp(
            camera_settings.pitch_range.start,
            camera_settings.pitch_range.end,
        );
        let roll = roll + delta_roll;
        let yaw = yaw + delta_yaw;
        camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

        // Adjust the translation to maintain the correct orientation toward the orbit target.
        // In our example it's a static target, but this could easily be customized.
        let target = Vec3::ZERO;
        camera.translation = target - camera.forward() * camera_settings.orbit_distance;
    }
}
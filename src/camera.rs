use bevy::{
    prelude::*, 
    // pbr::DirectionalLightShadowMap,
    // pbr::AmbientLight, 
    core_pipeline::tonemapping::Tonemapping,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems( 
            Startup,
            spawn_camera);
        app.add_systems(
            Update, 
            (
                rotate_camera,
        ));
    }
}


fn spawn_camera(
    mut commands: Commands,
    // asset_server: Res<AssetServer>
) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.7, 1.8, 5.0).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
        tonemapping: Tonemapping::BlenderFilmic, // 2.
        ..default()
    };
    commands.spawn(camera);
    // rotate_camera(camera, time);
}


// Rotate Camera
fn rotate_camera(mut camera: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let cam_transform = camera.single_mut().into_inner();

    cam_transform.rotate_around(
        Vec3::ZERO,
        Quat::from_axis_angle(Vec3::Y, 16f32.to_radians() * time.delta_seconds()),
    );
    cam_transform.look_at(Vec3::new(0.0,0.6,0.0), Vec3::Y);
}


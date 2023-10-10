use bevy::prelude::*;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems( 
            Startup,
            spawn_floor);
        app.add_systems(
            Update, 
            (
                _debug,
        ));
    }
}


fn spawn_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let floor = SceneBundle {
        scene: asset_server.load("models/Floor.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    };
    commands.spawn(floor);
}


fn _debug (
    mut gizmos: Gizmos, 
    _time: Res<Time>
) {
    gizmos.cuboid(
        Transform::from_xyz(0.0, -0.05, 0.0).with_scale(Vec3::new(4.28, 0.108, 4.28)),
        Color::GREEN,
    );
}


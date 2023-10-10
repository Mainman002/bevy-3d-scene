use bevy::prelude::*;

pub struct TiefighterPlugin;

impl Plugin for TiefighterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems( 
            Startup,
            spawn_tiefighter);
        app.add_systems(
            Update, 
            (
                _debug,
        ));
    }
}


fn spawn_tiefighter(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let tiefighter = SceneBundle {
        scene: asset_server.load("models/TieFighter.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 1.2, 0.0),
        ..default()
    };
    commands.spawn(tiefighter);
}


fn _debug (
    mut gizmos: Gizmos, 
    _time: Res<Time>
) {
    gizmos.cuboid(
        Transform::from_xyz(0.0, 1.2, 0.0).with_scale(Vec3::new(2.08, 2.4, 2.0)),
        Color::GREEN,
    );
}


use bevy::prelude::*;
use std::default::Default;

pub struct TiefighterPlugin;

impl Plugin for TiefighterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (rotate_tiefighter, _debug));
    }
}

#[derive(Resource)]
struct Tiefighter(Entity);

// Load GLTF Model Into Scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tiefighter_entity = commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/TieFighter.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 1.2, 0.0),
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .id(); // Get the entity ID

    // Create the Tiefighter resource with the associated entity
    commands.insert_resource(Tiefighter(tiefighter_entity));
}

// Rotate Tiefighter Entity
fn rotate_tiefighter(
    mut query: Query<&mut Transform>,
    tiefighter: Res<Tiefighter>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_component_mut::<Transform>(tiefighter.0) {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::Y, -8f32.to_radians() * time.delta_seconds()),
        );
    }
}

fn _debug(mut gizmos: Gizmos, mut query: Query<&mut Transform>, tiefighter: Res<Tiefighter>) {
    if let Ok(transform) = query.get_component_mut::<Transform>(tiefighter.0) {
        gizmos.cuboid(
            transform.with_scale(Vec3::new(2.08, 2.4, 2.0)),
            Color::GREEN,
        );
    }
}

use bevy::prelude::*;
use std::default::Default;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, _debug);
    }
}

#[derive(Resource)]
struct Floor(Entity);

// Load GLTF Model Into Scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let floor_entity = commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/Floor.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .id(); // Get the entity ID

    // Create the floor resource with the associated entity
    commands.insert_resource(Floor(floor_entity));
}

// Draw Debug Bounds
fn _debug(mut gizmos: Gizmos, mut query: Query<&mut Transform>, floor: Res<Floor>) {
    if let Ok(transform) = query.get_component_mut::<Transform>(floor.0) {
        // Create a new transform with the Y-axis offset
        let offset = Transform::from_translation(Vec3::new(0.0, -0.054, 0.0)); // Adjust the Y value as needed

        // Apply the offset to the original transform
        let debug_transform = *transform * offset;

        // Create and display bounds
        gizmos.cuboid(
            debug_transform.with_scale(Vec3::new(4.28, 0.108, 4.28)),
            Color::GREEN,
        );
    }
}

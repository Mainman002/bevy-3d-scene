use bevy::prelude::*;

pub struct SunPlugin;

impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, _debug);
    }
}

#[derive(Resource)]
struct Sun(Entity);

// Load GLTF Model Into Scene
fn setup(mut commands: Commands) {
    let sun_entity = commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 50000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(-6.0, 4.0, 14.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .id(); // Get the entity ID

    // Create the sun resource with the associated entity
    commands.insert_resource(Sun(sun_entity));
}

// Draw Debug Bounds
fn _debug(mut gizmos: Gizmos, mut query: Query<&mut Transform>, sun: Res<Sun>) {
    if let Ok(_transform) = query.get_component_mut::<Transform>(sun.0) {
        // Create and display bounds
        gizmos.sphere(
            Vec3::new(-6.0, 4.0, 14.0),
            Quat::IDENTITY,
            0.5,
            Color::YELLOW,
        );
    }
}

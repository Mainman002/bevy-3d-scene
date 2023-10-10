use bevy::prelude::*;

pub struct SunPlugin;

impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems( 
            Startup,
            spawn_sun);
        app.add_systems(
            Update, 
            (
                _debug,
        ));
    }
}


fn spawn_sun(
    mut commands: Commands,
) {
    let light_sun = DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-6.0, 4.0, 14.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    };
    commands.spawn(light_sun);
}


fn _debug (
    mut gizmos: Gizmos, 
    _time: Res<Time>
) {
    gizmos.sphere(
        Vec3::new(-6.0, 4.0, 14.0), 
        Quat::IDENTITY, 
        0.5, 
        Color::YELLOW);
}


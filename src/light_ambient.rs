use bevy::prelude::*;

pub struct AmbientLightPlugin;

impl Plugin for AmbientLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems( 
            Startup,
            spawn_light_ambient);
    }
}


fn spawn_light_ambient(
    mut commands: Commands,
) {
    let light_ambient = AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 0.5,
        ..default()
    };
    commands.insert_resource(light_ambient);
}


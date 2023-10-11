use bevy::prelude::*;

pub struct DebugTextPlugin;

impl Plugin for DebugTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_debug_text,));
    }
}

fn spawn_debug_text(mut commands: Commands) {
    let debug_text = TextBundle::from_section(
        "A simple scene with a Tiefighter and PBR rendering\n\
            Gizmos are manually sized,\nrotation and transform match ECS\n\
            All materials default from GLTF\n\
            Tonemapping::BlenderFilmic\n\
            MSAA::Off\n\
            SSAO\n\
            Bloom\n\
            Skybox\n\
            EnvironmentMap\n\
            TAA\n\
            TAAJitter",
        TextStyle {
            font_size: 20.,
            ..default()
        },
    )
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(12.0),
        left: Val::Px(12.0),
        ..default()
    });
    commands.spawn(debug_text);
}

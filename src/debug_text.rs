use bevy::prelude::*;

pub struct DebugTextPlugin;

impl Plugin for DebugTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup, 
            (
                spawn_debug_text,
        ));
    }
}


fn spawn_debug_text(
    mut commands: Commands,
) {
    let debug_text =
        TextBundle::from_section(
            "A simple scene with a Tiefighter and PBR rendering\n\
            Gizmos are manually placed and sized currently\n\
            All materials were default in GLTF export / import",
            TextStyle {
                font_size: 20.,
                ..default()
            })
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        });
    commands.spawn(debug_text);
}


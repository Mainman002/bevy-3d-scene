use bevy::prelude::*;

pub struct DebugWorldPlugin;

impl Plugin for DebugWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (_debug,));
    }
}

// Update Debug Function
fn _debug(mut gizmos: Gizmos, _time: Res<Time>) {
    gizmos.ray(Vec3::ZERO, Vec3::new(0.0, 4.0, 0.0), Color::BLUE);

    gizmos.ray(Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0), Color::RED);

    gizmos.ray(Vec3::ZERO, Vec3::new(0.0, 0.0, 4.0), Color::GREEN);
}

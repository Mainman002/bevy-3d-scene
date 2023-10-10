use bevy::{
    prelude::*, 
    pbr::DirectionalLightShadowMap,
    pbr::AmbientLight, core_pipeline::tonemapping::Tonemapping,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;


fn main() {
    App::new()
        .insert_resource( ClearColor(Color::rgb(0.2,0.2,0.2)) )
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_plugins(DefaultPlugins
            .build()
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
        )
        .add_systems(
           Startup,
           (
                spawn_camera,
                spawn_light_ambient,
                spawn_light_sun,
                spawn_floor,
                spawn_tiefighter,
                spawn_debug_text,
            )
        )
        .add_systems(Update, 
            (
                system,
                rotate_camera,
            )
        )
    .run();
}


// #####################################################################
// Object Spawn Functions
// #####################################################################


// Camera
fn spawn_camera(
    mut commands: Commands,
    // asset_server: Res<AssetServer>
) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.7, 1.8, 5.0).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
        tonemapping: Tonemapping::BlenderFilmic, // 2.
        ..default()
    };
    commands.spawn(camera);
    // rotate_camera(camera, time);
}


// Ambient Light
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


// Sun Light
fn spawn_light_sun(
    mut commands: Commands,
) {
    let light_sun = DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-6.0, 20.0, 14.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    };
    commands.spawn(light_sun);
}


// Floor
fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    };
    commands.spawn(floor);
}


// TieFighter
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


// Debug Text
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


// #####################################################################
// Debug Functions
// #####################################################################


// Update Debug Function
fn system(mut gizmos: Gizmos, _time: Res<Time>) {
    // Tiefighter
    gizmos.cuboid(
        Transform::from_xyz(0.0, 1.2, 0.0).with_scale(Vec3::new(2.08, 2.4, 2.0)),
        Color::GREEN,
    );

    // Floor
    gizmos.cuboid(
        Transform::from_xyz(0.0, -0.05, 0.0).with_scale(Vec3::new(4.28, 0.108, 4.28)),
        Color::GREEN,
    );

    gizmos.ray(
        Vec3::ZERO,
        Vec3::new(0.0, 4.0, 0.0),
        Color::BLUE,
    );

    gizmos.ray(
        Vec3::ZERO,
        Vec3::new(4.0, 0.0, 0.0),
        Color::RED,
    );

    gizmos.ray(
        Vec3::ZERO,
        Vec3::new(0.0, 0.0, 4.0),
        Color::GREEN,
    );
}


// #####################################################################
// Update Functions
// #####################################################################


// Rotate Camera
fn rotate_camera(mut camera: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let cam_transform = camera.single_mut().into_inner();

    cam_transform.rotate_around(
        Vec3::ZERO,
        Quat::from_axis_angle(Vec3::Y, 16f32.to_radians() * time.delta_seconds()),
    );
    cam_transform.look_at(Vec3::new(0.0,0.6,0.0), Vec3::Y);
}


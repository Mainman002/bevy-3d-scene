//! Load a cubemap texture onto a cube like a skybox and cycle through different compressed texture formats

use bevy::{
    asset::LoadState,
    core_pipeline::{
        Skybox,
        bloom::{
            BloomCompositeMode, 
            BloomSettings, 
            // self, 
            // BloomPrefilterSettings
        },
        tonemapping::Tonemapping,
    },
    core_pipeline::experimental::{
        // taa::TemporalAntiAliasBundle, 
        taa::TemporalAntiAliasPlugin
    },
    pbr::{
        CascadeShadowConfigBuilder, DirectionalLightShadowMap,
        ScreenSpaceAmbientOcclusionBundle, 
        ScreenSpaceAmbientOcclusionQualityLevel,
        ScreenSpaceAmbientOcclusionSettings,
        // wireframe::{
        //     // Wireframe, 
        //     // WireframeConfig, 
        //     // WireframePlugin
        // }
    },
    prelude::*,
    render::{
        camera::TemporalJitter,
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        renderer::RenderDevice,
        texture::CompressedImageFormats,
        // render_resource::WgpuFeatures, 
        // settings::WgpuSettings,
        // RenderPlugin
    },
    // render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin},
};

use bevy_embedded_assets::EmbeddedAssetPlugin;
// use std::f32::consts::PI;
use std::{
    f32::consts::PI,
    // collections::hash_map::DefaultHasher,
    // hash::{Hash, Hasher},
};

const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[
    (
        "textures/Ryfjallet_cubemap.png",
        CompressedImageFormats::NONE,
    ),
    (
        "textures/Ryfjallet_cubemap_astc4x4.ktx2",
        CompressedImageFormats::ASTC_LDR,
    ),
    (
        "textures/Ryfjallet_cubemap_bc7.ktx2",
        CompressedImageFormats::BC,
    ),
    (
        "textures/Ryfjallet_cubemap_etc2.ktx2",
        CompressedImageFormats::ETC2,
    ),
];

fn main() {
    App::new()
        .insert_resource( ClearColor(Color::rgb(0.2,0.2,0.2)) )
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(
            DefaultPlugins
            .build()
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
            .add( TemporalAntiAliasPlugin)
            // .add(WireframePlugin)
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                cycle_cubemap_asset,
                asset_loaded.after(cycle_cubemap_asset),
                rotate_camera,
                system, 
                (update_bloom_settings, bounce_spheres),
                update_ssao_settings,
                // toggle_global_wireframe_setting,
                // camera_controller,
                // animate_light_direction,
            ),
        )
    .run();
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}

fn setup(
    mut commands: Commands, 
    // mut meshes: ResMut<Assets<Mesh>>, 
    // mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>, 
    ) {
    
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..default()
    });

    let skybox_handle = asset_server.load(CUBEMAPS[0].0);
    // camera
    // commands.spawn((
    //     Camera3dBundle {
    //         tonemapping: Tonemapping::TonyMcMapface,
    //         transform: Transform::from_xyz(-2.7, 1.8, 5.0).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
    //         ..default()
    //     },
    //     CameraController::default(),
    //     TemporalJitter::default(),
    //     // TemporalAntiAliasBundle::default(),
    //     Skybox(skybox_handle.clone()),
    //     ScreenSpaceAmbientOcclusionBundle::default(),
    //     // ScreenSpaceAmbientOcclusionSettings {
    //     //     quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Low
    //     // },
    //     BloomSettings {
    //         intensity: 0.25, // the default is 0.3
    //         ..default()
    //     },
    //     EnvironmentMapLight { 
    //         diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"), 
    //         specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
    //     },
    // ));
    // .insert( EnvironmentMapLight { diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"), specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2")}, )
    // .insert(ScreenSpaceAmbientOcclusionBundle::default())
    // .insert(ScreenSpaceAmbientOcclusionSettings {quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Low});
    // .insert(TemporalJitter::default())
    // .insert(TemporalAntiAliasBundle::default());

    // Camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::BlenderFilmic, // 2. Using a tonemapper that desaturates to white is recommended
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
        TemporalJitter::default(),
        // TemporalAntiAliasBundle::default(),
        Skybox(skybox_handle.clone()),
        ScreenSpaceAmbientOcclusionBundle::default(),
        BloomSettings {
            intensity: 0.38,
            low_frequency_boost: 0.26,
            low_frequency_boost_curvature: 0.75,
            high_pass_frequency: 1.0,
            composite_mode: BloomCompositeMode::EnergyConserving,
            ..default()
        },
        EnvironmentMapLight { 
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"), 
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        },
    ));


    // Skybox
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("models/Skybox.gltf#Scene0"),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });

    // ChromeSphere
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("models/ChromeSphere.gltf#Scene0"),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });

    // LightSphere
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("models/LightSphere.gltf#Scene0"),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });

    // let material_emissive1 = materials.add(StandardMaterial {
    //     emissive: Color::rgb_linear(13.99, 5.32, 2.0), // 4. Put something bright in a dark environment to see the effect
    //     ..default()
    // });
    // let material_emissive2 = materials.add(StandardMaterial {
    //     emissive: Color::rgb_linear(2.0, 13.99, 5.32),
    //     ..default()
    // });
    // let material_emissive3 = materials.add(StandardMaterial {
    //     emissive: Color::rgb_linear(5.32, 2.0, 13.99),
    //     ..default()
    // });
    // let material_non_emissive = materials.add(StandardMaterial {
    //     base_color: Color::GRAY,
    //     ..default()
    // });

    // let material = material_emissive1.clone();

    // let mesh = meshes.add(
    //     shape::Icosphere {
    //         radius: 0.5,
    //         subdivisions: 5,
    //     }
    //     .try_into()
    //     .unwrap(),
    // );

    // commands.spawn((
    //     PbrBundle {
    //         mesh: mesh.clone(),
    //         material,
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..default()
    //     },
    // ));

    // Floor
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/Floor.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // let material_emissive1 = materials.add(StandardMaterial {
    //     emissive: Color::rgb_linear(13.99, 5.32, 2.0), // 4. Put something bright in a dark environment to see the effect
    //     ..default()
    // });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: material_emissive1,
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });

    // TieFighter
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/TieFighter.gltf#Scene0"),
        // material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 1.2, 0.0),
        // add_material( material_emissive1.clone() ),
        ..default()
    });

    // fn add_material(
    //     mut materials: ResMut<Assets<StandardMaterial>>,
    // ) {
    //     let new_mat = StandardMaterial {
    //         base_color: Color::rgba(0.25, 0.50, 0.75, 1.0),
    //         unlit: true,
    //         ..Default::default()
    //     };
    
    //     let handle = materials.add(new_mat);
    
    //     // do something with the handle
    // }

    // light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-2.0, 4.0, 3.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate color and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 0.5,
    });

    // Cubemap
    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });

    // Text
    commands.spawn(
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
        }),
    );
    
    // for x in -5..5 {
    //     for z in -5..5 {
    //         let mut hasher = DefaultHasher::new();
    //         (x, z).hash(&mut hasher);
    //         let rand = (hasher.finish() - 2) % 6;

    //         let material = match rand {
    //             0 => material_emissive1.clone(),
    //             1 => material_emissive2.clone(),
    //             2 => material_emissive3.clone(),
    //             3..=5 => material_non_emissive.clone(),
    //             _ => unreachable!(),
    //         };

    //         commands.spawn((
    //             PbrBundle {
    //                 mesh: mesh.clone(),
    //                 material,
    //                 transform: Transform::from_xyz(x as f32 * 2.0, 0.0, z as f32 * 2.0),
    //                 ..default()
    //             },
    //             Bouncing,
    //         ));
    //     }
    // }

}

const CUBEMAP_SWAP_DELAY: f32 = 3.0;

fn cycle_cubemap_asset(
    time: Res<Time>,
    mut next_swap: Local<f32>,
    mut cubemap: ResMut<Cubemap>,
    asset_server: Res<AssetServer>,
    render_device: Res<RenderDevice>,
) {
    let now = time.elapsed_seconds();
    if *next_swap == 0.0 {
        *next_swap = now + CUBEMAP_SWAP_DELAY;
        return;
    } else if now < *next_swap {
        return;
    }
    *next_swap = 1.0;

    let supported_compressed_formats =
        CompressedImageFormats::from_features(render_device.features());

    let mut new_index = cubemap.index;
    for _ in 0..CUBEMAPS.len() {
        new_index = (new_index + 1) % CUBEMAPS.len();
        if supported_compressed_formats.contains(CUBEMAPS[new_index].1) {
            break;
        }
        info!("Skipping unsupported format: {:?}", CUBEMAPS[new_index]);
    }

    // Skip swapping to the same texture. Useful for when ktx2, zstd, or compressed texture support
    // is missing
    if new_index == cubemap.index {
        return;
    }

    cubemap.index = new_index;
    cubemap.image_handle = asset_server.load(CUBEMAPS[cubemap.index].0);
    cubemap.is_loaded = false;
}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.get_load_state(&cubemap.image_handle) == LoadState::Loaded {
        // info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(
                image.texture_descriptor.size.height / image.texture_descriptor.size.width,
            );
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}

// fn animate_light_direction(
//     time: Res<Time>,
//     mut query: Query<&mut Transform, With<DirectionalLight>>,
// ) {
//     for mut transform in &mut query {
//         transform.rotate_y(time.delta_seconds() * 0.5);
//     }
// }

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_key_enable_mouse: MouseButton,
    pub keyboard_key_enable_mouse: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 0.5,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::ShiftLeft,
            mouse_key_enable_mouse: MouseButton::Left,
            keyboard_key_enable_mouse: KeyCode::M,
            walk_speed: 2.0,
            run_speed: 6.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

fn rotate_camera(mut camera: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let cam_transform = camera.single_mut().into_inner();

    cam_transform.rotate_around(
        Vec3::ZERO,
        Quat::from_axis_angle(Vec3::Y, 16f32.to_radians() * time.delta_seconds()),
    );
    cam_transform.look_at(Vec3::new(0.0,0.6,0.0), Vec3::Y);
}

// pub fn camera_controller(
//     time: Res<Time>,
//     // mut mouse_events: EventReader<MouseMotion>,
//     // mouse_button_input: Res<Input<MouseButton>>,
//     key_input: Res<Input<KeyCode>>,
//     mut move_toggled: Local<bool>,
//     mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
// ) {
//     let dt = time.delta_seconds();

//     if let Ok((mut transform, mut options)) = query.get_single_mut() {
//         if !options.initialized {
//             let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
//             options.yaw = yaw;
//             options.pitch = pitch;
//             options.initialized = true;
//         }
//         if !options.enabled {
//             return;
//         }

//         // Handle key input
//         let mut axis_input = Vec3::ZERO;
//         if key_input.pressed(options.key_forward) {
//             axis_input.z += 1.0;
//         }
//         if key_input.pressed(options.key_back) {
//             axis_input.z -= 1.0;
//         }
//         if key_input.pressed(options.key_right) {
//             axis_input.x += 1.0;
//         }
//         if key_input.pressed(options.key_left) {
//             axis_input.x -= 1.0;
//         }
//         if key_input.pressed(options.key_up) {
//             axis_input.y += 1.0;
//         }
//         if key_input.pressed(options.key_down) {
//             axis_input.y -= 1.0;
//         }
//         if key_input.just_pressed(options.keyboard_key_enable_mouse) {
//             *move_toggled = !*move_toggled;
//         }

//         // Apply movement update
//         if axis_input != Vec3::ZERO {
//             let max_speed = if key_input.pressed(options.key_run) {
//                 options.run_speed
//             } else {
//                 options.walk_speed
//             };
//             options.velocity = axis_input.normalize() * max_speed;
//         } else {
//             let friction = options.friction.clamp(0.0, 1.0);
//             options.velocity *= 1.0 - friction;
//             if options.velocity.length_squared() < 1e-6 {
//                 options.velocity = Vec3::ZERO;
//             }
//         }
//         let forward = transform.forward();
//         let right = transform.right();
//         transform.translation += options.velocity.x * dt * right
//             + options.velocity.y * dt * Vec3::Y
//             + options.velocity.z * dt * forward;

//         // Handle mouse input
//         // let mut mouse_delta = Vec2::ZERO;
//         // if mouse_button_input.pressed(options.mouse_key_enable_mouse) || *move_toggled {
//         //     for mouse_event in mouse_events.read() {
//         //         mouse_delta += mouse_event.delta;
//         //     }
//         // }

//         // if mouse_delta != Vec2::ZERO {
//         //     // Apply look update
//         //     options.pitch = (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt)
//         //         .clamp(-PI / 2., PI / 2.);
//         //     options.yaw -= mouse_delta.x * options.sensitivity * dt;
//         //     transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, options.yaw, options.pitch);
//         // }
//     }
// }

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

/// Periodically turns the global wireframe setting on and off, to show the differences between
/// [`Wireframe::AlwaysRender`], [`Wireframe::NeverRender`], and no override.
// fn toggle_global_wireframe_setting(
//     mut wireframe_config: ResMut<WireframeConfig>,
// ) {
//     // The global wireframe config enables drawing of wireframes on every mesh, except those with
//     // `WireframeOverride::NeverRender`. Meshes with `WireframeOverride::AlwaysRender` will
//     // always have a wireframe, regardless of the global configuration.
//     wireframe_config.global = !wireframe_config.global;
// }


fn update_bloom_settings(
    mut camera: Query<(
        Entity, 
        Option<&mut BloomSettings>,
    ), 
        With<Camera>>,

    mut text: Query<&mut Text>,
    mut commands: Commands,
    keycode: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // let bloom_settings = camera.single_mut();
    // let ssao_settings = camera.single_mut();

    // let (
    //     camera_entity, 
    //     bloom_settings,
    //     ssao_settings, 
    //     temporal_jitter) = camera.single();
    let bloom_settings = camera.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    match bloom_settings {
        (entity, Some(mut bloom_settings)) => {
            *text = "BloomSettings (Toggle: Space)\n".to_string();
            text.push_str(&format!("(Q/A) Intensity: {}\n", bloom_settings.intensity));
            text.push_str(&format!(
                "(W/S) Low-frequency boost: {}\n",
                bloom_settings.low_frequency_boost
            ));
            text.push_str(&format!(
                "(E/D) Low-frequency boost curvature: {}\n",
                bloom_settings.low_frequency_boost_curvature
            ));
            text.push_str(&format!(
                "(R/F) High-pass frequency: {}\n",
                bloom_settings.high_pass_frequency
            ));
            text.push_str(&format!(
                "(T/G) Mode: {}\n",
                match bloom_settings.composite_mode {
                    BloomCompositeMode::EnergyConserving => "Energy-conserving",
                    BloomCompositeMode::Additive => "Additive",
                }
            ));
            text.push_str(&format!(
                "(Y/H) Threshold: {}\n",
                bloom_settings.prefilter_settings.threshold
            ));
            text.push_str(&format!(
                "(U/J) Threshold softness: {}\n",
                bloom_settings.prefilter_settings.threshold_softness
            ));

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).remove::<BloomSettings>();
                // wireframe_config.global = !wireframe_config.global;
            }

            let dt = time.delta_seconds();

            if keycode.pressed(KeyCode::A) {
                bloom_settings.intensity -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::Q) {
                bloom_settings.intensity += dt / 10.0;
            }
            bloom_settings.intensity = bloom_settings.intensity.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::S) {
                bloom_settings.low_frequency_boost -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::W) {
                bloom_settings.low_frequency_boost += dt / 10.0;
            }
            bloom_settings.low_frequency_boost = bloom_settings.low_frequency_boost.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::D) {
                bloom_settings.low_frequency_boost_curvature -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::E) {
                bloom_settings.low_frequency_boost_curvature += dt / 10.0;
            }
            bloom_settings.low_frequency_boost_curvature =
                bloom_settings.low_frequency_boost_curvature.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::F) {
                bloom_settings.high_pass_frequency -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::R) {
                bloom_settings.high_pass_frequency += dt / 10.0;
            }
            bloom_settings.high_pass_frequency = bloom_settings.high_pass_frequency.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::G) {
                bloom_settings.composite_mode = BloomCompositeMode::Additive;
            }
            if keycode.pressed(KeyCode::T) {
                bloom_settings.composite_mode = BloomCompositeMode::EnergyConserving;
            }

            bloom_settings.prefilter_settings.threshold = 4.0;

            // if keycode.pressed(KeyCode::H) {
            //     bloom_settings.prefilter_settings.threshold -= dt;
            // }
            // if keycode.pressed(KeyCode::Y) {
            //     bloom_settings.prefilter_settings.threshold += dt;
            // }
            // bloom_settings.prefilter_settings.threshold = 0.5;
            // bloom_settings.prefilter_settings.threshold =
            //     bloom_settings.prefilter_settings.threshold.max(0.0);

            if keycode.pressed(KeyCode::J) {
                bloom_settings.prefilter_settings.threshold_softness -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::U) {
                bloom_settings.prefilter_settings.threshold_softness += dt / 10.0;
            }
            bloom_settings.prefilter_settings.threshold_softness = bloom_settings
                .prefilter_settings
                .threshold_softness
                .clamp(0.0, 1.0);
        }

        (entity, None) => {
            *text = "Bloom: Off (Toggle: Space)".to_string();

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).insert(BloomSettings::default());

                // if temporal_jitter.is_some() {
                //     commands.remove::<TemporalJitter>();
                // } else {
                //     commands.insert(TemporalJitter::default());
                // }
            }
        }
    }
}

#[derive(Component)]
struct Bouncing;

fn bounce_spheres(time: Res<Time>, mut query: Query<&mut Transform, With<Bouncing>>) {
    for mut transform in query.iter_mut() {
        transform.translation.y =
            (transform.translation.x + transform.translation.z + time.elapsed_seconds()).sin();
    }
}


fn update_ssao_settings(
    camera: Query<
        (
            Entity,
            Option<&ScreenSpaceAmbientOcclusionSettings>,
            Option<&TemporalJitter>,
        ),
        With<Camera>,
    >,
    mut text: Query<&mut Text>,
    // mut sphere: Query<&mut Transform, With<SphereMarker>>,
    mut commands: Commands,
    keycode: Res<Input<KeyCode>>,
    // time: Res<Time>,
) {
    // let mut sphere = sphere.single_mut();
    // sphere.translation.y = (time.elapsed_seconds() / 1.7).sin() * 0.7;

    let (camera_entity, ssao_settings, temporal_jitter) = camera.single();

    let mut commands = commands.entity(camera_entity);
    if keycode.just_pressed(KeyCode::Key1) {
        commands.remove::<ScreenSpaceAmbientOcclusionSettings>();
    }
    if keycode.just_pressed(KeyCode::Key2) {
        commands.insert(ScreenSpaceAmbientOcclusionSettings {
            quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Low,
        });
    }
    if keycode.just_pressed(KeyCode::Key3) {
        commands.insert(ScreenSpaceAmbientOcclusionSettings {
            quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Medium,
        });
    }
    if keycode.just_pressed(KeyCode::Key4) {
        commands.insert(ScreenSpaceAmbientOcclusionSettings {
            quality_level: ScreenSpaceAmbientOcclusionQualityLevel::High,
        });
    }
    if keycode.just_pressed(KeyCode::Key5) {
        commands.insert(ScreenSpaceAmbientOcclusionSettings {
            quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
        });
    }
    if keycode.just_pressed(KeyCode::Space) {
        if temporal_jitter.is_some() {
            commands.remove::<TemporalJitter>();
        } else {
            commands.insert(TemporalJitter::default());
        }
    }

    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;
    text.clear();

    let (o, l, m, h, u) = match ssao_settings.map(|s| s.quality_level) {
        None => ("*", "", "", "", ""),
        Some(ScreenSpaceAmbientOcclusionQualityLevel::Low) => ("", "*", "", "", ""),
        Some(ScreenSpaceAmbientOcclusionQualityLevel::Medium) => ("", "", "*", "", ""),
        Some(ScreenSpaceAmbientOcclusionQualityLevel::High) => ("", "", "", "*", ""),
        Some(ScreenSpaceAmbientOcclusionQualityLevel::Ultra) => ("", "", "", "", "*"),
        _ => unreachable!(),
    };

    text.push_str("SSAO Quality:\n");
    text.push_str(&format!("(1) {o}Off{o}\n"));
    text.push_str(&format!("(2) {l}Low{l}\n"));
    text.push_str(&format!("(3) {m}Medium{m}\n"));
    text.push_str(&format!("(4) {h}High{h}\n"));
    text.push_str(&format!("(5) {u}Ultra{u}\n\n"));

    text.push_str("Temporal Antialiasing:\n");
    text.push_str(match temporal_jitter {
        Some(_) => "(Space) Enabled",
        None => "(Space) Disabled",
    });
}


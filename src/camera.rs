use bevy::{
    asset::LoadState,
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        experimental::taa::TemporalAntiAliasBundle,
        tonemapping::Tonemapping,
        Skybox,
    },
    pbr::ScreenSpaceAmbientOcclusionBundle,
    prelude::*,
    render::{
        camera::TemporalJitter,
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        renderer::RenderDevice,
        texture::CompressedImageFormats,
    },
};
use std::default::Default;

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

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, load_cubemap, environment_map_load_finish));
        app.add_systems(
            Update,
            (
                rotate_camera,
                load_bloom_settings,
                cycle_cubemap_asset,
                asset_loaded.after(cycle_cubemap_asset),
            ),
        );
    }
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}

#[derive(Resource)]
struct Cam(Entity);

// Load Camera Into Scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox_handle = asset_server.load(CUBEMAPS[0].0);
    let camera_entity = commands
        .spawn(Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            transform: Transform::from_xyz(-2.7, 1.8, 5.0)
                .looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
            tonemapping: Tonemapping::BlenderFilmic, // 2.
            ..Default::default()
        })
        .insert(ScreenSpaceAmbientOcclusionBundle::default())
        .insert(BloomSettings::default())
        .insert(Skybox(skybox_handle.clone()))
        .insert(EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        })
        .insert(TemporalAntiAliasBundle::default())
        .insert(TemporalJitter::default())
        .insert(GlobalTransform::default())
        .id(); // Get the entity ID

    // Create skybox and render to screen
    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });

    // Create the camera resource with the associated entity
    commands.insert_resource(Cam(camera_entity));
}

// Rotate camera Entity
fn rotate_camera(mut query: Query<&mut Transform>, camera: Res<Cam>, time: Res<Time>) {
    if let Ok(mut transform) = query.get_component_mut::<Transform>(camera.0) {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::Y, 4f32.to_radians() * time.delta_seconds()),
        );
    }
}

fn load_bloom_settings(mut query: Query<&mut BloomSettings>, camera: Res<Cam>) {
    if let Ok(mut bloom_settings) = query.get_mut(camera.0) {
        // Apply clamping
        bloom_settings.intensity = bloom_settings.intensity.clamp(0.0, 1.0);
        bloom_settings.low_frequency_boost = bloom_settings.low_frequency_boost.clamp(0.0, 1.0);
        bloom_settings.low_frequency_boost_curvature =
            bloom_settings.low_frequency_boost_curvature.clamp(0.0, 1.0);
        bloom_settings.high_pass_frequency = bloom_settings.high_pass_frequency.clamp(0.0, 1.0);
        bloom_settings.prefilter_settings.threshold_softness = bloom_settings
            .prefilter_settings
            .threshold_softness
            .clamp(0.0, 1.0);

        // Set composite mode
        bloom_settings.composite_mode = BloomCompositeMode::EnergyConserving;
        // bloom_settings.composite_mode = BloomCompositeMode::Additive;

        // Set prefilter threshold
        bloom_settings.prefilter_settings.threshold = 4.0;
    }
}

fn load_cubemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox_handle = asset_server.load(CUBEMAPS[0].0);
    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle.clone(),
    });
    commands.spawn(Skybox(skybox_handle));
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
    if !cubemap.is_loaded && asset_server.get_load_state(&cubemap.image_handle) == LoadState::Loaded
    {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
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

fn environment_map_load_finish(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    environment_maps: Query<&EnvironmentMapLight>,
    label_query: Query<Entity, With<EnvironmentMapLabel>>,
) {
    if let Ok(environment_map) = environment_maps.get_single() {
        if asset_server.get_load_state(&environment_map.diffuse_map) == LoadState::Loaded
            && asset_server.get_load_state(&environment_map.specular_map) == LoadState::Loaded
        {
            if let Ok(label_entity) = label_query.get_single() {
                commands.entity(label_entity).despawn();
            }
        }
    }
}

#[derive(Component)]
struct EnvironmentMapLabel;

use bevy::{
    prelude::*, 
    pbr::DirectionalLightShadowMap,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;


// #####################################################################
// Module Plugins
// #####################################################################


mod debug_world;
use debug_world::DebugWorldPlugin;

mod debug_text;
use debug_text::DebugTextPlugin;

mod camera;
use camera::CameraPlugin;

mod light_sun;
use light_sun::SunPlugin;

mod light_ambient;
use light_ambient::AmbientLightPlugin;

mod floor;
use floor::FloorPlugin;

mod tiefighter;
use tiefighter::TiefighterPlugin;


// #####################################################################
// Main App Functions
// #####################################################################


fn main() {
    App::new()
        .insert_resource( ClearColor(Color::rgb(0.2,0.2,0.2)) )
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_plugins(DefaultPlugins
            .build()
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin))
        .add_plugins((
            DebugWorldPlugin,
            DebugTextPlugin,
            CameraPlugin,
            SunPlugin,
            AmbientLightPlugin,
            FloorPlugin,
            TiefighterPlugin))
    .run();
}


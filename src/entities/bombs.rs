use crate::{assets::AssetsState, components::*, controlls::PlaceBomb, screens::Screen};
use avian2d::parry::simba::scalar::SupersetOf;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<BombAssets>(),
    );
    app.add_systems(OnEnter(Screen::Gameplay), |a: &mut World| {
        a.add_observer(place_bomb);
    });
}

pub fn create_bomb(assets: &BombAssets, position: Vec2) -> impl Bundle {
    info!("create bomb at {}", position);
    (
        Name::new("Bomb"),
        Sprite {
            image: assets.ball.clone(),
            texture_atlas: Some(TextureAtlas {
                index: 0,
                layout: assets.ball_layout.clone(),
                ..default()
            }),
            ..default()
        },
        Transform::from_translation(position.extend(0.0).clone()),
        AnimationConfig::new(0, 8, 3),
        Animating,
    )
}

#[derive(AssetCollection, Resource)]
pub struct BombAssets {
    #[asset(path = "images/vfx/Charge_Fire.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub charge: Handle<Image>,
    #[asset(path = "images/vfx/Fire_Explosion.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub explosion: Handle<Image>,
    #[asset(path = "images/vfx/Lavaball.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub ball: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 6, rows = 4))]
    pub ball_layout: Handle<TextureAtlasLayout>,
}

fn place_bomb(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    screen_state: Res<State<Screen>>,
    assets_state: Res<State<AssetsState>>,
    assets: Res<BombAssets>,
) {
    if screen_state.get() == &Screen::Gameplay
    // && assets_state.get() == &AssetsState::GameplayReady
    {
        // info!("clicky {}", trigger.pointer_location.position);
        let location = trigger.pointer_location.position;
        commands.spawn(create_bomb(&assets, location));
    }
}

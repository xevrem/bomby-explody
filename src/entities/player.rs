use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    assets::AssetsState, components::*, constants::SCREEN_WIDTH, menus::Menu, screens::Screen,
    AppSystems, GameplaySystems, PausableSystems, Pause,
};

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<CharacterAssets>(),
    );
    app.add_systems(
        Update,
        (check_if_player_dead)
            .in_set(AppSystems::Update)
            .in_set(GameplaySystems)
            .in_set(PausableSystems),
    );
}

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(path = "images/character.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub character: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 48, tile_size_y = 48, columns = 4, rows = 4))]
    pub character_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/character_idle.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub character_idle: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 48, tile_size_y = 48, columns = 4, rows = 4))]
    pub character_idle_layout: Handle<TextureAtlasLayout>,
}

pub fn create_player_character(assets: &CharacterAssets, health: i32) -> impl Bundle {
    let start_index = 8;
    let start_pos = Vec3::new(-SCREEN_WIDTH / 2.0 + 24.0, 0.0, 0.0);
    (
        Name::new("Character"),
        Animating,
        AnimationConfig::new(start_index, 4, 4),
        Character,
        Damageable,
        Health { current: health },
        Player,
        Sprite {
            image: assets.character_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                index: start_index,
                layout: assets.character_idle_layout.clone(),
            }),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        StateScoped(Screen::Gameplay),
        Transform::from_translation(start_pos),
    )
}

fn check_if_player_dead(
    player: Option<Single<Entity, (With<Player>, With<Dead>)>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_pause: ResMut<NextState<Pause>>
) {
    if player.is_some() {
        // player is dead,
        next_menu.set(Menu::GameOver);
        next_pause.set(Pause(true));
    }
}

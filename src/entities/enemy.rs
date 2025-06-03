use crate::{assets::AssetsState, components::*};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<EnemyAssets>(),
    );
    app.add_systems(Update, animate_enemy);
}

pub fn create_enemy(enemy_assets: &EnemyAssets, index: usize, position: Vec2) -> impl Bundle {
    (
        Name::new("Enemy"),
        Enemy,
        Sprite::from_atlas_image(
            enemy_assets.enemies.clone(),
            TextureAtlas {
                layout: enemy_assets.layout.clone(),
                index,
                ..default()
            },
        ),
        AnimationConfig::new(index, 4, 4),
        Transform {
            translation: position.extend(0.0),
            ..default()
        },
    )
}

#[derive(AssetCollection, Resource)]
pub struct EnemyAssets {
    #[asset(path = "images/enemies.png")]
    #[asset(image(sampler(filter = nearest)))]
    enemies: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 30, tile_size_y = 30, columns = 4, rows = 48))]
    layout: Handle<TextureAtlasLayout>,
}

fn animate_enemy(
    mut enemy_query: Query<(&mut Sprite, &mut AnimationConfig), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut sprite, mut anim) in &mut enemy_query {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == anim.index + anim.frames {
                    // last frame, so reset
                    atlas.index = anim.index;
                } else {
                    // continue iterating
                    atlas.index += 1;
                }
            }

            anim.timer = anim.timer_from_self_fps();
        }
    }
}

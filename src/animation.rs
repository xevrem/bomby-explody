use crate::{assets::AssetsState, components::*, AppSystems, GameplaySystems, PausableSystems};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        animate_entity
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

fn animate_entity(
    mut enemy_query: Query<(&mut Sprite, &mut AnimationConfig), With<Animating>>,
    time: Res<Time>,
) {
    for (mut sprite, mut anim) in &mut enemy_query {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == anim.index + anim.frames - 1 {
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

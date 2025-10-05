use crate::{
    assets::AssetsState, components::*, events::DamageEvent, AppSystems, GameplaySystems,
    PausableSystems,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<BulletAssets>(),
    )
    .add_systems(
        Update,
        (check_bullet_hit_player)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

#[derive(AssetCollection, Resource)]
pub struct BulletAssets {
    #[asset(path = "images/vfx/bullet.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub bullet: Handle<Image>,
}

pub fn create_bullet(
    bullet_assets: &BulletAssets,
    target_position: Vec2,
    start_position: Vec2,
    speed: f32,
) -> impl Bundle {
    let movement = (target_position - start_position).normalize();
    let angle = Vec2::Y.angle_to(movement);
    let transform = Transform::from_translation(start_position.extend(0.0))
        .with_rotation(Quat::from_rotation_z(angle));
    (
        Name::new("Bullet"),
        Bullet,
        MovementConfig::from_vec2(movement).with_speed(speed),
        Moving,
        Sprite {
            image: bullet_assets.bullet.clone(),
            custom_size: Some(Vec2::splat(15.0 * 3.0)),
            ..default()
        },
        transform,
    )
}

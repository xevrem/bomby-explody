use crate::{components::*, events::*, AppSystems, GameplaySystems, PausableSystems};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_damage_to_hp
            .in_set(AppSystems::Events)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

fn apply_damage_to_hp(
    mut commands: Commands,
    mut damage_events: EventReader<DamageEvent>,
    mut query: Query<(Entity, &mut Health), With<Damageable>>,
) -> Result {
    if !damage_events.is_empty() {
        for damage in damage_events.read() {
            let (entity, mut health) = query.get_mut(damage.target)?;
            health.current -= damage.amount;
            // for damage effects later
            commands.entity(entity).insert(Damaged);

            if health.current <= 0 {
                // ded
                commands.entity(entity).insert(Dead {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                });
            }
        }
    }
    Ok(())
}

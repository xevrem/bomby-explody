use crate::{components::*, events::*, AppSystems, GameplaySystems, PausableSystems};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_blast_damage
            .in_set(AppSystems::Events)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

fn apply_blast_damage(
    mut blast_reader: EventReader<BlastEvent>,
    mut damage_writer: EventWriter<DamageEvent>,
    enemy_query: Query<(Entity, &GlobalTransform), (With<Damageable>, Without<Dead>)>,
) -> Result {
    if !blast_reader.is_empty() {
        for blast_event in blast_reader.read() {
            // let blast_trans = blast_query.get(blast_event.source)?;
            for (enemy, enemy_trans) in &enemy_query {
                if enemy_trans
                    .translation()
                    .xy()
                    .distance(blast_event.location)
                    <= 100.0
                {
                    // blasted
                    damage_writer.write(DamageEvent {
                        target: enemy,
                        amount: 1,
                    });
                }
            }
        }
    }

    Ok(())
}

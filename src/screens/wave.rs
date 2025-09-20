//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{
    components::{Countdown, Wave},
    screens::Screen,
    theme::prelude::*,
    waves::WaveState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(WaveState::Announce), spawn_wave_screen)
        .add_systems(
            Update,
            wave_screen_fade.run_if(in_state(WaveState::Announce).and(in_state(Screen::Gameplay))),
        );
}

#[derive(Component)]
struct WaveScreen;

fn spawn_wave_screen(mut commands: Commands, wave_query: Single<&Wave>) {
    let level = wave_query.level;
    commands.spawn((
        WaveScreen,
        widget::ui_root("Wave Screen"),
        StateScoped(Screen::Gameplay),
        StateScoped(WaveState::Announce),
        Countdown {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
        },
        children![widget::header(format!("Wave {level}"))],
    ));
}

fn wave_screen_fade(
    mut query: Single<(&mut Countdown, &Children), With<WaveScreen>>,
    mut child_query: Query<&mut TextColor, With<Text>>,
    mut next_state: ResMut<NextState<WaveState>>,
    time: Res<Time>,
) {
    if query.0.timer.just_finished() {
        next_state.set(WaveState::Init);
    } else {
        // for mut text_color in child_query.iter_mut() {
        for child in query.1.iter() {
            if let Ok(mut text_color) = child_query.get_mut(child) {
                let mut new_color = text_color.to_srgba();
                let fraction = query.0.timer.fraction();
                if fraction < 0.5 {
                    new_color.alpha = fraction / 0.5;
                } else {
                    new_color.alpha = (1.0 - fraction) / 0.5;
                }
                text_color.0 = Color::Srgba(new_color);
            }
        }
        query.0.timer.tick(time.delta());
    }
}

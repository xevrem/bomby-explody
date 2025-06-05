use bevy::{audio::Volume, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::{assets::AssetsState, components::*};

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<SfxAssets>(),
    );

    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );
}

#[derive(Resource, AssetCollection)]
pub struct SfxAssets {
    #[asset(
        paths(
            "audio/sound_effects/bomb_1.ogg",
            "audio/sound_effects/bomb_2.ogg",
            "audio/sound_effects/bomb_3.ogg",
            "audio/sound_effects/bomb_4.ogg",
        ),
        collection(typed)
    )]
    pub bombs: Vec<Handle<AudioSource>>,
}

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
}

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>, volume: f32) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume)), SoundEffect)
}

/// [`GlobalVolume`] doesn't apply to already-running audio entities, so this system will update them.
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}

use std::time::{SystemTime, UNIX_EPOCH};

use librespot_metadata::audio::UniqueFields;
use librespot_playback::player::{PlayerEvent, PlayerEventChannel};
use serde::Serialize;
use tauri::{AppHandle, Emitter};

/// What the frontend receives in the `track` event.
/// `position_ms` was true at `timestamp_ms` (unix time); extrapolate from there while playing.
#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackPayload {
    title: String,
    artist: String,
    art_url: String,
    duration_ms: u32,
    position_ms: u32,
    playing: bool,
    timestamp_ms: u64,
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Listens to the player's events in the background and emits `track` to the frontend.
pub fn spawn(mut events: PlayerEventChannel, app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        // The loop owns the "current track" state; each event updates part of it.
        let mut current = TrackPayload::default();

        while let Some(event) = events.recv().await {
            match event {
                PlayerEvent::TrackChanged { audio_item } => {
                    current.title = audio_item.name.clone();
                    current.artist = match &audio_item.unique_fields {
                        UniqueFields::Track { artists, .. } => artists
                            .iter()
                            .map(|a| a.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", "),
                        UniqueFields::Local { artists, .. } => artists.clone().unwrap_or_default(),
                        UniqueFields::Episode { show_name, .. } => show_name.clone(),
                    };
                    // covers are sorted largest first
                    current.art_url = audio_item
                        .covers
                        .first()
                        .map(|c| c.url.clone())
                        .unwrap_or_default();
                    current.duration_ms = audio_item.duration_ms;
                    current.position_ms = 0;
                }
                PlayerEvent::Playing { position_ms, .. } => {
                    current.playing = true;
                    current.position_ms = position_ms;
                }
                PlayerEvent::Paused { position_ms, .. } => {
                    current.playing = false;
                    current.position_ms = position_ms;
                }
                PlayerEvent::Seeked { position_ms, .. } => {
                    current.position_ms = position_ms;
                }
                // Not part of the `track` payload; the frontend sets volume itself for now.
                _ => continue,
            }

            current.timestamp_ms = now_ms();
            if let Err(e) = app.emit("track", &current) {
                eprintln!("failed to emit track: {e}");
            }
        }
    });
}

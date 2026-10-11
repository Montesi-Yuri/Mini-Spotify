use std::sync::Mutex;

use crate::player_event;

use librespot_connect::{ConnectConfig, LoadRequest, LoadRequestOptions, Spirc};
use librespot_core::{authentication::Credentials, session::Session, Error};
use librespot_playback::{
    audio_backend,
    config::{AudioFormat, PlayerConfig},
    mixer::{self, MixerConfig},
    player::Player,
};

/// Tauri-managed state holding the Spirc handle once the device is up.
#[derive(Default)]
pub struct ConnectState(pub Mutex<Option<Spirc>>);

/// Starts the connect device (connects the session) and returns the Spirc handle.
/// The Spirc task is spawned in the background; commands are sent via the handle.
pub async fn start(
    session: Session,
    credentials: Credentials,
    app: tauri::AppHandle,
) -> Result<Spirc, Error> {
    let sink_builder = audio_backend::find(None).unwrap();
    let mixer_builder = mixer::find(None).unwrap();
    let audio_format = AudioFormat::default();

    let mixer = mixer_builder(MixerConfig::default())?;
    let player = Player::new(
        PlayerConfig::default(),
        session.clone(),
        mixer.get_soft_volume(),
        move || sink_builder(None, audio_format),
    );

    // Must be taken before `player` is moved into Spirc.
    player_event::spawn(player.get_player_event_channel(), app);

    let connect_config = ConnectConfig {
        name: "MiniWidget".to_string(),
        ..Default::default()
    };
    let (spirc, spirc_task) =
        Spirc::new(connect_config, session, credentials, player, mixer).await?;

    tauri::async_runtime::spawn(spirc_task);
    spirc.activate()?;
    Ok(spirc)
}

/// Runs `f` on the Spirc handle, mapping errors to strings for the frontend.
fn with_spirc(
    state: &ConnectState,
    f: impl FnOnce(&Spirc) -> Result<(), Error>,
) -> Result<(), String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    let spirc = guard.as_ref().ok_or("spotify not connected yet")?;
    f(spirc).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn play_pause(state: tauri::State<'_, ConnectState>) -> Result<(), String> {
    with_spirc(&state, |s| s.play_pause())
}

#[tauri::command]
pub fn next(state: tauri::State<'_, ConnectState>) -> Result<(), String> {
    with_spirc(&state, |s| s.next())
}

#[tauri::command]
pub fn prev(state: tauri::State<'_, ConnectState>) -> Result<(), String> {
    with_spirc(&state, |s| s.prev())
}

/// `position_ms`: absolute position in the current track.
#[tauri::command]
pub fn seek(state: tauri::State<'_, ConnectState>, position_ms: u32) -> Result<(), String> {
    with_spirc(&state, |s| s.set_position_ms(position_ms))
}

/// `volume`: 0..=65535 (u16::MAX is full volume).
#[tauri::command]
pub fn set_volume(state: tauri::State<'_, ConnectState>, volume: u16) -> Result<(), String> {
    with_spirc(&state, |s| s.set_volume(volume))
}

/// Frontend: `invoke("play_uri", { uri: "spotify:playlist:..." })`
#[tauri::command]
pub fn play_uri(state: tauri::State<'_, ConnectState>, uri: String) -> Result<(), String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    let spirc = guard.as_ref().ok_or("spotify not connected yet")?;
    spirc
        .load(LoadRequest::from_context_uri(uri, LoadRequestOptions::default()))
        .map_err(|e| e.to_string())?;
    spirc.play().map_err(|e| e.to_string())
}

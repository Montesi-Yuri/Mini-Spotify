// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod auth;
mod hotkeys;
mod layout;
mod play_connect;
mod player_event;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Position only: size is owned by the layout, visibility by the tray.
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(tauri_plugin_window_state::StateFlags::POSITION)
                .build(),
        )
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(play_connect::ConnectState::default())
        .invoke_handler(tauri::generate_handler![
            play_connect::play_uri,
            play_connect::play_pause,
            play_connect::next,
            play_connect::prev,
            play_connect::seek,
            play_connect::set_volume,
            layout::cycle_layout,
        ])
        .setup(|app| {
            use tauri::Manager;
            layout::init(app.handle());
            tray::init(app.handle())?;
            hotkeys::init(app.handle());
            let dir = app.path().app_data_dir().expect("no app data dir");
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let result = async {
                    let (session, credentials) = auth::login(dir).await?;
                    play_connect::start(session, credentials, handle.clone()).await
                }
                .await;
                match result {
                    Ok(spirc) => {
                        let state = handle.state::<play_connect::ConnectState>();
                        *state.0.lock().unwrap() = Some(spirc);
                        println!("spotify connect ready");
                    }
                    Err(e) => eprintln!("spotify setup failed: {e}"),
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application") 
}

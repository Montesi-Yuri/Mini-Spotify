// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod auth;
mod play_connect;
mod player_event;

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(play_connect::ConnectState::default())
        .invoke_handler(tauri::generate_handler![
            play_connect::play_uri,
            play_connect::play_pause,
            play_connect::next,
            play_connect::prev,
            play_connect::seek,
            play_connect::set_volume,
        ])
        .setup(|app| {
            use tauri::Manager;
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

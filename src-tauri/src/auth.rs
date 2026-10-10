use std::path::PathBuf;
use librespot_core::{
    authentication::Credentials,
    cache::Cache,
    config::SessionConfig,
    session::Session, Error,
};

/// Builds a session and credentials but does NOT connect: `Spirc::new` connects it.
pub async fn login(cache_dir: PathBuf) -> Result<(Session, Credentials), Error> {
    let session_config = SessionConfig::default();
    let cache = Cache::new(Some(cache_dir), None, None, None)?;

    let credentials = match cache.credentials() {
        Some(c) => c, // logged in before: no browser needed
        None => {
            let token = librespot_oauth::OAuthClientBuilder::new(
                &session_config.client_id,
                "http://127.0.0.1:8898/login",
                vec!["streaming"],
            )
            .open_in_browser()
            .build()?
            .get_access_token()?;
            Credentials::with_access_token(token.access_token)
        }
    };

    let session = Session::new(session_config, Some(cache));
    Ok((session, credentials))
}

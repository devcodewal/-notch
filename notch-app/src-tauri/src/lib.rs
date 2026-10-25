//! Notch backend: media session (SMTC), playback control, LRCLIB lyrics.
//! v0.1 scaffold. The SMTC part is Windows-only; on other platforms the
//! media commands return None and the frontend falls back to its demo mode.

mod media;

use serde::Serialize;
use tauri::Manager;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    /// total length in seconds
    pub duration_secs: f64,
    /// current playback position in seconds
    pub position_secs: f64,
    pub playing: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsResult {
    /// raw LRC text, e.g. "[00:12.34] some line"
    pub synced: Option<String>,
    pub plain: Option<String>,
}

#[tauri::command]
async fn get_media() -> Option<MediaInfo> {
    #[cfg(windows)]
    {
        media::get_media().await
    }
    #[cfg(not(windows))]
    {
        None
    }
}

#[tauri::command]
async fn media_toggle() -> bool {
    #[cfg(windows)]
    {
        media::toggle().await
    }
    #[cfg(not(windows))]
    {
        false
    }
}

#[tauri::command]
async fn media_next() -> bool {
    #[cfg(windows)]
    {
        media::skip_next().await
    }
    #[cfg(not(windows))]
    {
        false
    }
}

#[tauri::command]
async fn media_prev() -> bool {
    #[cfg(windows)]
    {
        media::skip_prev().await
    }
    #[cfg(not(windows))]
    {
        false
    }
}

#[tauri::command]
async fn get_lyrics(
    artist: String,
    title: String,
    album: String,
    duration_secs: f64,
) -> LyricsResult {
    let client = reqwest::Client::new();
    let url = "https://lrclib.net/api/get";
    let resp = client
        .get(url)
        .query(&[
            ("artist_name", artist.as_str()),
            ("track_name", title.as_str()),
            ("album_name", album.as_str()),
            ("duration", &format!("{}", duration_secs.round() as i64)),
        ])
        .send()
        .await;

    match resp {
        Ok(r) => {
            if let Ok(v) = r.json::<serde_json::Value>().await {
                return LyricsResult {
                    synced: v
                        .get("syncedLyrics")
                        .and_then(|x| x.as_str())
                        .map(|s| s.to_string()),
                    plain: v
                        .get("plainLyrics")
                        .and_then(|x| x.as_str())
                        .map(|s| s.to_string()),
                };
            }
            LyricsResult {
                synced: None,
                plain: None,
            }
        }
        Err(_) => LyricsResult {
            synced: None,
            plain: None,
        },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_media,
            media_toggle,
            media_next,
            media_prev,
            get_lyrics
        ])
        .setup(|app| {
            // Put the notch at the top-center of the primary monitor.
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(Some(monitor)) = window.primary_monitor() {
                    let size = monitor.size();
                    let x = (size.width as i32 - 340) / 2;
                    let _ = window.set_position(tauri::Position::Physical(
                        tauri::PhysicalPosition { x, y: 0 },
                    ));
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

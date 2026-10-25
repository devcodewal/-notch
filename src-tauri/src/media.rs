//! Windows System Media Transport Controls (SMTC) access.
//! Reads whatever is currently playing in any player that reports to the
//! Windows media overlay: Spotify, Apple Music, YouTube in a browser, etc.
//! Every failure returns None/false on purpose: no player, no session, or a
//! denied request just means "nothing playing right now", never a crash.

#[cfg(windows)]
use crate::MediaInfo;
#[cfg(windows)]
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};

#[cfg(windows)]
async fn current_session(
) -> Option<windows::Media::Control::GlobalSystemMediaTransportControlsSession> {
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .ok()?
        .await
        .ok()?;
    manager.GetCurrentSession().ok()
}

#[cfg(windows)]
fn hstring_to_string(h: windows::core::HSTRING) -> String {
    h.to_string()
}

/// Current track + position, or None when nothing is playing.
#[cfg(windows)]
pub async fn get_media() -> Option<MediaInfo> {
    let session = current_session().await?;
    let props = session.TryGetMediaPropertiesAsync().ok()?.await.ok()?;
    let timeline = session.GetTimelineProperties().ok()?;
    let playback = session.GetPlaybackInfo().ok()?;

    let status = playback.PlaybackStatus().ok()?;
    let playing = status == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing;

    // TimeSpan stores 100-nanosecond ticks.
    let position_secs = timeline.Position().ok()?.Duration as f64 / 10_000_000.0;
    let duration_secs = timeline.EndTime().ok()?.Duration as f64 / 10_000_000.0;

    Some(MediaInfo {
        title: hstring_to_string(props.Title().ok()?),
        artist: hstring_to_string(props.Artist().ok()?),
        album: hstring_to_string(props.AlbumTitle().ok()?),
        duration_secs,
        position_secs,
        playing,
    })
}

#[cfg(windows)]
pub async fn toggle() -> bool {
    if let Some(s) = current_session().await {
        if let Ok(op) = s.TryTogglePlayPauseAsync() {
            return op.await.is_ok();
        }
    }
    false
}

#[cfg(windows)]
pub async fn skip_next() -> bool {
    if let Some(s) = current_session().await {
        if let Ok(op) = s.TrySkipNextAsync() {
            return op.await.is_ok();
        }
    }
    false
}

#[cfg(windows)]
pub async fn skip_prev() -> bool {
    if let Some(s) = current_session().await {
        if let Ok(op) = s.TrySkipPreviousAsync() {
            return op.await.is_ok();
        }
    }
    false
}

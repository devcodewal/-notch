//! Windows System Media Transport Controls (SMTC) access.
//! Reads whatever is currently playing in any player that reports to the
//! Windows media overlay: Spotify, Apple Music, YouTube in a browser, etc.
//!
//! LESSON LEARNED (25 Okt 2026, proven by CI error E0277):
//! windows-rs's `IAsyncOperation` does NOT implement `Future` in this
//! setup, so `.await` on it does not compile. Instead of awaiting, we wait
//! for completion by polling `Status()` and then call `GetResults()`.
//! These are fast local COM calls (milliseconds), and the wait runs on a
//! blocking thread via `spawn_blocking` in lib.rs, so nothing stalls.

#[cfg(windows)]
use crate::MediaInfo;
#[cfg(windows)]
use windows::{
    core::Result as WinResult,
    Foundation::{AsyncStatus, IAsyncOperation},
    Media::Control::{
        GlobalSystemMediaTransportControlsSession,
        GlobalSystemMediaTransportControlsSessionManager,
        GlobalSystemMediaTransportControlsSessionPlaybackStatus,
    },
};

/// Block until a WinRT async operation finishes, then return its result.
#[cfg(windows)]
fn wait<T>(op: IAsyncOperation<T>) -> WinResult<T>
where
    T: windows::core::RuntimeType,
{
    loop {
        match op.Status()? {
            AsyncStatus::Completed => return op.GetResults(),
            AsyncStatus::Error | AsyncStatus::Canceled => {
                return Err(op.ErrorCode()?.into())
            }
            _ => std::thread::sleep(std::time::Duration::from_millis(5)),
        }
    }
}

#[cfg(windows)]
fn current_session() -> Option<GlobalSystemMediaTransportControlsSession> {
    let manager =
        wait(GlobalSystemMediaTransportControlsSessionManager::RequestAsync().ok()?).ok()?;
    manager.GetCurrentSession().ok()
}

/// Current track + position, or None when nothing is playing.
#[cfg(windows)]
pub fn get_media() -> Option<MediaInfo> {
    let session = current_session()?;
    let props = wait(session.TryGetMediaPropertiesAsync().ok()?).ok()?;
    let timeline = session.GetTimelineProperties().ok()?;
    let playback = session.GetPlaybackInfo().ok()?;

    let playing = playback.PlaybackStatus().ok()?
        == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing;

    // TimeSpan stores 100-nanosecond ticks.
    let position_secs = timeline.Position().ok()?.Duration as f64 / 10_000_000.0;
    let duration_secs = timeline.EndTime().ok()?.Duration as f64 / 10_000_000.0;

    Some(MediaInfo {
        title: props.Title().ok()?.to_string(),
        artist: props.Artist().ok()?.to_string(),
        album: props.AlbumTitle().ok()?.to_string(),
        duration_secs,
        position_secs,
        playing,
    })
}

#[cfg(windows)]
pub fn toggle() -> bool {
    let Some(session) = current_session() else {
        return false;
    };
    let Ok(op) = session.TryTogglePlayPauseAsync() else {
        return false;
    };
    wait(op).is_ok()
}

#[cfg(windows)]
pub fn skip_next() -> bool {
    let Some(session) = current_session() else {
        return false;
    };
    let Ok(op) = session.TrySkipNextAsync() else {
        return false;
    };
    wait(op).is_ok()
}

#[cfg(windows)]
pub fn skip_prev() -> bool {
    let Some(session) = current_session() else {
        return false;
    };
    let Ok(op) = session.TrySkipPreviousAsync() else {
        return false;
    };
    wait(op).is_ok()
}

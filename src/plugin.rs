mod linux;
mod macos;
mod save;
mod types;
mod windows;

use crate::prelude::*;
use std::sync::mpsc::channel;
use tauri::{command, Window};
use types::Options;

use self::save::save_to_disk;

#[command(async)]
pub fn snapshot(window: Window, options: Options) -> Result<Vec<u8>> {
    let (tx, rx) = channel::<Result<Vec<u8>>>();
    let Options {
        region,
        capture,
        save,
    } = options;

    window
        .with_webview(|webview| {
            #[cfg(target_os = "linux")]
            linux::snapshot(webview, region, capture, tx);

            #[cfg(target_os = "windows")]
            windows::snapshot(webview, region, capture, tx);

            #[cfg(target_os = "macos")]
            macos::snapshot(webview, region, capture, tx);
        })
        .map_err(|err| Error::WebView(err))?;

    let png_buffer = rx.recv().map_err(|err| Error::Threading(err))??;

    if let Some(save) = save {
        save_to_disk(save, &png_buffer)?;
    };

    Ok(png_buffer)
}

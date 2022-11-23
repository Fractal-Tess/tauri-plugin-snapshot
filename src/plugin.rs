mod linux;
mod macos;
mod windows;

use crate::prelude::*;
use cairo::ImageSurface;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::mpsc::channel};
use tauri::{command, Window};
use webkit2gtk::{SnapshotOptions, WebViewExt};

#[derive(Debug, Deserialize)]
struct Area {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Capture {
    transparent_background: Option<bool>,
    highlighted: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Options {
    area: Option<Area>,
    path: Option<PathBuf>,
    capture: Option<Capture>,
}

#[command]
pub async fn snapshot(window: Window, options: Options) -> Result<Vec<u8>> {
    println!("{:?}", options);
    let (tx, rx) = channel::<Result<Vec<u8>>>();
    window
        .with_webview(|webview| {
            #[cfg(target_os = "linux")]
            linux::snapshot(webview, tx);
            #[cfg(target_os = "windows")]
            windows::snapshot(webview, tx);
            #[cfg(target_os = "macos")]
            macos::snapshot(webview, tx);
        })
        .unwrap();
    let data = rx.recv().unwrap().unwrap();
    Ok(data)
}

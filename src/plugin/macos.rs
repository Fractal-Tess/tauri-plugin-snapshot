use std::sync::mpsc::Sender;

use cairo::ImageSurface;
use tauri::window::PlatformWebview;
use webkit2gtk::SnapshotOptions;

pub fn snapshot(webview: PlatformWebview, tx: Sender<Vec<u8>>) {
    tx.send(vec![]);
}

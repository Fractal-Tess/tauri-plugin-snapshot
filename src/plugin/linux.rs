use crate::prelude::*;
use cairo::ImageSurface;
use std::sync::mpsc::Sender;
use tauri::window::PlatformWebview;
use webkit2gtk::{traits::WebViewExt, SnapshotOptions};

pub fn snapshot(webview: PlatformWebview, tx: Sender<Result<Vec<u8>>>) {
    webview.inner().snapshot(
        webkit2gtk::SnapshotRegion::FullDocument,
        SnapshotOptions::TRANSPARENT_BACKGROUND,
        webkit2gtk::gio::Cancellable::NONE,
        move |surface| {
            let image_surface = ImageSurface::try_from(surface.unwrap()).unwrap();
            let mut buffer = Vec::new();
            image_surface.write_to_png(&mut buffer).unwrap();
            tx.send(Ok(buffer)).unwrap();
        },
    );
}

#![allow(unused)]

use super::types::{CaptureOptions, Region};
use crate::prelude::*;
use std::sync::mpsc::Sender;
use tauri::window::PlatformWebview;
use webview2_com::Microsoft::Web::WebView2::Win32::COREWEBVIEW2_CAPTURE_PREVIEW_IMAGE_FORMAT_PNG;
use windows::Graphics::Imaging::BitmapDecoder;
use windows::Graphics::Imaging::ImageStream;
use windows::Storage::Streams::IRandomAccessStream;
use windows::Web::UI::IWebViewControl;
use windows::Win32::Media::DirectShow::IMpeg2Stream;
use windows::Win32::{System::Com::IStream, UI::Shell::SHCreateMemStream};

use webview2_com::CapturePreviewCompletedHandler;
pub fn snapshot(
    webview: PlatformWebview,
    region: Option<Region>,
    capture_options: Option<CaptureOptions>,
    tx: Sender<Result<Vec<u8>>>,
) {
    let handler = CapturePreviewCompletedHandler::create(Box::new(|_| Ok(())));
    unsafe {
        let core_webview = webview.controller().CoreWebView2().unwrap();
        core_webview.CapturePreview(
            COREWEBVIEW2_CAPTURE_PREVIEW_IMAGE_FORMAT_PNG,
            imagestream,
            &handler,
        );
    }
}

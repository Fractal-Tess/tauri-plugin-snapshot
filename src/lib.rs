#![allow(unused)]

mod error;
mod plugin;
mod prelude;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Wry,
};

use plugin::snapshot;

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("screen-shot")
        .invoke_handler(tauri::generate_handler![snapshot])
        .build()
}

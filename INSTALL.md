# Installing tauri-plugin-snapshot

The plugin is not published to crates.io or npm. Install both halves from this repository, and pin a commit if you want reproducible builds.

## Rust

In `src-tauri/Cargo.toml`:

```toml
[dependencies.tauri-plugin-snapshot]
git = "https://github.com/Fractal-Tess/tauri-plugin-snapshot"
# rev = "<commit sha>"
```

Register it in `src-tauri/src/main.rs`:

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_snapshot::init())
```

## JavaScript

```sh
npm install github:Fractal-Tess/tauri-plugin-snapshot
# or pin a commit
npm install github:Fractal-Tess/tauri-plugin-snapshot#<commit sha>
```

The package is called `tauri-plugin-snapshot-api` and ships prebuilt JS in `webview-dist/`.

Another option is to add the repository as a git submodule and point both `Cargo.toml` (`path = ...`) and `package.json` (`"file:..."`) at it. The example app does this with relative paths.

## Options

Both `snapshotViewport` and `snapshotDocument` take the same optional object:

```ts
{
  save?: { path: string; overwrite?: boolean };          // also write the PNG to disk
  capture?: { transparentBackground?: boolean; highlighted?: boolean }; // Linux only
}
```

Without `overwrite`, saving to an existing file fails.

## Platform support

| Platform | Backend | Notes |
| -------- | ------- | ----- |
| Linux | WebKitGTK (`webkit_web_view_get_snapshot`) | Viewport and document regions, both capture options |
| Windows | WebView2 (`CapturePreview`) | Always captures the visible area; region and capture options are ignored |
| macOS | none | Not implemented; calling `snapshot` panics |

## Linux system libraries

Tauri 1 links against `webkit2gtk-4.0` and `libsoup-2.4`. Current distributions and nixpkgs have dropped these in favour of the 4.1 / libsoup 3 versions, so on a recent system you may need an older package set (for example nixpkgs `nixos-24.05`) to build a Tauri 1 app at all. See [CONTRIBUTING.md](CONTRIBUTING.md) for the shell used to build the example.

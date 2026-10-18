# Contributing

## Layout

- `src/` is the Rust plugin. `src/plugin.rs` holds the `snapshot` command; each platform backend lives in `src/plugin/<os>.rs`.
- `webview-src/` is the TypeScript API, bundled with Rollup into `webview-dist/`. Run `pnpm build` (or `yarn build`) after changing it and commit the output.
- `example/` is a small Tauri app with buttons for each capture mode.

## Running the example on NixOS

Tauri 1 needs `webkit2gtk-4.0`, which current nixpkgs no longer ships. A dev shell that takes the GTK and WebKit libraries from `nixos-24.05` and a current Rust toolchain from unstable works (Rust 1.77 from 24.05 is too old for today's crate versions):

```nix
{
  inputs.old.url = "github:NixOS/nixpkgs/nixos-24.05";
  inputs.new.url = "nixpkgs";
  outputs = { old, new, ... }: let
    p = old.legacyPackages.x86_64-linux;
    n = new.legacyPackages.x86_64-linux;
  in {
    devShells.x86_64-linux.default = p.mkShell {
      nativeBuildInputs = [ p.pkg-config n.cargo n.rustc ];
      buildInputs = with p; [ webkitgtk gtk3 libsoup cairo glib openssl gdk-pixbuf pango atk harfbuzz ];
    };
  };
}
```

Then:

```sh
cd example
pnpm install && pnpm build
cd src-tauri
nix develop /path/to/that/flake -c cargo build --release
nix develop /path/to/that/flake -c ./target/release/app
```

"Document to path" opens a native save dialog, which needs GSettings schemas on `XDG_DATA_DIRS`; without them the app aborts.

## README images

The README cover is composed in `assets/screenshots/source/hero.html` from raw captures in `assets/screenshots/source/shots/`, which are not committed. To remake it:

1. Run the example at `GDK_SCALE=2` (a 1600 × 1200 window).
2. Save a screenshot of the window as it opens as `shots/start.png`.
3. Click "Viewport" three times and save another as `shots/viewport.png`.
4. Render and convert:

```bash
google-chrome-stable --headless=new --hide-scrollbars --force-device-scale-factor=2 \
  --window-size=1600,1000 --screenshot=/tmp/hero.png assets/screenshots/source/hero.html
cwebp -q 82 /tmp/hero.png -o assets/screenshots/hero.webp
```

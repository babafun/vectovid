# Development notes

This file documents how to build the `core` wasm artifacts and the desktop binary locally.

Prerequisites
- Rust toolchain (stable)
- `wasm-pack` for web-target builds (optional but recommended)

Build core for web (wasm)

```sh
cd core
wasm-pack build --target web --out-dir pkg --release

# After this you'll have `core/pkg/vectovid_core.js` and `vectovid_core_bg.wasm`.
```

WASM helper usage

- The repo includes an example at `web/example_wasm.html` which loads `core/pkg/vectovid_core.js` and demonstrates `pack_vvf`.

Local dev iter

- When iterating on the Rust wasm, run in `core`:

	```sh
	wasm-pack build --target web --out-dir pkg --dev
	```

Build desktop (Linux)

```sh
cargo build -p vectovid_desktop --release

# Binary will be at `target/release/vectovid_desktop`.
```

Build desktop (Windows MSVC)

```ps1
# On Windows with MSVC toolchain
cargo build -p vectovid_desktop --release
# Binary at `target\release\vectovid_desktop.exe`
```

Serving the web UI locally

From repository root (simple HTTP server):

```sh
python3 -m http.server 8000
# open http://localhost:8000/web/index.html
```

CI

- `.github/workflows/build/web.yml` builds the core wasm (wasm-pack) and Linux desktop.
- `.github/workflows/build/windows.yml` builds the desktop on Windows and uploads the artifact.

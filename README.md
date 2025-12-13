# vectovid

Vector-based video format that I'm working on.

Repository layout (new):

- `web/` — the web UI (moved from project root): `index.html`, `player.html`, JS and CSS.
- `core/` — primary Rust core crate (WASM-targetable via `wasm-bindgen`).
- `desktop/` — example desktop binary crate that uses the core library.

Development notes:

- Build the Rust workspace (requires Rust toolchain):

	```sh
	cargo build -p vectovid_desktop
	```

+- Build the core for WASM with `wasm-pack` or `cargo build --target wasm32-unknown-unknown` and `wasm-bindgen`.


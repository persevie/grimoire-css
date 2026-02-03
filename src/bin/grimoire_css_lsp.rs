//! Feature-gated LSP server (stdio).
//!
//! Build: `cargo build --features lsp --bin grimoire_css_lsp`
//! Run (stdio): `target/debug/grimoire_css_lsp`

#[tokio::main]
async fn main() {
    grimoire_css_lib::lsp::serve_stdio().await;
}

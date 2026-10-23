# static_site — Zero-JS Static Site Builder

Renders the markdown sources the WASM app embeds (`src/ui/loader.rs`) into
plain HTML pages styled by `web/style.css`. No JavaScript: no theme picker,
no WASM, default (Basic Dark) palette.

Site content lives in this directory: the page manifest (`PAGES` in
`render.rs`), the embed files (`logo_big.html`, `logo_small.html`,
`moose_clicker.html`), and the markdown-to-HTML renderer.

Defined in the root `Cargo.toml` as:

```toml
[[bin]]
name = "static_site"
path = "static_site/main.rs"
```

## Usage

Normally invoked through the package tool — `static` target for a full
build, or automatically inside the `wasm` target so `docs/` always holds
both experiences:

```bash
./package static          # full build via the package tool
./package wasm --preview  # wasm build, static pages regenerated inside it
```

Direct invocation (writes into `docs/` the same way):

```bash
cargo run --bin static_site  # full build: WASM shell + pages + images
```

The package tool runs it with `cargo run --release --quiet --bin static_site`,
so the build happens as part of packaging — there is no separate build step.
There is also no repo-root copy of the binary: `./static_site` is this
tool's source directory.

To prebuild it (e.g. for CI), use the alias in `.cargo/config.toml`:

```bash
cargo build-static-site
```

## Adding a page or embed

- New markdown file → append `(src, out)` to `PAGES` in `render.rs` **and**
  to the `markdown_load!` list in `src/ui/loader.rs` (the lists are
  deliberately not identical — see each file's comments).
- New `<embed src="...">` → add a branch in `raw_html_line` in
  `render.rs` **and** a matching `.handle(...)` widget registration in
  `src/ui/markdown.rs`. Unknown embeds render as nothing.

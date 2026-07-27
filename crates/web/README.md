# lestallum-web

The Leptos app for Lestallum Town. Compiled twice: once with the `ssr` feature into the
server (driven by `lestallum-core`, which calls `run()` from `src/lib.rs`), and once with
the `hydrate` feature into the WASM bundle that takes over in the browser.

The `ssr` build also mounts `lestallum-api` under `/api`.

## Routes

| Path           | Page                 |
| -------------- | -------------------- |
| `/`            | `pages::home`        |
| `/areas`       | `pages::areas`       |
| `/areas/:area` | `pages::area_detail` |
| `/rules`       | `pages::rules`       |
| anything else  | `pages::not_found`   |

`/areas/:area` matches the `slug` field of the entries in `src/data.rs`; an unknown slug
renders the in-page "unknown district" view rather than the global 404.

The navbar and footer also link to `/map`, `/lore`, `/gallery` and `/wiki`. Those routes do
not exist yet and currently fall through to the 404 page.

## Layout

- `src/app.rs` — document shell and the route table
- `src/data.rs` — the `AREAS` table (name, theme, accent colors, screenshots) and `DISCORD_URL`
- `src/pages/` — one module per route
- `src/layouts/main.rs` — navbar + overlay animations + footer wrapper used by every page
- `src/components/sections/` — navbar and footer
- `src/components/animations/` — scroll-triggered reveal and the randomly scheduled overlay videos
- `public/` — static assets, copied to the site root by cargo-leptos
- `style/main.scss` — plain SCSS; `style/tailwind.css` — Tailwind entrypoint with the daisyUI theme

Both stylesheets are compiled and concatenated into `target/site/pkg/lestallum-web.css`.

## Building

Tailwind pulls in `daisyui` and `tw-animate-css` through `@plugin` / `@import`, which are
resolved from `node_modules` in this directory. Install them once before the first build:

```bash
npm install
```

Then build or watch from the workspace root, since the `[[workspace.metadata.leptos]]`
config lives there:

```bash
cargo leptos watch
cargo leptos build --release
```

`cargo build --workspace` compiles the server side only and skips the WASM and CSS steps,
which makes it a much faster check while working on the `ssr` code.

To check the browser build on its own:

```bash
cargo check -p lestallum-web --target wasm32-unknown-unknown --no-default-features --features hydrate
```

## End-to-end tests

Playwright specs live in `end2end/tests`. Run `npm install` in `end2end/` once, then:

```bash
cargo leptos end-to-end
cargo leptos end-to-end --release
```

## Licensing

`LICENSE` in this directory is the Unlicense that shipped with the Leptos starter template
and covers the template scaffolding only.

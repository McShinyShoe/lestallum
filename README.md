# Lestallum

This repository contains the code for Lestallum Website and Bots.
The stuff in here is made for **Lestallum Town**, a Minecraft town from **TheCavern** Minecraft server.

This project utilizes async + multithreading to divide the services, and cross compile to WASM for the frontend.
All of this compiles into a single rust binary for the app with a monolithic design, and each bot and website have its own thread.

- Site: <https://lestallum.shinyshoe.net>
- Source: <https://github.com/McShinyShoe/lestallum>

## Stack

| Layer     | Choice                                                        |
| --------- | ------------------------------------------------------------- |
| Frontend  | Leptos 0.8 (SSR + WASM hydration), Tailwind 4, daisyUI 5      |
| Server    | Axum 0.8, served by `cargo-leptos`                            |
| Database  | Postgres 17 via SeaORM 2.0                                    |
| Runtime   | Tokio, one runtime per service thread                         |
| Packaging | Multi-stage Docker build, `docker compose` for local Postgres |

## Workspace layout

```text
lestallum
├── crates
│   ├── core           binary entrypoint, it loads config, connects the DB, and spawns the services
│   ├── web            Leptos app, feature-gated `ssr` (server) / `hydrate` (browser)
│   ├── api            Axum handlers, mounted under /api by the web crate's `ssr` build
│   ├── bot-discord    Discord integration
│   ├── bot-minecraft  Minecraft server integration
│   ├── shared         AppConfig, AppState, and the outbound HTTP clients
│   └── db             SeaORM entities and controllers, connection pool setup
│       └── migration  SeaORM migration CLI
├── docker             dockerfile and docker-compose.yml
├── config.toml        committed defaults for local development
└── deny.toml          cargo-deny license/advisory policy
```

`main.rs` builds the config and database pool at first, then starts `web`, `bot-discord` and `bot-minecraft` on three OS threads, and each of them have its own current-thread Tokio runtime.
All three share a `Arc<AppState>` holding the config and the shared controllers.
If any thread returns an error it auto restarts the thread.

## Crate dependencies

```mermaid
graph TD
    core["lestallum-core<br/>(binary)"]
    web["lestallum-web"]
    api["lestallum-api"]
    discord["bot-discord"]
    minecraft["bot-minecraft"]
    shared["shared"]
    db["db"]
    migration["migration<br/>(standalone CLI)"]

    core --> web
    core --> discord
    core --> minecraft
    core --> shared
    core --> db

    web -.ssr.-> api
    web -.ssr.-> shared

    discord --> shared
    minecraft --> shared

    shared --> db
```

Every edge is a `path` dependency inside the workspace. Dotted edges are optional and only turn on with the feature named on them, so the WASM build of `lestallum-web` pulls in neither `lestallum-api` nor `shared`.

| Crate            | Depends on                           |
| ---------------- | ------------------------------------ |
| `lestallum-core` | `web` (ssr), `bot-*`, `shared`, `db` |
| `lestallum-web`  | `api` (ssr), `shared` (ssr)          |
| `lestallum-api`  | none                                 |
| `bot-discord`    | `shared`                             |
| `bot-minecraft`  | `shared`                             |
| `shared`         | `db`                                 |
| `db`             | none                                 |
| `migration`      | none                                 |

:::info
`migration` is not here cause it is its own binary, run against a live database, and nothing in the app links it.
`db` sits at the bottom with no workspace dependencies, so `shared` can embed `db`'s `DatabaseConfig` inside `AppConfig` without a cycle.
:::

## Prerequisites

- A Rust tool chain new enough for edition 2024 (1.85+), the release image builds on stable
- The WASM target: `rustup target add wasm32-unknown-unknown`
- `cargo install cargo-leptos --locked`
- Node.js, for the Tailwind/daisyUI plugins
- Docker, or a Postgres 17 instance you point the config at

## Getting started

```bash
# 1. Tailwind plugins (daisyui, tw-animate-css) resolve from crates/web/node_modules
npm install --prefix crates/web

# 2. Postgres on localhost:5432 with the credentials in config.toml
docker compose -f docker/docker-compose.yml up -d db

# 3. Apply migrations
DATABASE_URL=postgres://lestallum:lestallum@localhost:5432/lestallum \
  cargo run -p migration -- up

# 4. Dev server with rebuild-on-save at http://127.0.0.1:3000
cargo leptos watch
```

`cargo build --workspace` compiles the server side only, skipping the WASM and CSS steps.
It is the fastest way to check `ssr` code while working.

## Configuration

All them configs are on`AppConfig` (`crates/shared/src/app_config.rs`). It layers four sources, the lowest priority first:

1. Built-in defaults (`127.0.0.1:3000`, pool sizes, timeouts, and `DATABASE_URL` if set)
2. `config.toml`: committed, local-development values
3. `config.local.toml`: gitignored, for your own overrides
4. `APP_`: prefixed environment variables, with `__` separating nested keys

| Key                             | Env                             | Default     |
| ------------------------------- | ------------------------------- | ----------- |
| `host`                          | `APP_HOST`                      | `127.0.0.1` |
| `port`                          | `APP_PORT`                      | `3000`      |
| `database.url`                  | `APP_DATABASE__URL`             | none        |
| `database.max_connections`      | `APP_DATABASE__MAX_CONNECTIONS` | `10`        |
| `database.min_connections`      | `APP_DATABASE__MIN_CONNECTIONS` | `1`         |
| `database.connect_timeout_secs` | ...                             | `8`         |
| `database.acquire_timeout_secs` | ...                             | `8`         |

Two things to watch:

- `DATABASE_URL` is applied as a *default*, so the `database.url` in `config.toml` wins over it.
  To point a local checkout at another database, use `config.local.toml` or `APP_DATABASE__URL`.
  `DATABASE_URL` does work in the container, which ships no `config.toml`, and it is what the migration CLI reads.
- The container binds `0.0.0.0:3000` on purpose, set through `APP_HOST`/`APP_PORT` in the
  dockerfile, so Docker can route to it. (the reason you can change it is if somehow you want to just not use docker ig :P)

::: warning
The Postgres credentials in `config.toml` and `docker/docker-compose.yml` (`lestallum:lestallum`) are local placeholders.
Anything resembling a real deployment needs injected secrets instead.
`DatabaseConfig`'s `Debug` impl redacts the URL, so the connection string does not reach the logs.
:::

Logging goes through `tracing_subscriber`'s env filter, you can change this by setting `RUST_LOG` (`info` in the
container). SQL statements are logged at `debug`.

## HTTP surface

| Path           | Handler              |
| -------------- | -------------------- |
| `/`            | `pages::home`        |
| `/areas`       | `pages::areas`       |
| `/areas/:area` | `pages::area_detail` |
| `/rules`       | `pages::rules`       |
| `/api/health`  | `{"status":"ok"}`    |
| anything else  | `pages::not_found`   |

:::info
`:area` matches the `slug` fields in `crates/web/src/data.rs`
:::

The document shell loads a Microsoft Clarity analytics script from `clarity.ms`.

## Database

Two tables so far, both created by the migration crate:

- `users`: `id` (string primary key, not auto-assigned), unique `mc_name`, `password_hash`
- `user_create_requests`: serial `id`, indexed `mc_user`, unique 6-character `code`,
  `active_until` timestamp with time zone

Query them through `DatabaseController` (`crates/db/src/controllers/`) rather than making your own query directly. Full CLI usage lives in `crates/db/migration/README.md`. For example:

```bash
cargo run -p migration -- up
cargo run -p migration -- status
cargo run -p migration -- generate MIGRATION_NAME
```

## Outbound APIs

`shared::api::ApiController` wraps a rustls-backed `reqwest` client for three upstreams:

- GeyserMC, for linking Bedrock and Java accounts by XUID, UUID, or gamertag
- Mojang, for username/UUID lookups and decoding the texture blob into skin and cape URLs
- `mc-heads.net`, for getting heads ;)

Every helper validates its input before it reaches a URL, bounds the response body (64 KiB
for JSON, 2 MiB for images), and maps failures onto a single `ApiError`. Nothing wires the
controller into `AppState` yet.

## Docker

```bash
docker compose -f docker/docker-compose.yml up --build
```

This builds the release image and starts it against a Postgres container, waiting on that
container's health check first. The app is exposed on port 3000. The build stage installs
`cargo-leptos` and runs `cargo leptos build --release`, which uses the size-optimized
`wasm-release` profile for the browser bundle, the runtime stage is a slim Debian image
carrying only the binary, `target/site`, and an unprivileged `app` user.

Migrations are not run by the container. Apply them separately before starting it.

## Contributing

Commit message format, versioning policy, and code conventions live in
[CONTRIBUTING.md](CONTRIBUTING.md). Run the checks listed there before opening a PR.

Vulnerabilities go through the private path in [SECURITY.md](SECURITY.md), not a public issue.

## License

GPL-3.0-or-later. See [LICENCE](LICENCE).

`crates/web/LICENSE` is the Unlicense that came with the Leptos starter template and covers
that scaffolding only.

The `deny.toml` allow list never admits AGPL-3.0, since GPL-3.0 §13 would extend the AGPL
network clause to the entire application.

# Security Policy

## Project status

lestallum is pre-release. There are no tagged releases and no version numbers, so security
fixes land on `master` and nowhere else. There is nothing to backport to. Anyone
self-hosting this should track `master` and rebuild to pick up fixes.

This section becomes a supported-versions table once the project starts tagging releases.

## Reporting a vulnerability

Do not open a public issue, pull request, or discussion for a security problem.

Report privately through GitHub:
[Security → Report a vulnerability](https://github.com/McShinyShoe/lestallum/security/advisories/new).
This keeps both the report and the fix private until an advisory is published.

If you would rather not use GitHub, email mcshinyshoe@protonmail.com with `SECURITY` in the
subject line.

Please include:

- what the issue is, and which crate or file it lives in
- steps to reproduce, ideally against a local `docker compose` deployment
- the commit SHA you tested against
- what an attacker gains, and what access they need to start
- whether this has been discussed publicly anywhere already

One vulnerability per report. If you find several unrelated issues, send several reports.

## What to expect

One person maintains this in their spare time, so these targets are what a single
maintainer can honestly commit to.

| Stage | Target |
| --- | --- |
| Acknowledgement of your report | 7 days |
| Initial assessment, accepted or rejected | 30 days |
| Fix on `master` | severity dependent, agreed with you |

A rejected report gets a reason, not silence.

## Disclosure

Disclosure is coordinated. Please keep the report private until a fix reaches `master` or
90 days pass, whichever comes first. If a fix will realistically take longer than 90 days,
we agree on a new date rather than letting the deadline lapse quietly.

Accepted reports get a published GitHub Security Advisory. Reporters are credited by name
or handle unless you ask to stay anonymous.

## Scope

In scope:

- the Axum API in `crates/api`
- the Leptos server and hydration code in `crates/web`
- database access and migrations in `crates/db`
- config and secret handling in `crates/shared`
- the Discord and Minecraft integrations in `crates/bot-discord` and `crates/bot-minecraft`
- the container build under `docker/`

Areas that carry the most damage, and are worth the most attention:

- any path reaching the Discord bot token, Minecraft RCON credentials, or `DATABASE_URL`
- any path where text from Discord or the web UI reaches an RCON command, since that
  boundary requires validation and allowlisting on the lestallum side
- credential storage on the `users` table
- authentication and session handling, once it exists
- handling of third-party responses in `crates/shared/src/api`, which fetches from the
  Geyser, Mojang, and mc-heads APIs and parses data this project does not control

Both bots are currently stubs and the API exposes only `/health`, so the live attack
surface is small today. Reports against code that is merged but not yet wired up are still
welcome.

## Not in scope

The items below are known and deliberate. Reporting them gets a pointer back to this
section.

- The `lestallum:lestallum` credentials in `docker/docker-compose.yml`. A local development
  placeholder, documented as such, used in no deployment.
- The container binding `0.0.0.0:3000`. Intentional, and required for Docker networking.
  The default in `config.toml` and `AppConfig` is `127.0.0.1`.
- `RUSTSEC-2024-0436` (`paste`) and `RUSTSEC-2026-0173` (`proc-macro-error2`). Both crates
  are unmaintained, neither is vulnerable. They arrive transitively through the Leptos
  macros and carry documented reasons in `deny.toml`.
- Dependency vulnerabilities that already have an upstream advisory, unless lestallum uses
  the crate in a way that makes the impact worse than upstream describes.
- Missing hardening that is not exploitable on its own, such as absent security headers on
  a route serving static content.
- Anything requiring physical access to the host, or a host that is already compromised.
- Denial of service through raw traffic volume.
- Scanner output pasted without a working reproduction.

## Research guidelines

Test against a local deployment you run yourself. Do not test against any live instance you
do not own.

Do not access, modify, or destroy data belonging to anyone else. If you encounter someone
else's data during testing, stop and say so in your report. No social engineering, no
phishing, and no attacks on infrastructure or accounts belonging to project contributors.

Research that follows these guidelines is welcome and will not be treated as an
unauthorised act.

There is no bug bounty. This is an unfunded personal project and the only reward on offer
is credit in the advisory.

## Dependency and supply chain hygiene

`cargo audit` and `cargo deny check` run whenever a dependency changes. The license
allowlist in `deny.toml` is deliberately closed, and unknown registries and git sources are
denied. `Cargo.lock` is committed and stays committed.

AGPL-3.0 is never admitted into the dependency graph. Under GPL-3.0 §13 it would extend the
AGPL network clause across the whole application, which is not the license this project
ships under.

`unsafe` is forbidden workspace-wide via `unsafe_code = "forbid"`. A report showing that
constraint has been bypassed is in scope.

## License

lestallum is GPL-3.0-or-later. Reporting a vulnerability transfers no rights, and nothing
in this policy overrides the terms in [LICENCE](LICENCE).

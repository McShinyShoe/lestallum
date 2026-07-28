# Contributing

Thanks for looking at this. The [README](README.md) covers the prerequisites, the workspace
layout, and how to get a dev server running. This file covers everything that only matters
once you are about to change something.

## Checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo audit
cargo deny check
```

Clippy has to pass clean before a commit. `unsafe` is forbidden workspace-wide.

Playwright specs live in `crates/web/end2end/tests` and run with `cargo leptos end-to-end`.

## Commit messages

This project follows [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/).
A message is structured as:

```text
<type>[optional scope][!]: <description>
```

The type is one of the eleven from
[`@commitlint/config-conventional`](https://github.com/conventional-changelog/commitlint/tree/master/%40commitlint/config-conventional):

| Type       | Use for                                                                       |
| ---------- | ----------------------------------------------------------------------------- |
| `feat`     | a new capability: a route, an API endpoint, a config option, a bot command    |
| `fix`      | a bug fix                                                                     |
| `docs`     | documentation only, including the README and the per-crate ones               |
| `refactor` | restructuring that neither fixes a bug nor adds a feature                     |
| `perf`     | a change made specifically to improve performance                             |
| `test`     | adding or correcting tests                                                    |
| `build`    | the build itself: `Cargo.toml`, the dockerfile, cargo-leptos config, npm deps |
| `ci`       | CI configuration                                                              |
| `style`    | formatting with no behavior change, such as a stray `cargo fmt`               |
| `revert`   | reverting an earlier commit                                                   |
| `chore`    | anything none of the above covers                                             |

The scope is optional and names the crate or area touched: `core`, `web`, `api`, `db`,
`migration`, `shared`, `bot-discord`, `bot-minecraft`, `docker`, `deps`.

Rules for the subject line:

- Past tense, describing what the commit did. Write it, so it completes the sentence "this commit ...",
  which makes it `Added password hash to users`, not `Add password hash to users`.
- Capitalize the first word after the colon. No trailing period.
- Keep the header under 100 characters. Under 72 keeps `git log --oneline` readable.
- The header is the whole message, with no body and no footers.

The spec constrains neither the tense nor the casing of the description, so both of those are this project's own convention.
Note that `@commitlint/config-conventional`, borrowed above for its type list, is stricter: its `subject-case` rule rejects a capitalized subject.
Set that one rule to `0` if commitlint ever lands in CI.

Detail that does not fit in the header goes in the PR description, where it stays reviewable and linkable.

```text
feat(api): Added rate limiting to the health route
```

A breaking change takes a `!` before the colon:

```text
feat(shared)!: Moved the bind address out of config.toml
```

Under semantic versioning, `fix` maps to a patch release, `feat` to a minor one, and any
breaking change to a major one.

## Versioning

Releases follow [Semantic Versioning 2.0.0](https://semver.org/): `MAJOR.MINOR.PATCH`.

This workspace ships an application rather than a library, and every crate is
`publish = false`. The "public API" that SemVer talks about is therefore not the Rust
surface, which nobody outside the repo can link against. It is what someone outside the
repo actually depends on:

- the routes under `/`, and the JSON shape anything under `/api` returns
- config keys and their `APP_*` environment equivalents
- Discord and in-game bot commands
- the database schema, and whether a migration rolls back cleanly
- the container contract: exposed port, expected environment variables

Bump against that surface:

| Bump    | When                                                                                                                           |
| ------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `MAJOR` | an incompatible change: a route or config key renamed or removed, a response field dropped, a migration with no working `down` |
| `MINOR` | a backward compatible addition: a new route, an optional config key, a new bot command                                         |
| `PATCH` | a backward compatible bug fix that leaves the surface alone                                                                    |

The project is at `0.1.0` with nothing tagged yet. SemVer treats `0.y.z` as initial
development, where anything may change at any time, so bump the minor for each release
while the leading zero holds. Move to `1.0.0` once the site is serving real traffic and the
routes and config keys are ones worth keeping.

Deprecate before you remove. Document the change and ship it in a minor release with the
old behavior still working, then drop it in the next major.

Release candidates append a pre-release identifier, which sorts *below* the release it
precedes (`1.0.0-rc.1` < `1.0.0`):

```bash
git tag -a v1.0.0-rc.1 -m "release candidate 1"
git tag -a v1.0.0 -m "first stable release"
git push --tags
```

The `version = "0.1.0"` in each crate manifest is an unused placeholder, since nothing
publishes and `Cargo.lock` pins the internal crates by path. The tag is the release version.
If you would rather keep the manifests in lockstep with it, add `version` to
`[workspace.package]` and inherit it with `version.workspace = true`, the way `license`
already works.

## Code

Conventions worth knowing before opening a PR:

- Add `// SPDX-License-Identifier: GPL-3.0-or-later` header every `.rs` file carries, followed by a blank line.
- Comment where a comment earns its place. Say why the code does what it does, the code itself already says what.
- No `.unwrap()`, `.expect()`, or panicking indexing outside tests and startup code in `main.rs`.
- Propagate with `Result` and `?`, and index with `.get()`.
- Check a new dependency's RustSec status, last publish date, and download count before adding it,
  and prefer a crate already in the workspace over a near-duplicate.
- `Cargo.lock` is committed and stays committed.
- A new route, config option, or bot command gets its README update in the same PR.
- Never build SQL by formatting request data into a string,
  and never concatenate user-supplied text into a Minecraft RCON command.
- Tokens, passwords, and connection strings come from the environment. Nothing at `info` or louder may contain one.

The `deny.toml` license allow list is deliberately closed, so a new dependency may need an entry there.
AGPL-3.0 must never be admitted: `GPL-3.0 §13` would extend the AGPL network clause to the entire application.

## Security

Do not open a public issue for a vulnerability. [SECURITY.md](SECURITY.md) has the private
disclosure path.

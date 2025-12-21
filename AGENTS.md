# Repository Guidelines

## Project Structure & Modules
- Core Rust sources live in `src/`; GTK UI flow is in `src/app.rs`, `src/ui.rs`, and state helpers in `src/state.rs`. Data handling sits in `src/data.rs` and `src/storage.rs`; optional SQLite plumbing is in `src/db.rs`.
- CLI importers reside in `src/bin/import.rs` and `src/bin/tanzil_import.rs` (feature-gated by `sqlite`).
- Assets are under `assets/` (`quran/` JSON, `translations/`, `icons/`, `desktop/`, `fonts/`). Packaging helpers are `PKGBUILD`, `hyprquran.install`, `verify_package.sh`, and `test_package.sh`.

## Build, Test, and Development Commands
- Build GUI app (default features include GTK): `cargo build --release` then run `./target/release/hyprquran`.
- Headless build without GTK (useful for CI/lint): `cargo build --no-default-features`.
- Importers with SQLite only: `cargo run --no-default-features --features sqlite --bin import` (or `tanzil_import`) after ensuring the DB path is configured.
- Tests (no GTK needed): `cargo test --no-default-features`.
- Lint/format: `cargo fmt` and `cargo clippy --all-targets --all-features -- -D warnings`.

## Coding Style & Naming Conventions
- Rust 2021 edition; use rustfmt defaults (4-space indent). Keep modules small and prefer `Result<T, anyhow::Error>` for fallible paths.
- Opt-in features: `gui` for GTK, `sqlite` for database-backed import/search. Guard UI-only code with `#[cfg(feature = "gui")]` as in `src/lib.rs`.
- File/layout naming mirrors responsibility (e.g., `surah_index.rs` for static metadata, `logging.rs` for tracing setup); follow that pattern for new modules.

## Testing Guidelines
- Primary tests live in `src/lib.rs`; add module-level `#[cfg(test)]` blocks near the code they verify.
- Name tests descriptively (`test_load_surah_json`, `test_insert_bookmark`) and prefer unit tests over integration until the SQLite schema stabilizes.
- If touching data import, add a lightweight fixture under `assets/quran/` or `assets/translations/` and validate parse/shape with `cargo test --no-default-features`.

## Commit & Pull Request Guidelines
- Use concise, imperative summaries similar to existing history (`Add …`, `P2: …`). Mention scope or priority tags when relevant.
- In PRs, include: what changed, why, how to test (exact commands), and screenshots/gifs for UI-facing tweaks. Link related issues or `TASKS.md` items.
- Keep commits logically scoped; avoid mixing formatting with feature work. Run `cargo fmt` and `cargo clippy` before pushing.

## Packaging & Release Notes
- Arch packaging artifacts (`hyprquran-*.pkg.tar.zst`) are produced via `makepkg` with the provided `PKGBUILD`. Validate contents using `./verify_package.sh`.
- Ensure the Amiri Quran font is available in the package payload and that `assets/desktop/hyprquran.desktop` launches the built binary.***

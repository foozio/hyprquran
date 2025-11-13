# Framework Decision
- Choose `gtk4` via `gtk-rs` with `pango` for text shaping
- Justification: native Wayland support, perfect Arabic shaping via Pango/HarfBuzz, low startup latency, smaller dependency surface than custom `wgpu` pipelines

# Architecture
- Binaries: `hyprquran`
- Modules: `app` (startup, app_id), `ui` (widgets/layout), `text` (Arabic rendering helpers), `data` (Qur'an/translation models, index, search), `state` (navigation, settings), `storage` (bookmarks, progress), `config` (paths, preferences), `logging` (tracing)
- Pattern: MVC-ish; UI reacts to `state`, actions mutate `state` and call `data/storage`

# Dependencies
- `gtk4`, `pango`, `glib`, `gio`
- `serde`, `serde_json`, `anyhow`, `thiserror`, `tracing`, `tracing-subscriber`, `once_cell`, `directories`
- Optional: `rusqlite` (feature `sqlite`) for persistent bookmarks/progress

# Data Model
- `Surah { id, name_ar, name_en, ayah_count }`
- `AyahRef { surah_id, ayah_index }`
- `AyahText { original_ar, uthmani_ar, codepoints }`
- `TranslationEntry { surah_id, ayah_index, lang, text }`
- JSON schema: `{ lang: "en"|"id", entries: [{ surah: 1, ayah: 1, text: "..." }] }`
- Indexes: maps for fast lookup by `AyahRef` and reverse search index for full-text search

# Text Rendering
- Use `gtk::TextView` or `gtk::Label` with `pango::Layout` configured for RTL
- Set `WidgetExt::set_direction(gtk::TextDirection::Rtl)` and `pango::attr_list` for font, size, line spacing
- Font recommendations: `Scheherazade New` or `Amiri Quran` with full diacritics
- High-quality typography: increased line-height, kerning enabled, disable ligature breaking

# UI Layout
- Single floating window with `GtkHeaderBar`
- Header: Surah selector (`ComboBoxText`), Ayah selector (`SpinButton`), Search (`SearchEntry`), Translation toggle (`ToggleButton`), Settings (`MenuButton`)
- Body: two `ScrolledWindow`s stacked: Arabic text top, translation bottom; translation area collapsible
- Status bar: current surah/ayah, theme indicator

# Navigation & Shortcuts
- Jump: pick surah by name, ayah by number
- Search: live filter across translations and Arabic (by codepoint normalized)
- Shortcuts: `Ctrl+F` search, `Enter` next result, `Shift+Enter` previous, `Ctrl+J` quick jump dialog, `Alt+Left/Right` prev/next ayah, `Alt+Up/Down` prev/next surah, `T` toggle translation, `B` bookmark, `Ctrl+L` toggle theme

# Translation Support
- Load JSON translations lazily per-surah
- Toggle display per language: English, Indonesian
- Clear separation: Arabic area uses RTL, translation area LTR, distinct typography

# Window Management
- Set stable `app_id` (e.g., `org.hyprquran.app`) for Hyprland rules
- Transparency: `window.set_opacity(conf.opacity)` with default 0.92
- Wayland compliance: rely on GTK4’s Wayland backend, no X11 dependencies
- Provide Hyprland rules:
  - `windowrulev2 = float, appid:org.hyprquran.app`
  - `windowrulev2 = opacity 0.92, appid:org.hyprquran.app`
  - `windowrulev2 = size 960 720, appid:org.hyprquran.app`

# Build System
- `Cargo.toml` with above dependencies and an optional `sqlite` feature
- Arch instructions: install `rust`, `gtk4`, `pango`, `glib2`, `glibc`; build with `cargo build --release`
- PKGBUILD outline: fetch from VCS, run `cargo build --release`, install binary to `/usr/bin`, desktop entry and icon

# Stretch Goals
- Bookmarks: JSON by default; optional SQLite when `--features sqlite`
- Reading progress: last `AyahRef` persisted per language
- Theme: follow system via GTK settings; add manual override

# System Integration
- Shell launch: minimal CLI accepting `--surah` and `--ayah`
- Hyprland keybinding examples: `bind = $mod, Q, exec, hyprquran --surah 1 --ayah 1`
- Desktop entry:
```
[Desktop Entry]
Type=Application
Name=HyprQur'an
Exec=hyprquran
Icon=hyprquran
Categories=Education;Utility;
StartupWMClass=org.hyprquran.app
```

# Error Handling & Logging
- `thiserror` enums for domain errors (I/O, JSON parse, font load)
- `anyhow` for application-level `Result`
- `tracing` with env filter; log startup, file loads, navigation, search timings

# Performance Targets
- <100MB memory: lazy loading translations, reuse `pango::Layout`, avoid large buffers
- <1s launch: precompute lightweight surah index, defer font discovery, async file I/O
- Validate with a simple benchmark and `tracing` timings

# Assets
- Icon: SVG in `assets/icons/hyprquran.svg` plus PNG
- Fonts: document recommended fonts; do not bundle
- Sample translation JSON: small subset (Al-Fatiha) for tests

# Milestones
- Phase 1: skeleton app, window, headerbar, Arabic rendering with one surah, navigation
- Phase 2: search, translations toggle, shortcuts, persistence (JSON)
- Phase 3: themes, desktop entry, PKGBUILD, performance tuning, documentation

# Deliverables
- Source: `main.rs` plus modular files under `src/`
- Documentation: README with build steps, config, shortcuts, dependencies
- Assets: icon, sample JSON, font recommendations

# Risks & Mitigations
- Arabic accuracy: use Pango, test with multiple fonts
- Wayland-only: ensure `GDK_BACKEND=wayland` during QA
- Memory: profile and lazy-load translations

# Approval
- Confirm framework choice and module breakdown
- After approval, implement Phase 1 end-to-end and share a preview
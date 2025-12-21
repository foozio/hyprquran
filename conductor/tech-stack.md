# Tech Stack: HyprQur'an

## Core Technologies
- **Programming Language:** [Rust](https://www.rust-lang.org/) - For performance, memory safety, and modern development tooling.
- **UI Framework:** [GTK4](https://www.gtk.org/) - Utilizing the `gtk4-rs` bindings for a native, high-performance Linux interface.
- **Display Protocol:** [Wayland](https://wayland.freedesktop.org/) - Targeted as the primary display protocol for modern Linux desktops.

## Data & Persistence
- **Current Storage:** JSON files - Used for initial prototyping and simplified data loading.
- **Target Storage:** [SQLite](https://www.sqlite.org/) - Planned migration for efficient indexing, searching, and management of the full Qur'an text and user metadata.
- **State Management:** Custom Rust-based state handling integrated with GTK's reactive patterns.

## Development & Build
- **Build System:** [Cargo](https://doc.rust-lang.org/cargo/) - Standard Rust package manager and build tool.
- **Logging:** `env_logger` or similar crate for structured console output.

## Deployment & Packaging
- **Target Platform:** Linux (Wayland-based distributions).
- **Packaging:** Arch Linux (PKGBUILD) with planned support for other formats (e.g., Flatpak).

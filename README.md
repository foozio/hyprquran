# HyprQur'an

HyprQur'an is a modern, Wayland-native desktop application for reading the Qur'an, built with Rust and GTK4. It aims to provide a fast, beautiful, and distraction-free experience for users on Linux-based operating systems utilizing the Wayland display protocol.

## Features

*   **Qur'an Text Display:** View the original Arabic text of the Qur'an.
*   **Translation Display:** Read translations alongside the Arabic text.
*   **Theme Switching:** Toggle between light and dark themes for comfortable reading.
*   **Font Size Adjustment:** Customize font size to your preference.
*   **Ayah Bookmarking:** Bookmark specific verses for quick access (Note: There is a known bug where bookmarks may not persist correctly; this is a high-priority fix).
*   **Keyboard Shortcuts:** Efficient navigation and control through intuitive keyboard shortcuts.

## Current Status & Limitations

HyprQur'an is currently in an early prototype stage.

*   **Limited Content:** Currently, only Surah Al-Fatiha is loaded and displayed. The application is designed to support all 114 surahs, and this functionality is a primary development focus.
*   **Data Storage:** The application uses JSON files for data in its prototype phase. Future plans include migrating to an SQLite database for more efficient data management.
*   **Bookmark Bug:** A critical bug exists where bookmarks may be lost upon certain state changes. This is the highest priority for resolution.
*   **Font Dependency:** The application relies on the "Amiri Quran" font being installed on your system for optimal Arabic text rendering.

## Installation

### Prerequisites

*   **Rust:** Ensure you have Rust and Cargo installed. You can install them via `rustup`:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
*   **GTK4 Development Libraries:** Install the GTK4 development libraries for your Linux distribution.
    *   **Debian/Ubuntu:** `sudo apt install libgtk-4-dev`
    *   **Fedora:** `sudo dnf install gtk4-devel`
    *   **Arch Linux:** `sudo pacman -S gtk4`
*   **Amiri Quran Font:** For the best Arabic text rendering, install the "Amiri Quran" font on your system.

### Building from Source

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/foozio/hyprquran.git
    cd hyprquran
    ```
2.  **Build the application:**
    ```bash
    cargo build --release
    ```
3.  **Run the application:**
    ```bash
    ./target/release/hyprquran
    ```

## Usage

Once the application is running, you can:

*   Scroll through the displayed Surah.
*   Use the UI controls or keyboard shortcuts to:
    *   Toggle between light and dark themes.
    *   Adjust the font size.
    *   Bookmark the current Ayah.

## Contributing

Contributions are welcome! Please refer to the `CONTRIBUTING.md` (if available) or the `TASKS.md` file for a list of prioritized development tasks.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Maintainer

Nuzli Hernawan (@foozio / nuzlilatief@gmail.com)

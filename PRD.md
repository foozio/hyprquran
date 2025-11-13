# Product Requirements Document (PRD) for HyprQur'an

## 1. Introduction

HyprQur'an is a Wayland-native desktop application designed for reading the Qur'an. It is built with Rust and GTK4, focusing on providing a simple, fast, and distraction-free experience for users on Linux-based operating systems using the Wayland display protocol.

## 2. Goals

*   To provide a beautiful and easy-to-use application for reading the Qur'an.
*   To be lightweight, performant, and adhere to modern desktop Linux conventions.
*   To serve as a showcase for building native desktop applications with Rust and GTK4.

## 3. Features

### 3.1. Core Functionality

*   **Surah and Ayah Display:** View the original Arabic text of the Qur'an.
*   **Translation Display:** View a translation of the Qur'an text alongside the original Arabic.
*   **Chapter Navigation:** The application currently only loads Surah Al-Fatiha. The architecture should support navigation between all 114 surahs.

### 3.2. User Experience

*   **Theme Switching:** Users can toggle between a light and dark theme for comfortable reading in different lighting conditions.
*   **Font Size Adjustment:** Users can increase or decrease the font size of the displayed text.
*   **Bookmarking:** Users can bookmark specific ayahs for future reference.

### 3.3. Shortcuts

*   The application provides keyboard shortcuts for common actions to allow for quick and efficient navigation and control.

## 4. User Flow

1.  The user launches the HyprQur'an application.
2.  The application window opens, displaying the first surah (Al-Fatiha).
3.  The user can scroll through the ayahs of the surah.
4.  The user can use keyboard shortcuts or UI buttons to:
    *   Toggle the color scheme (light/dark).
    *   Increase or decrease the font size.
    *   Bookmark the currently focused ayah.
5.  The application state (theme, font size, bookmarks) is persisted locally and restored on the next launch.

## 5. Non-Functional Requirements

*   **Platform:** The application must be a native Wayland application.
*   **Technology Stack:** The application must be built using Rust and the GTK4 toolkit.
*   **Performance:** The application should be lightweight, with low memory usage and fast startup times.
*   **Persistence:** User settings and bookmarks must be saved locally on the user's machine.
*   **Dependencies:** The application has a dependency on the "Amiri Quran" font being installed on the system.

## 6. Current Limitations & Future Scope

*   **Limited Content:** The application is currently hardcoded to display only Surah Al-Fatiha. A major next step is to implement a data loading mechanism for all 114 surahs.
*   **Data Storage:** The prototype uses JSON files for data. The inclusion of `rusqlite` suggests a plan to migrate to a more robust SQLite database for managing Qur'anic text, translations, and user data.
*   **Bookmark Instability:** There is a critical bug where bookmarks are wiped on certain state changes. This must be fixed.
*   **Search:** A search functionality is a natural and necessary feature for a future version.
*   **Translation Management:** The ability to switch between different translations or download new ones would be a valuable enhancement.

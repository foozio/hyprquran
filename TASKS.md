# Development Tasks for HyprQur'an

This document lists the development tasks required to move HyprQur'an from its current prototype state to a stable, usable application. Tasks are prioritized based on the findings in the enhancement and stability reports.

## Priority Legend

*   **P0 (Critical):** Must be fixed immediately. Blocks further development or usage.
*   **P1 (High):** Essential for core functionality and stability.
*   **P2 (Medium):** Important features that significantly improve the application.
*   **P3 (Low):** Nice-to-have features and minor improvements.

---

## P0: Critical Tasks

-   [x] **Fix Bookmark Data Loss Bug**
    -   **Description:** The current persistence logic in `src/ui.rs` wipes the bookmark list on state changes. This makes the bookmark feature unusable.
    -   **Acceptance Criteria:**
        -   Bookmarking an ayah correctly saves it to `state.json`.
        -   Changing the theme, font size, or other state does **not** delete existing bookmarks.
        -   The `add_bookmark` function in `src/storage.rs` should be used for this purpose. The general-purpose `persist` function in `ui.rs` should be removed or refactored.

## P1: High-Priority Tasks

-   [x] **Migrate Data Storage from JSON to SQLite**
    -   **Description:** The current JSON-based data loading is a prototype limitation. The application needs to use the `rusqlite` dependency to manage all Qur'anic text and translations.
    -   **Acceptance Criteria:**
        -   An SQLite database file (`quran.db`) is created and populated with data.
        -   The database schema matches the one defined in `ERD.md`.
        -   A script is created to ingest data from a source like Tanzil.net into the database.

-   [x] **Implement Data Loading for All Surahs**
    -   **Description:** Refactor the hardcoded data loading functions in `src/data.rs` to read from the new SQLite database.
    -   **Acceptance Criteria:**
        -   Functions exist to fetch a specific surah's text and translation from the database.
        -   The application can dynamically load and display any of the 114 surahs.

-   [x] **Implement Surah Navigation UI**
    -   **Description:** Add a UI component (e.g., a `Gtk::DropDown`) to allow users to select and navigate to any surah.
    -   **Acceptance Criteria:**
        -   The UI displays a list of all 114 surahs.
        -   Selecting a surah from the list loads and displays its content in the main view.

## P2: Medium-Priority Tasks

-   [ ] **Package Font with the Application**
    -   **Description:** Remove the dependency on a system-installed "Amiri Quran" font by bundling the font file with the application.
    -   **Acceptance Criteria:**
        -   The font file is included in the `assets` directory.
        -   The application loads the font at runtime, ensuring correct rendering on all systems.

-   [ ] **Implement Basic Search Functionality**
    -   **Description:** Add a search bar to allow users to search for text within the loaded translation.
    -   **Acceptance Criteria:**
        -   A search input field is added to the UI.
        -   Executing a search queries the SQLite database for matching ayahs.
        -   Search results are displayed to the user, with a way to navigate to the found ayah.

-   [ ] **Set Up CI/CD Pipeline**
    -   **Description:** Create a GitHub Actions workflow for basic quality assurance and automated builds.
    -   **Acceptance Criteria:**
        -   A workflow file is created in `.github/workflows/`.
        -   The workflow runs `cargo clippy` and `cargo test` on every push.
        -   The workflow builds the application binary.

## P3: Low-Priority Tasks

-   [ ] **Add Support for Multiple Translations**
    -   **Description:** Allow users to switch between different translations.
    -   **Acceptance Criteria:**
        -   The SQLite database is populated with at least one additional translation.
        -   A UI element (e.g., a dropdown in a settings menu) allows the user to select the active translation.

-   [ ] **Improve Error Handling**
    -   **Description:** Provide more graceful error handling for situations like a missing database file or corrupt configuration.
    -   **Acceptance Criteria:**
        -   The application displays a user-friendly error dialog instead of panicking if the database cannot be loaded.

-   [ ] **Refine UI/UX**
    -   **Description:** General improvements to the user interface, such as better spacing, iconography, or adding a dedicated settings window.
    -   **Acceptance Criteria:**
        -   The UI feels more polished and intuitive.

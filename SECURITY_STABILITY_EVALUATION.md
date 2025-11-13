# Security & Stability Evaluation for HyprQur'an

## 1. Security Evaluation

Overall, the security risk for HyprQur'an is **low**. The application is a local desktop client with a minimal attack surface.

### 1.1. Attack Surface

*   **Network:** The application does not perform any network operations. It does not connect to any remote servers, listen on any ports, or expose any APIs. This eliminates the most common vector for security vulnerabilities.
*   **File System:** The application's primary interaction with the outside world is reading and writing files on the local filesystem.
    *   It reads Qur'an data from its `assets` directory.
    *   It loads and saves a `state.json` file in a standard user configuration directory (`~/.config/hyprquran/`).
*   **Dependencies:** The application uses a set of well-known Rust crates. A supply chain attack is a theoretical possibility, but the risk is mitigated by using `Cargo`'s security features and keeping dependencies updated.

### 1.2. Potential Vulnerabilities

*   **File Path Manipulation:** An attacker with local access could potentially manipulate the `state.json` file or the data files in the `assets` directory. However, since the application is running with user-level privileges, the impact would be limited to corrupting the application's state or content for that user only. The risk is minimal and within the expected threat model for a desktop application.
*   **SQL Injection (Future Risk):** The codebase includes `rusqlite` as a dependency, indicating a plan to use an SQLite database. When this feature is implemented, care must be taken to use parameterized queries to prevent SQL injection vulnerabilities. As the feature is currently unused, this is a future consideration, not a current vulnerability.

### 1.3. Security Conclusion

The current architecture is secure for its intended purpose. The lack of network connectivity is the single most significant security feature.

## 2. Stability Evaluation

The stability of the HyprQur'an application is currently **low**, and it should be considered a **prototype or alpha-level software**. Several issues prevent it from being reliable for general use.

### 2.1. Critical Issues

*   **Bookmark Data Loss (Critical Bug):** This is the most severe stability issue. The `persist` function in `src/ui.rs` is called on most state changes (like toggling the theme or changing font size). This function incorrectly reads the current state and writes it back to `state.json`, but it fails to properly handle bookmarks. It effectively overwrites the existing list of bookmarks with an empty list, leading to immediate and silent data loss for the user. This bug makes the bookmarking feature unusable and untrustworthy.

### 2.2. Major Issues

*   **Hardcoded Content:** The application is hardcoded to load only Surah Al-Fatiha from specific JSON files (`fatiha.json`, `en_fatiha.json`, `id_fatiha.json`). This makes the application non-functional for its primary purpose of reading the entire Qur'an. It is a major architectural limitation that must be addressed.
*   **External Font Dependency:** The application's UI is styled to use the "Amiri Quran" font. If this font is not installed on the user's system, the GTK toolkit will fall back to a default font, which will likely result in incorrect rendering of the Arabic text, making the application difficult or impossible to read. This makes the user experience unreliable across different systems.

### 2.3. Minor Issues

*   **Error Handling:** While the code uses `anyhow::Result`, the error handling could be more robust. For example, if a data file is missing or corrupt, the application may panic or exit without a user-friendly message.

### 2.4. Stability Conclusion

HyprQur'an is not currently stable for end-users. The data loss bug is critical and must be fixed. The architectural limitations around data loading and external dependencies need to be resolved before the application can be considered reliable.

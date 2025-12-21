# Track Spec: Migrate to SQLite and Import Full Qur'an Data

## Overview
Currently, HyprQur'an relies on static JSON files for Qur'an text and translations. Only Surah Al-Fatiha is available. To support the full Qur'an, efficient searching, and multiple translations, we will migrate to an SQLite-based storage backend and import the full 114 Surahs.

## Goals
- Enable the `sqlite` feature in the application.
- Download the full Uthmani Qur'an text.
- Import the text into a local SQLite database using the existing `tanzil_import` utility.
- Verify that the application correctly loads and displays all Surahs from the database.
- (Optional) Clean up legacy JSON code if no longer needed, or keep as fallback.

## Acceptance Criteria
- `quran.db` is created in the user's data directory.
- Database contains all 114 Surahs and their Ayahs.
- Application launches with `sqlite` feature enabled by default (or configured).
- User can navigate to any Surah (e.g., Al-Baqarah) and see the text.
- User can search for text across the full Qur'an.

## Technical Details
- Use `download_and_convert.py` to fetch data.
- Use `src/bin/tanzil_import.rs` to import data.
- Update `Cargo.toml` to make `sqlite` the default feature (or ensure CI/builds use it).
- Ensure `src/data.rs` correctly prioritizes SQLite loading.

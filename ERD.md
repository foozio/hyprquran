# Entity-Relationship Diagram (ERD) for HyprQur'an

This document outlines the database schema for the HyprQur'an application. The current prototype uses JSON files, but the codebase includes `rusqlite`, suggesting a future migration to a relational database. This ERD is based on that assumption.

## Schema

The schema is designed to store the Qur'anic text, translations, and user-specific data like bookmarks.

```
+-----------+      +-----------+
|   Surah   |      |   Ayah    |
+-----------+      +-----------+
| surah_id (PK) |---<| ayah_id (PK)  |
| name_arabic |      | surah_id (FK) |
| name_english|      | ayah_number   |
+-----------+      | text_uthmani  |
                   +-----------+
                         |
                         |
+------------------+     |
|   Bookmark       |     |
+------------------+     |
| bookmark_id (PK) |     |
| surah_id (FK)    |-----'
| ayah_number      |
+------------------+

+---------------+      +------------------+
|  Translation  |      |  TranslatedAyah  |
+---------------+      +------------------+
| trans_id (PK) |---<| ayah_id (FK)     |
| name          |      | trans_id (FK)    |
| language      |      | text             |
+---------------+      +------------------+
      |                      ^
      |                      |
      '----------------------'
```

## Entities

### Surah
Represents a chapter of the Qur'an.
-   `surah_id`: Unique identifier for the surah (1-114).
-   `name_arabic`: The Arabic name of the surah.
-   `name_english`: The English transliteration of the surah's name.

### Ayah
Represents a verse within a surah.
-   `ayah_id`: Unique identifier for the ayah (globally unique).
-   `surah_id`: Foreign key referencing the `Surah` table.
-   `ayah_number`: The verse number within its surah.
-   `text_uthmani`: The original Arabic text of the ayah.

### Translation
Represents a specific translation of the Qur'an.
-   `trans_id`: Unique identifier for the translation.
-   `name`: The name of the translation (e.g., "Sahih International").
-   `language`: The language of the translation (e.g., "en", "id").

### TranslatedAyah
Represents the translated text of a specific ayah for a given translation.
-   `ayah_id`: Foreign key referencing the `Ayah` table.
-   `trans_id`: Foreign key referencing the `Translation` table.
-   `text`: The translated text of the ayah.

### Bookmark
Represents a user's bookmark.
-   `bookmark_id`: Unique identifier for the bookmark.
-   `surah_id`: Foreign key referencing the `Surah` table.
-   `ayah_number`: The number of the bookmarked ayah within the surah.

## Relationships

-   A `Surah` has one or more `Ayah`s.
-   An `Ayah` can have multiple `TranslatedAyah` records, one for each `Translation`.
-   A `Bookmark` points to a specific `Ayah` via its `surah_id` and `ayah_number`.

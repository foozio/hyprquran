# Track Spec: Fix Ayah Bookmarking Persistence

## Overview
A critical bug exists where Ayah bookmarks are not persisted correctly across application restarts or certain state changes. This track aims to identify the root cause, implement a robust fix, and ensure that user progress is saved reliably.

## Problem Description
- Bookmarks are lost or not saved to persistent storage.
- State transitions (e.g., theme switching, navigation) might be inadvertently clearing the bookmark state.
- The current JSON-based storage might have concurrency or serialization issues.

## Goals
- Identify the exact mechanism causing bookmark loss.
- Ensure bookmarks are saved to `~/.local/share/hyprquran/bookmarks.json` (or integrated into the main state file).
- Ensure bookmarks are loaded correctly on application startup.
- Prevent state changes from affecting persisted bookmarks negatively.

## Acceptance Criteria
- User can bookmark an Ayah.
- Bookmark persists after closing and reopening the application.
- Bookmark persists after switching themes or font sizes.
- Unit tests verify the save/load logic for bookmarks.
- Manual verification confirms the fix in the running application.

## Technical Constraints
- Follow XDG Base Directory Specification for storage.
- Adhere to the existing Rust/GTK4 architecture.
- Use TDD as specified in the workflow.

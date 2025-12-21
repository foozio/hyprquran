# Product Guidelines: HyprQur'an

## Visual Identity & UI Design
- **Standard Native Experience:** Adhere to the **Adwaita/GNOME Human Interface Guidelines (HIG)** to ensure HyprQur'an feels like a first-class citizen in GTK4/Wayland environments.
- **Content-First Layout:** The interface must prioritize the Arabic text and its translation, with UI elements remaining unobtrusive.
- **Typography:** Use the "Amiri Quran" font for Arabic text to ensure traditional and clear rendering. For translations and UI, use standard system sans-serif fonts.

## Tone & Style
- **Helpful & Mindful:** UI text should be clear, helpful, and approachable, maintaining a balance between modern software standards and the respect due to the content.
- **Error Handling:** Provide actionable and clear error messages (e.g., "Font not found" rather than a generic error) without being overly verbose.

## Multilingual Support & Layout
- **Comparative Reading:** The primary reading view should be optimized for side-by-side or interlinear display of Arabic text and translations.
- **Dynamic Sizing:** Layouts must gracefully handle varying lengths of text across different languages and font sizes.

## Interaction Model
- **Hybrid Accessibility:** Provide discoverable on-screen controls for all features while offering a highly optimized and comprehensive set of keyboard shortcuts.
- **Focus-Driven:** Navigation should feel "snappy" and responsive, minimizing the distance the user's focus must travel to perform common tasks.

## State & Data Management
- **Linux Standards:** Strictly follow **XDG Base Directory Specifications** for configuration (`~/.config/hyprquran`) and persistent data (`~/.local/share/hyprquran`).
- **Persistence:** Ensure that user preferences (theme, font size) and progress (bookmarks) are saved reliably and immediately upon change.

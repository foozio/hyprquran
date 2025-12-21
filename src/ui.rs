use crate::data;
use crate::state::{AppState, AyahRef};
use crate::storage;
use anyhow::Result;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk4 as gtk;
use gtk4::prelude::ActionMapExt;
use pango::{AttrList, AttrSize, AttrString};
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_ui_with_init(app: &gtk::Application, init: Option<AyahRef>) -> Result<()> {
    let state = Rc::new(RefCell::new(AppState::new()));
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("HyprQur'an")
        .default_width(960)
        .default_height(720)
        .build();
    window.set_opacity(0.92);

    let header = gtk::HeaderBar::builder()
        .title_widget(&gtk::Label::new(Some("HyprQur'an")))
        .build();
    let surah_combo = gtk::ComboBoxText::new();
    #[cfg(feature = "sqlite")]
    {
        if let Ok(conn) = crate::db::open() {
            if let Ok(list) = crate::db::get_surah_list(&conn) {
                for (_, ar, en) in list {
                    surah_combo.append_text(&format!("{} — {}", en, ar));
                }
            }
        }
    }
    #[cfg(not(feature = "sqlite"))]
    {
        for s in &state.borrow().surahs {
            surah_combo.append_text(&format!("{} — {}", s.name_en, s.name_ar));
        }
    }
    surah_combo.set_active(Some(0));
    let ayah_spin = gtk::SpinButton::with_range(1.0, 7.0, 1.0);
    ayah_spin.set_value(1.0);
    let search_entry = gtk::SearchEntry::new();
    let toggle_translation = gtk::ToggleButton::with_label("Translation");
    let dark_toggle = gtk::ToggleButton::with_label("Dark");
    let lang_combo = gtk::ComboBoxText::new();

    // Populate language combo with available translations
    #[cfg(feature = "sqlite")]
    {
        if let Ok(conn) = crate::db::open() {
            if let Ok(translations) = crate::db::get_available_translations(&conn) {
                for (lang_code, lang_name) in translations {
                    lang_combo.append(
                        Some(&lang_code),
                        &format!("{} ({})", lang_name, lang_code.to_uppercase()),
                    );
                }
            }
        }
    }
    #[cfg(not(feature = "sqlite"))]
    {
        lang_combo.append(Some("en"), "EN");
        lang_combo.append(Some("id"), "ID");
    }

    lang_combo.set_active(Some(0));
    header.pack_start(&surah_combo);
    header.pack_start(&ayah_spin);
    header.pack_start(&search_entry);
    header.pack_end(&lang_combo);
    header.pack_end(&toggle_translation);
    header.pack_end(&dark_toggle);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    let arabic_area = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .build();
    let translation_area = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .build();
    let arabic_label = gtk::Label::new(None);
    let translation_label = gtk::Label::new(None);
    arabic_label.set_wrap(true);
    translation_label.set_wrap(true);
    arabic_label.set_xalign(0.0);
    translation_label.set_xalign(0.0);
    arabic_label.set_direction(gtk::TextDirection::Rtl);
    translation_label.set_direction(gtk::TextDirection::Ltr);
    arabic_label.set_margin_top(12);
    arabic_label.set_margin_bottom(12);
    arabic_label.set_margin_start(12);
    arabic_label.set_margin_end(12);
    translation_label.set_margin_top(8);
    translation_label.set_margin_bottom(12);
    translation_label.set_margin_start(12);
    translation_label.set_margin_end(12);
    let attrs = arabic_attrs(20);
    arabic_label.set_attributes(Some(&attrs));
    arabic_area.set_child(Some(&arabic_label));
    translation_area.set_child(Some(&translation_label));
    vbox.append(&arabic_area);
    vbox.append(&translation_area);

    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    root.append(&header);
    root.append(&vbox);
    window.set_child(Some(&root));

    let bookmarks_button = gtk::MenuButton::new();
    bookmarks_button.set_label("Bookmarks");
    let popover = gtk::Popover::new();
    let list = gtk::ListBox::new();
    popover.set_child(Some(&list));
    bookmarks_button.set_popover(Some(&popover));
    header.pack_end(&bookmarks_button);

    // Create a separate function that can be called from mutable borrow contexts
    let update_display = {
        let arabic_label = arabic_label.clone();
        let translation_label = translation_label.clone();
        let state = state.clone();
        move || {
            let st = state.borrow().clone();
            if !st.current_ayat.is_empty() {
                let idx = (st.current.ayah_index.max(1) as usize).min(st.current_ayat.len());
                let arabic = st.current_ayat[idx - 1].clone();
                arabic_label.set_text(&arabic);
                arabic_label.set_attributes(Some(&arabic_attrs(st.font_size)));
            }
            if let Some(lang) = &st.translation_lang {
                if let Ok(tf) = data::load_translation(lang, st.current.surah_id) {
                    let idx = st.current.ayah_index;
                    if let Some(e) = tf.entries.iter().find(|e| e.ayah == idx) {
                        translation_label.set_text(&e.text);
                    } else {
                        translation_label.set_text("");
                    }
                } else {
                    translation_label.set_text("");
                }
            } else {
                translation_label.set_text("");
            }
        }
    };

    let refresh = {
        let update_display = update_display.clone();
        move || {
            update_display();
        }
    };

    surah_combo.connect_changed(
        clone!(@strong state, @strong ayah_spin, @strong update_display => move |c| {
            if let Some(idx) = c.active() {
                let surah = {
                    let st = state.borrow();
                    st.surahs.get(idx as usize).cloned()
                };

                if let Some(s) = surah {
                    let mut ayat = Vec::new();
                    if let Ok(sf) = crate::data::load_surah_text(s.id) {
                        ayat = sf.ayat;
                    }

                    let current = AyahRef { surah_id: s.id, ayah_index: 1 };
                    
                    {
                        let mut st = state.borrow_mut();
                        st.current = current;
                        st.set_ayat(ayat.clone());
                    }

                    ayah_spin.set_range(1.0, s.ayah_count as f64);
                    ayah_spin.set_value(1.0);
                    
                    update_display();
                }
            }
        }),
    );

    ayah_spin.connect_value_changed(clone!(@strong state, @strong update_display => move |sp| {
        {
            let mut st = state.borrow_mut();
            st.current.ayah_index = sp.value() as u16;
        }
        update_display();
    }));

    toggle_translation.connect_toggled(clone!(@strong state, @strong update_display => move |t| {
        {
            let active = t.is_active();
            let mut st = state.borrow_mut();
            st.translation_lang = if active { Some("en".to_string()) } else { None };
            persist(&st);
        }
        update_display();
    }));

    lang_combo.connect_changed(
        clone!(@strong state, @strong toggle_translation, @strong update_display => move |c| {
            {
                let mut st = state.borrow_mut();
                let lang = c.active_id().map(|id| id.to_string());
                st.translation_lang = if toggle_translation.is_active() { lang } else { None };
                persist(&st);
            }
            update_display();
        }),
    );

    search_entry.connect_changed(clone!(@strong state => move |se| {
        let q = se.text().to_string();
        let mut st = state.borrow_mut();
        st.run_search(&q);
    }));

    dark_toggle.connect_toggled(clone!(@strong state => move |t| {
        let mut st = state.borrow_mut();
        st.prefer_dark = t.is_active();
        if let Some(settings) = gtk::Settings::default() {
            settings.set_gtk_application_prefer_dark_theme(st.prefer_dark);
        }
        persist(&st);
    }));

    add_shortcuts(
        app,
        state.clone(),
        search_entry.clone(),
        update_display,
        list.as_ref(),
    );

    if let Ok(s) = data::load_surah_text(1) {
        let mut st = state.borrow_mut();
        st.set_ayat(s.ayat);
    }
    if let Some(p) = storage::load() {
        // Sync Surah Combo first to setup context (text, range)
        if p.last.surah_id > 0 {
             surah_combo.set_active(Some((p.last.surah_id as u32).saturating_sub(1)));
        }

        {
            let mut st = state.borrow_mut();
            st.current = p.last;
            st.translation_lang = p.translation_lang.clone();
            st.prefer_dark = p.prefer_dark;
            st.bookmarks = p.bookmarks.clone();
            st.font_size = p.font_size;
        }
        if let Some(settings) = gtk::Settings::default() {
            let dark = p.prefer_dark;
            settings.set_gtk_application_prefer_dark_theme(dark);
        }
        dark_toggle.set_active(p.prefer_dark);
        if let Some(lang) = &p.translation_lang {
            toggle_translation.set_active(true);
            // Set the active ID in the combo box
            lang_combo.set_active_id(Some(lang));
        };
        ayah_spin.set_value(p.last.ayah_index as f64);
    }
    if let Some(i) = init {
        {
            let mut st = state.borrow_mut();
            st.current = i;
        }
        ayah_spin.set_value(i.ayah_index as f64);
    }
    refresh();
    window.present();
    Ok(())
}

pub fn show_error(app: &gtk::Application, msg: String) -> Result<()> {
    let win = gtk::ApplicationWindow::builder()
        .application(app)
        .title("HyprQur'an Error")
        .default_width(480)
        .default_height(180)
        .build();
    let boxv = gtk::Box::new(gtk::Orientation::Vertical, 12);
    let lbl = gtk::Label::new(Some(&msg));
    let btn = gtk::Button::with_label("Close");
    boxv.append(&lbl);
    boxv.append(&btn);
    win.set_child(Some(&boxv));
    let win2 = win.clone();
    btn.connect_clicked(move |_| { win2.close(); });
    win.present();
    Ok(())
}

fn arabic_attrs(size: i32) -> AttrList {
    let attrs = AttrList::new();
    let family = crate::fonts::prefer_font_family();
    attrs.insert(AttrString::new_family(&family));
    attrs.insert(AttrSize::new_size_absolute(size * pango::SCALE));
    attrs
}

fn add_shortcuts(
    app: &gtk::Application,
    state: Rc<RefCell<AppState>>,
    search_entry: gtk::SearchEntry,
    update_display: impl Fn() + 'static + Clone,
    list: &gtk::ListBox,
) {
    let next_ayah = gio::SimpleAction::new("next-ayah", None);
    next_ayah.connect_activate(
        clone!(@strong state, @strong update_display => move |_, _| {
            {
                let mut st = state.borrow_mut();
                let max_ayahs = st.surahs.get(st.current.surah_id as usize - 1)
                    .map(|s| s.ayah_count)
                    .unwrap_or(st.current.ayah_index);
                st.current.ayah_index = (st.current.ayah_index + 1).min(max_ayahs);
                persist(&st);
            }
            update_display();
        }),
    );
    app.add_action(&next_ayah);
    app.set_accels_for_action("app.next-ayah", &["<Alt>Right"]);

    let prev_ayah = gio::SimpleAction::new("prev-ayah", None);
    prev_ayah.connect_activate(
        clone!(@strong state, @strong update_display => move |_, _| {
            {
                let mut st = state.borrow_mut();
                st.current.ayah_index = st.current.ayah_index.saturating_sub(1).max(1);
                persist(&st);
            }
            update_display();
        }),
    );
    app.add_action(&prev_ayah);
    app.set_accels_for_action("app.prev-ayah", &["<Alt>Left"]);

    let toggle_t = gio::SimpleAction::new("toggle-translation", None);
    toggle_t.connect_activate(clone!(@strong state, @strong update_display => move |_, _| {
        {
            let mut st = state.borrow_mut();
            st.translation_lang = if st.translation_lang.is_some() { None } else { Some("en".to_string()) };
            persist(&st);
        }
        update_display();
    }));
    app.add_action(&toggle_t);
    app.set_accels_for_action("app.toggle-translation", &["T"]);

    let focus_search = gio::SimpleAction::new("focus-search", None);
    focus_search.connect_activate(move |_, _| {
        search_entry.grab_focus();
    });
    app.add_action(&focus_search);
    app.set_accels_for_action("app.focus-search", &["<Control>F"]);

    let next_result = gtk4::gio::SimpleAction::new("next-result", None);
    next_result.connect_activate(
        clone!(@strong state, @strong update_display => move |_, _| {
            {
                let mut st = state.borrow_mut();
                if let Some(&i) = st.search_results.iter().find(|&&i| i > st.current.ayah_index) {
                    st.current.ayah_index = i;
                    persist(&st);
                }
            }
            update_display();
        }),
    );
    app.add_action(&next_result);
    app.set_accels_for_action("app.next-result", &["Return"]);

    let prev_result = gio::SimpleAction::new("prev-result", None);
    prev_result.connect_activate(clone!(@strong state, @strong update_display => move |_, _| {
        {
            let mut st = state.borrow_mut();
            if let Some(&i) = st.search_results.iter().rev().find(|&&i| i < st.current.ayah_index) {
                st.current.ayah_index = i;
                persist(&st);
            }
        }
        update_display();
    }));
    app.add_action(&prev_result);
    app.set_accels_for_action("app.prev-result", &["<Shift>Return"]);

    let bookmark = gio::SimpleAction::new("bookmark", None);
    bookmark.connect_activate(clone!(@strong state, @strong list => move |_, _| {
        let current = {
            let mut st = state.borrow_mut();
            let c = st.current;
            st.add_bookmark(c);
            persist(&st);
            c
        };
        // Append to UI list
        let row = gtk::ListBoxRow::new();
        let label = gtk::Label::new(Some(&format!("{}:{}", current.surah_id, current.ayah_index)));
        row.set_child(Some(&label));
        list.append(&row);
    }));
    app.add_action(&bookmark);
    app.set_accels_for_action("app.bookmark", &["B"]);

    let settings_action = gio::SimpleAction::new("settings", None);
    settings_action.connect_activate(clone!(@strong state, @strong update_display => move |_, _| {
        let dlg = gtk::Dialog::builder().title("Settings").modal(true).build();
        let content = gtk::Box::new(gtk::Orientation::Vertical, 8);
        content.set_margin_top(12);
        content.set_margin_bottom(12);
        content.set_margin_start(12);
        content.set_margin_end(12);
        let font_spin = gtk::SpinButton::with_range(14.0, 48.0, 1.0);
        {
            let st = state.borrow();
            font_spin.set_value(st.font_size as f64);
        }
        content.append(&gtk::Label::new(Some("Font size")));
        content.append(&font_spin);
        dlg.set_child(Some(&content));
        
        // Note: Dialog value changed doesn't take 'refresh' or 'update_display' directly cleanly
        // if we are inside a closure that captured it.
        // We need to clone update_display for this inner closure.
        let update_display = update_display.clone();
        let state = state.clone();
        
        font_spin.connect_value_changed(move |sp| {
            let mut st = state.borrow_mut();
            st.font_size = sp.value() as i32;
            persist(&st);
            update_display();
        });
        
        dlg.add_button("Close", gtk::ResponseType::Close);
        dlg.connect_response(|d, _| d.close());
        dlg.present();
    }));
    app.add_action(&settings_action);
    app.set_accels_for_action("app.settings", &["<Control>comma"]);

    // Load bookmarks into list
    if let Some(p) = storage::load() {
        for b in p.bookmarks {
            let row = gtk::ListBoxRow::new();
            let label = gtk::Label::new(Some(&format!("{}:{}", b.surah_id, b.ayah_index)));
            row.set_child(Some(&label));
            list.append(&row);
        }
    }

    list.connect_row_activated(clone!(@strong state, @strong update_display => move |_, row| {
        if let Some(child) = row.child() {
            if let Ok(lbl) = child.downcast::<gtk::Label>() {
                let text = lbl.text();
                let parts: Vec<_> = text.split(':').collect();
                if parts.len() == 2 { if let (Ok(s), Ok(a)) = (parts[0].parse::<u16>(), parts[1].parse::<u16>()) {
                    {
                        let mut st = state.borrow_mut();
                        st.current = AyahRef { surah_id: s, ayah_index: a };
                        persist(&st);
                    }
                    update_display();
                }}
            }
        }

    }));
}

fn persist(st: &crate::state::AppState) {
    let mut p = storage::load().unwrap_or_default();
    p.last = st.current;
    p.translation_lang = st.translation_lang.clone();
    p.prefer_dark = st.prefer_dark;
    p.font_size = st.font_size;
    p.bookmarks = st.bookmarks.clone();
    let _ = storage::save(&p);
}
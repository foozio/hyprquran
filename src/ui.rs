#![cfg(feature = "gui")]
use crate::data;
use crate::state::{AppState, AyahRef};
use crate::storage;
use anyhow::Result;
use gio::prelude::*;
use glib::clone;
use gtk4 as gtk;
use gtk::prelude::*;
use pango::{AttrInt, AttrList, AttrString};
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

    let header = gtk::HeaderBar::builder().title_widget(&gtk::Label::new(Some("HyprQur'an"))).build();
    let surah_combo = gtk::ComboBoxText::new();
    // populate surahs
    for s in &state.borrow().surahs {
        surah_combo.append_text(&format!("{} — {}", s.name_en, s.name_ar));
    }
    surah_combo.set_active(Some(0));
    let ayah_spin = gtk::SpinButton::with_range(1.0, 7.0, 1.0);
    ayah_spin.set_value(1.0);
    let search_entry = gtk::SearchEntry::new();
    let toggle_translation = gtk::ToggleButton::with_label("Translation");
    let dark_toggle = gtk::ToggleButton::with_label("Dark");
    let lang_combo = gtk::ComboBoxText::new();
    lang_combo.append_text("EN");
    lang_combo.append_text("ID");
    lang_combo.set_active(Some(0));
    header.pack_start(&surah_combo);
    header.pack_start(&ayah_spin);
    header.pack_start(&search_entry);
    header.pack_end(&lang_combo);
    header.pack_end(&toggle_translation);
    header.pack_end(&dark_toggle);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    let arabic_area = gtk::ScrolledWindow::builder().hexpand(true).vexpand(true).build();
    let translation_area = gtk::ScrolledWindow::builder().hexpand(true).vexpand(true).build();
    let arabic_label = gtk::Label::new(None);
    let translation_label = gtk::Label::new(None);
    arabic_label.set_wrap(true);
    translation_label.set_wrap(true);
    arabic_label.set_xalign(0.0);
    translation_label.set_xalign(0.0);
    arabic_label.set_direction(gtk::TextDirection::Rtl);
    translation_label.set_direction(gtk::TextDirection::Ltr);
    let attrs = arabic_attrs();
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

    let refresh = {
        let state = state.clone();
        let arabic_label = arabic_label.clone();
        let translation_label = translation_label.clone();
        move || {
            let st = state.borrow().clone();
            if !st.current_ayat.is_empty() {
                let idx = (st.current.ayah_index.max(1) as usize).min(st.current_ayat.len());
                let arabic = st.current_ayat[idx - 1].clone();
                arabic_label.set_text(&arabic);
            }
            if let Some(lang) = st.translation_lang.clone() {
                if let Ok(tf) = data::load_translation(&lang, st.current.surah_id) {
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

    surah_combo.connect_changed(clone!(@strong state, @strong ayah_spin, @strong refresh => move |c| {
        if let Some(idx) = c.active() {
            let mut st = state.borrow_mut();
            let surah = st.surahs.get(idx as usize).cloned();
            if let Some(s) = surah {
                st.current = AyahRef { surah_id: s.id, ayah_index: 1 };
                ayah_spin.set_range(1.0, s.ayah_count as f64);
                ayah_spin.set_value(1.0);
                if let Ok(sf) = crate::data::load_surah_text(s.id) {
                    st.set_ayat(sf.ayat);
                } else {
                    st.set_ayat(Vec::new());
                }
                refresh();
            }
        }
    }));

    ayah_spin.connect_value_changed(clone!(@strong state, @strong refresh => move |sp| {
        let mut st = state.borrow_mut();
        st.current.ayah_index = sp.value() as u16;
        refresh();
    }));

    toggle_translation.connect_toggled(clone!(@strong state, @strong refresh => move |t| {
        let active = t.is_active();
        let mut st = state.borrow_mut();
        st.translation_lang = if active { Some("en".to_string()) } else { None };
        persist(&st);
        refresh();
    }));

    lang_combo.connect_changed(clone!(@strong state, @strong toggle_translation, @strong refresh => move |c| {
        let mut st = state.borrow_mut();
        let lang = match c.active_text().map(|s| s.to_string()) {
            Some(s) if s == "EN" => Some("en".to_string()),
            Some(s) if s == "ID" => Some("id".to_string()),
            _ => None,
        };
        st.translation_lang = if toggle_translation.is_active() { lang } else { None };
        persist(&st);
        refresh();
    }));

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

    add_shortcuts(app, state.clone(), search_entry.clone(), refresh.clone());

    if let Ok(s) = data::load_surah_text(1) {
        let mut st = state.borrow_mut();
        st.set_ayat(s.ayat);
    }
    if let Some(p) = storage::load() {
        let mut st = state.borrow_mut();
        st.current = p.last;
        st.translation_lang = p.translation_lang;
        st.prefer_dark = p.prefer_dark;
        if let Some(settings) = gtk::Settings::default() { settings.set_gtk_application_prefer_dark_theme(st.prefer_dark); }
        dark_toggle.set_active(st.prefer_dark);
        if let Some(lang) = &st.translation_lang { toggle_translation.set_active(true); if lang == "id" { lang_combo.set_active(Some(1)); } else { lang_combo.set_active(Some(0)); } };
        ayah_spin.set_value(st.current.ayah_index as f64);
    }
    if let Some(i) = init {
        let mut st = state.borrow_mut();
        st.current = i;
        ayah_spin.set_value(st.current.ayah_index as f64);
    }
    refresh();
    window.present();
    Ok(())
}

fn arabic_attrs() -> AttrList {
    let attrs = AttrList::new();
    attrs.insert(AttrString::new_family("Amiri Quran"));
    attrs.insert(AttrInt::new_size(20 * pango::SCALE));
    attrs
}

fn add_shortcuts(app: &gtk::Application, state: Rc<RefCell<AppState>>, search_entry: gtk::SearchEntry, refresh: impl Fn() + 'static) {
    let next_ayah = gio::SimpleAction::new("next-ayah", None);
    next_ayah.connect_activate(clone!(@strong state, @strong refresh => move |_, _| {
        let mut st = state.borrow_mut();
        st.current.ayah_index = (st.current.ayah_index + 1).min(7);
        persist(&st);
        refresh();
    }));
    app.add_action(&next_ayah);
    app.set_accels_for_action("app.next-ayah", &["<Alt>Right"]);

    let prev_ayah = gio::SimpleAction::new("prev-ayah", None);
    prev_ayah.connect_activate(clone!(@strong state, @strong refresh => move |_, _| {
        let mut st = state.borrow_mut();
        st.current.ayah_index = st.current.ayah_index.saturating_sub(1).max(1);
        persist(&st);
        refresh();
    }));
    app.add_action(&prev_ayah);
    app.set_accels_for_action("app.prev-ayah", &["<Alt>Left"]);

    let toggle_t = gio::SimpleAction::new("toggle-translation", None);
    toggle_t.connect_activate(clone!(@strong state, @strong refresh => move |_, _| {
        let mut st = state.borrow_mut();
        st.translation_lang = if st.translation_lang.is_some() { None } else { Some("en".to_string()) };
        persist(&st);
        refresh();
    }));
    app.add_action(&toggle_t);
    app.set_accels_for_action("app.toggle-translation", &["T"]);

    let focus_search = gio::SimpleAction::new("focus-search", None);
    focus_search.connect_activate(move |_, _| {
        search_entry.grab_focus();
    });
    app.add_action(&focus_search);
    app.set_accels_for_action("app.focus-search", &["<Control>F"]);

    let next_result = gio::SimpleAction::new("next-result", None);
    next_result.connect_activate(clone!(@strong state, @strong refresh => move |_, _| {
        let mut st = state.borrow_mut();
        if let Some(&i) = st.search_results.iter().find(|&&i| i > st.current.ayah_index) {
            st.current.ayah_index = i;
            persist(&st);
            refresh();
        }
    }));
    app.add_action(&next_result);
    app.set_accels_for_action("app.next-result", &["Return"]);

    let prev_result = gio::SimpleAction::new("prev-result", None);
    prev_result.connect_activate(clone!(@strong state, @strong refresh => move |_, _| {
        let mut st = state.borrow_mut();
        if let Some(&i) = st.search_results.iter().rev().find(|&&i| i < st.current.ayah_index) {
            st.current.ayah_index = i;
            persist(&st);
            refresh();
        }
    }));
    app.add_action(&prev_result);
    app.set_accels_for_action("app.prev-result", &["<Shift>Return"]);

    let bookmark = gio::SimpleAction::new("bookmark", None);
    bookmark.connect_activate(clone!(@strong state, @strong list => move |_, _| {
        let st = state.borrow();
        let _ = storage::add_bookmark(st.current.clone());
        // Append to UI list
        let row = gtk::ListBoxRow::new();
        let label = gtk::Label::new(Some(&format!("{}:{}", st.current.surah_id, st.current.ayah_index)));
        row.set_child(Some(&label));
        list.append(&row);
    }));
    app.add_action(&bookmark);
    app.set_accels_for_action("app.bookmark", &["B"]);

    // Load bookmarks into list
    if let Some(p) = storage::load() {
        for b in p.bookmarks {
            let row = gtk::ListBoxRow::new();
            let label = gtk::Label::new(Some(&format!("{}:{}", b.surah_id, b.ayah_index)));
            row.set_child(Some(&label));
            list.append(&row);
        }
    }

    list.connect_row_activated(clone!(@strong state, @strong refresh => move |_, row| {
        if let Some(child) = row.child() {
            if let Ok(lbl) = child.downcast::<gtk::Label>() {
                if let Some(text) = lbl.text() {
                    let parts: Vec<_> = text.split(':').collect();
                    if parts.len() == 2 { if let (Ok(s), Ok(a)) = (parts[0].parse::<u16>(), parts[1].parse::<u16>()) {
                        let mut st = state.borrow_mut();
                        st.current = AyahRef { surah_id: s, ayah_index: a };
                        persist(&st);
                        refresh();
                    }}
                }
            }
        }
    }));
}

fn persist(st: &crate::state::AppState) {
    let mut p = storage::load().unwrap_or_default();
    p.last = st.current.clone();
    p.translation_lang = st.translation_lang.clone();
    p.prefer_dark = st.prefer_dark;
    let _ = storage::save(&p);
}

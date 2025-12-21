use anyhow::Result;
use gtk::prelude::*;
use gtk4 as gtk;

const APP_ID: &str = "org.hyprquran.app";

pub fn run(init: Option<crate::state::AyahRef>) -> Result<()> {
    let _ = crate::fonts::register_bundled_fonts();
    let app = gtk::Application::new(Some(APP_ID), Default::default());
    app.connect_activate(move |a| {
        if let Err(e) = crate::ui::build_ui_with_init(a, init.clone()) {
            let _ = crate::ui::show_error(a, format!("{}", e));
        }
    });
    app.run();
    Ok(())
}
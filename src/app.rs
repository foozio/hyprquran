use anyhow::Result;
use gtk4 as gtk;
use gtk::prelude::*;

const APP_ID: &str = "org.hyprquran.app";

pub fn run(init: Option<crate::state::AyahRef>) -> Result<()> {
    let app = gtk::Application::new(Some(APP_ID), Default::default());
    app.connect_activate(move |a| {
        if let Err(e) = crate::ui::build_ui_with_init(a, init.clone()) {
            eprintln!("{}", e);
        }
    });
    app.run();
    Ok(())
}

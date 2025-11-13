mod app;
mod ui;
mod data;
mod state;
mod logging;

use anyhow::Result;
use std::env;
use state::AyahRef;

fn main() -> Result<()> {
    logging::init();
    let mut surah: Option<u16> = None;
    let mut ayah: Option<u16> = None;
    let mut args = env::args().skip(1).peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--surah" => {
                if let Some(v) = args.next() { surah = v.parse::<u16>().ok(); }
            }
            "--ayah" => {
                if let Some(v) = args.next() { ayah = v.parse::<u16>().ok(); }
            }
            _ => {}
        }
    }
    let init = match (surah, ayah) {
        (Some(s), Some(a)) => Some(AyahRef { surah_id: s, ayah_index: a }),
        _ => None,
    };
    app::run(init)
}

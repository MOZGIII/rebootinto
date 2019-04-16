#[macro_use]
extern crate failure_derive;

use failure::Error;
use rebootinto_core as core;

use std::io;

use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;

mod error;
mod item;
mod ui;

use error::NoLoadOptions;
use item::Item;
use ui::BootNextSelectorUI;

type Result<T> = std::result::Result<T, Error>;

fn main() {
    if let Err(err) = run() {
        match std::env::var("PANIC_ON_ERROR") {
            Ok(ref val) if val == "true" => panic!("Error: {}", err),
            _ => {}
        }

        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut backend = core::Backend::init()?;
    let load_options = backend
        .load_options()
        .collect::<Result<Vec<core::LoadOption>>>()?;

    if load_options.is_empty() {
        return Err(NoLoadOptions.into());
    }

    let reboot_into = {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        let mut events = io::stdin().events();

        let mut items: Vec<Item> = load_options.into_iter().map(|e| Item::from(e)).collect();
        let mut ui = BootNextSelectorUI::new(&mut terminal, &mut events, &items, 0);

        let selected_item_idx = ui.run()?;
        selected_item_idx.map(|e| items.swap_remove(e).into_inner())
    };

    if let Some(reboot_into) = reboot_into {
        println!("Rebooting into: {}", &reboot_into);
        backend.reboot_into(reboot_into.number)?;
    } else {
        println!("Reboot cancelled");
    }
    Ok(())
}

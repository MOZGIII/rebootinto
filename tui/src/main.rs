#[macro_use]
extern crate failure_derive;

use failure::Error;
use rebootinto_core as core;

use tui::Terminal;

mod error;
mod event;
mod input_backend;
mod item;
mod terminal_backend;
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
        let backend = terminal_backend::create_terminal_backend()?;
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        let mut input = input_backend::create_input_backend();

        let mut items: Vec<Item> = load_options.into_iter().map(|e| Item::from(e)).collect();
        let mut ui = BootNextSelectorUI::new(&mut terminal, &mut input, &items, 0);

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

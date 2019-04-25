#![windows_subsystem = "windows"]

use failure::Error;
use rebootinto_core as core;

use iui::controls::{Button, Label, VerticalBox};
use iui::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

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

    let backend = Rc::new(RefCell::new(backend));

    let ui = UI::init()?;

    let mut win = Window::new(&ui, "Reboot Into", 200, 200, WindowType::NoMenubar);

    let mut vbox_rebooting = VerticalBox::new(&ui);
    let mut lbl_reboot = Label::new(&ui, "Rebooting...");
    lbl_reboot.show(&ui);
    vbox_rebooting.append(&ui, lbl_reboot.clone(), LayoutStrategy::Stretchy);

    let mut vbox_buttons = VerticalBox::new(&ui);
    vbox_buttons.set_padded(&ui, true);

    for load_option in load_options {
        let mut button = Button::new(&ui, &load_option.description);
        button.on_clicked(&ui, {
            let backend = backend.clone();
            let ui = ui.clone();
            let mut win = win.clone();
            let mut lbl_reboot = lbl_reboot.clone();
            let vbox_rebooting = vbox_rebooting.clone();
            move |_btn| {
                let load_option = load_option.clone();
                let mut backend = backend.borrow_mut();
                if let Err(err) = backend.reboot_into(load_option.number) {
                    win.modal_err(&ui, "Reboot error", &format!("Error: {}", err));
                } else {
                    let text = format!("Rebooting into:\n{}", load_option.description);
                    &lbl_reboot.set_text(&ui, &text);
                    win.set_child(&ui, vbox_rebooting.clone());
                }
            }
        });
        vbox_buttons.append(&ui, button, LayoutStrategy::Compact);
    }

    win.set_child(&ui, vbox_buttons);

    win.show(&ui);
    ui.main();

    Ok(())
}

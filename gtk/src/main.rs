//! A GTK-based GUI app for rebootinto.

#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::{clone, gformat};
use rebootinto_core as core;

use gtk::{prelude::*, Button, Orientation, PolicyType, ScrolledWindow};
use gtk::{Application, ApplicationWindow};

/// The Application ID.
const APP_ID: &str = "mozgiii.Rebootinto";

fn main() {
    if let Err(err) = run() {
        match std::env::var("PANIC_ON_ERROR") {
            Ok(ref val) if val == "true" => panic!("Error: {err}"),
            _ => {}
        }

        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

/// Run the app and return the error.
fn run() -> Result<(), anyhow::Error> {
    let mut backend = core::Backend::init()?;
    let load_options = backend
        .load_options()?
        .collect::<Result<Vec<core::LoadOption>, core::LoadOptionError>>()?;

    let reboot_into = Rc::new(RefCell::new(None));

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(clone!(@strong reboot_into => move |app| {
        let buttons_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .homogeneous(true)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(6)
            .margin_end(6)
            .build();

        for load_option in &load_options {
            let button = Button::builder()
                .label(gformat!("{}", &load_option))
                .margin_top(6)
                .margin_bottom(6)
                .margin_start(6)
                .margin_end(6)
                .build();

            button.connect_clicked(clone!(@strong app, @strong reboot_into, @strong load_option => move |_| {
                *reboot_into.borrow_mut() = Some(load_option.clone());
                app.quit();
            }));

            buttons_box.append(&button);
        }

        let scrolled_window = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .propagate_natural_height(true)
            .child(&buttons_box)
            .build();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Rebootinto")
            .default_width(350)
            .default_height(200)
            .child(&scrolled_window)
            .build();
        window.show();
    }));

    let exit_code = app.run();
    if exit_code.value() != 0 {
        anyhow::bail!("error exit: {exit_code:?}")
    }

    drop(app);

    if let Some(ref reboot_into) = *reboot_into.borrow() {
        println!("Rebooting into: {}", &reboot_into);
        backend.reboot_into(reboot_into.number)?;
    } else {
        println!("Reboot cancelled");
    }

    Ok(())
}

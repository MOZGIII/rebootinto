//! An iced-based GUI app for rebootinto.

#![windows_subsystem = "windows"]

use rebootinto_core as core;

use iced::{Application, Settings};

mod app;

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

/// Compute an estimate of the height of the window to accomodate the contents
/// based on the amount of items, taking the padding and spacing into account.
fn estimate_window_height(items: usize) -> u32 {
    let padding = (app::LAYOUT_PADDING as u32) * 2;

    let displayed_items_estimate = items.try_into().unwrap_or(10);

    let spacing = if displayed_items_estimate > 0 {
        (displayed_items_estimate - 1) * (app::LAYOUT_SPACING as u32)
    } else {
        0
    };

    let assumed_button_height = 30;
    let content = displayed_items_estimate * assumed_button_height;

    padding + spacing + content
}

/// Run the app and return the error.
fn run() -> Result<(), anyhow::Error> {
    let mut backend = core::Backend::init()?;
    let load_options = backend
        .load_options()
        .collect::<Result<Vec<core::LoadOption>, core::LoadOptionError>>()?;

    let size = (350, estimate_window_height(load_options.len()));

    let mut settings = Settings::with_flags(app::Init {
        backend,
        load_options,
    });
    settings.window.size = size;
    app::App::run(settings)?;
    Ok(())
}

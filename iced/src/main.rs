//! An iced-based GUI app for rebootinto.

#![windows_subsystem = "windows"]

use rebootinto_core as core;

use iced::{Application, Settings};

mod app;

/// The application icon, shown by the window manager.
const ICON_PNG: &[u8] = include_bytes!("../../assets/icons/hicolor/128x128/apps/rebootinto.png");

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

    let assumed_button_height = 31;
    let content = displayed_items_estimate * assumed_button_height;

    padding + spacing + content
}

/// Decodes the application icon into a window icon.
fn window_icon() -> Result<iced::window::Icon, anyhow::Error> {
    let mut reader = png::Decoder::new(ICON_PNG).read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    anyhow::ensure!(
        info.color_type == png::ColorType::Rgba && info.bit_depth == png::BitDepth::Eight,
        "unexpected icon format: {:?} {:?}",
        info.color_type,
        info.bit_depth,
    );
    buf.truncate(info.buffer_size());
    Ok(iced::window::icon::from_rgba(buf, info.width, info.height)?)
}

/// Run the app and return the error.
fn run() -> Result<(), anyhow::Error> {
    let mut backend = core::Backend::init()?;
    let load_options = backend
        .load_options()?
        .collect::<Result<Vec<core::LoadOption>, core::LoadOptionError>>()?;

    let size = (350, estimate_window_height(load_options.len()));

    let mut settings = Settings::with_flags(app::Init {
        backend,
        load_options,
    });
    settings.window.size = size;
    match window_icon() {
        Ok(icon) => settings.window.icon = Some(icon),
        Err(err) => eprintln!("Unable to load the window icon: {err}"),
    }
    app::App::run(settings)?;
    Ok(())
}

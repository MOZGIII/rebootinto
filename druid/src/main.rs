#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![windows_subsystem = "windows"]

use std::convert::TryInto;

use failure::Error;
use rebootinto_core as core;

use druid::{
    im::Vector,
    widget::{Button, List, Scroll},
    UnitPoint,
};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

const LIST_ITEM_HEIGHT: f64 = 48.0;
const LIST_ITEM_PADDING: f64 = 8.0;
const LIST_PADDING: f64 = 8.0;
const WINDOW_TITLE: LocalizedString<AppData> = LocalizedString::new("Rebootinto");

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

fn estimate_window_height(items: usize) -> f64 {
    let items = items.try_into().unwrap_or(10);

    let list_content = LIST_ITEM_HEIGHT * items as f64;
    let list_padding = LIST_PADDING * 2.0;

    list_content + list_padding
}

fn run() -> Result<()> {
    let mut backend = core::Backend::init()?;
    let load_options = backend
        .load_options()
        .map(|result| result.map(Into::into))
        .collect::<Result<Vector<_>>>()?;

    let size = (350.0, estimate_window_height(load_options.len()));

    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size(size);

    let initial_state = AppData { load_options };
    AppLauncher::with_window(main_window).launch(initial_state)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct LoadOptionData(pub core::LoadOption);

impl Data for LoadOptionData {
    fn same(&self, other: &Self) -> bool {
        self.0.description.same(&other.0.description) && self.0.number.same(&other.0.number)
    }
}

impl From<core::LoadOption> for LoadOptionData {
    fn from(val: core::LoadOption) -> Self {
        Self(val)
    }
}

#[derive(Clone, Data, Lens)]
struct AppData {
    load_options: Vector<LoadOptionData>,
}

fn build_root_widget() -> impl Widget<AppData> {
    const BUTTON_HEIGHT: f64 = LIST_ITEM_HEIGHT - LIST_ITEM_PADDING * 2.0;

    Scroll::new(
        List::new(|| {
            Button::new(|load_option: &LoadOptionData, _env: &_| load_option.0.description.clone())
                .expand()
                .height(BUTTON_HEIGHT)
                .align_vertical(UnitPoint::CENTER)
                .padding(LIST_ITEM_PADDING)
        })
        .padding(LIST_PADDING),
    )
    .vertical()
    .lens(AppData::load_options)
}

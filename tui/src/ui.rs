//! The UI implementation.

use crate::event::Event;
use crate::input_backend::InputBackend;
use tui::style::{Modifier, Style};
use tui::widgets::List;
use tui::Terminal;

use crate::item::Item;
use crate::stateful_list::StatefulList;

/// The UI for selecting the boot option to use as [`BootNext`].
pub struct BootNextSelectorUI<'a, B: tui::backend::Backend> {
    /// The terminal backend.
    terminal: &'a mut Terminal<B>,
    /// The input backend.
    input: &'a mut dyn InputBackend,
    /// The UI state.
    state: StatefulList<'a, Item>,
}

impl<'a, B: tui::backend::Backend> BootNextSelectorUI<'a, B> {
    /// Construct a new [`Self`].
    pub fn new(
        terminal: &'a mut Terminal<B>,
        input: &'a mut dyn InputBackend,
        items: &'a [Item],
        current_item: usize,
    ) -> Self {
        let state = StatefulList::new(items, current_item);
        Self {
            terminal,
            input,
            state,
        }
    }

    /// Execute the UI and return the selected load option index (if any).
    pub fn run(&mut self) -> Result<Option<usize>, anyhow::Error> {
        let result = loop {
            let state = &mut self.state;

            {
                let (iter, state) = state.render_params();
                self.terminal.draw(|mut f| {
                    let rect = f.size();
                    let style = Style::default();
                    let list = List::new(iter.map(Into::into))
                        .highlight_style(style.modifier(Modifier::BOLD))
                        .highlight_symbol("- ");
                    f.render_stateful_widget(list, rect, state)
                })?;
            }

            let evt = match self.input.next() {
                Some(evt) => evt,
                None => continue,
            };
            match evt {
                Event::Quit => break None,
                Event::Down => state.next(),
                Event::Up => state.previous(),
                Event::Enter => break Some(state.selected_index()),
                _ => {}
            }
        };
        Ok(result)
    }
}

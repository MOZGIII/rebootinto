use crate::event::Event;
use crate::input_backend::InputBackend;
use tui::style::{Modifier, Style};
use tui::widgets::List;
use tui::Terminal;

use crate::item::Item;
use crate::stateful_list::StatefulList;
use crate::Result;

pub struct BootNextSelectorUI<'a, B: tui::backend::Backend> {
    terminal: &'a mut Terminal<B>,
    input: &'a mut dyn InputBackend,
    state: StatefulList<'a, Item>,
}

impl<'a, B: tui::backend::Backend> BootNextSelectorUI<'a, B> {
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

    pub fn run(&mut self) -> Result<Option<usize>> {
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

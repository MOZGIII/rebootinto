use crate::event::Event;
use crate::input_backend::InputBackend;
use tui::style::{Modifier, Style};
use tui::widgets::{SelectableList, Widget};
use tui::Terminal;

use crate::Item;
use crate::Result;

pub struct BootNextSelectorUI<'a, B: tui::backend::Backend> {
    terminal: &'a mut Terminal<B>,
    input: &'a mut dyn InputBackend,
    items: &'a [Item],
    current_item: usize,
}

impl<'a, B: tui::backend::Backend> BootNextSelectorUI<'a, B> {
    pub fn new(
        terminal: &'a mut Terminal<B>,
        input: &'a mut dyn InputBackend,
        items: &'a [Item],
        current_item: usize,
    ) -> Self {
        Self {
            terminal: terminal,
            input: input,
            items: items,
            current_item: current_item,
        }
    }

    pub fn run(&mut self) -> Result<Option<usize>> {
        let mut result: Option<usize> = None;

        loop {
            let items = self.items;
            let current_item = self.current_item;

            self.terminal.draw(|mut f| {
                let rect = f.size();
                let style = Style::default();
                SelectableList::default()
                    .items(&items)
                    .select(Some(current_item))
                    .highlight_style(style.modifier(Modifier::BOLD))
                    .highlight_symbol("-")
                    .render(&mut f, rect);
            })?;

            let evt = match self.input.next() {
                Some(evt) => evt,
                None => continue,
            };
            match evt {
                Event::Quit => {
                    break;
                }
                Event::Down => {
                    self.current_item = if current_item >= self.items.len() - 1 {
                        0
                    } else {
                        current_item + 1
                    }
                }
                Event::Up => {
                    self.current_item = if current_item > 0 {
                        current_item - 1
                    } else {
                        self.items.len() - 1
                    }
                }
                Event::Enter => {
                    result = Some(self.current_item);
                    break;
                }
                _ => {}
            }
        }

        Ok(result)
    }
}

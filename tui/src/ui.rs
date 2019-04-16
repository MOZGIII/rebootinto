use termion::event::{Event, Key};
use termion::input::Events;
use tui::style::{Modifier, Style};
use tui::widgets::{SelectableList, Widget};
use tui::Terminal;

use crate::Item;
use crate::Result;

pub struct BootNextSelectorUI<'a, B: tui::backend::Backend, R: std::io::Read> {
    terminal: &'a mut Terminal<B>,
    events: &'a mut Events<R>,
    items: &'a [Item],
    current_item: usize,
}

impl<'a, B: tui::backend::Backend, R: std::io::Read> BootNextSelectorUI<'a, B, R> {
    pub fn new(
        terminal: &'a mut Terminal<B>,
        events: &'a mut Events<R>,
        items: &'a [Item],
        current_item: usize,
    ) -> Self {
        Self {
            terminal: terminal,
            events: events,
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
                    .render(&mut f, rect);
            })?;

            let evt = match self.events.next() {
                Some(evt) => evt,
                None => break,
            }?;
            match evt {
                Event::Key(Key::Char('q')) => {
                    break;
                }
                Event::Key(Key::Down) => {
                    self.current_item = if current_item >= self.items.len() - 1 {
                        0
                    } else {
                        current_item + 1
                    }
                }
                Event::Key(Key::Up) => {
                    self.current_item = if current_item > 0 {
                        current_item - 1
                    } else {
                        self.items.len() - 1
                    }
                }
                Event::Key(Key::Char('\n')) => {
                    result = Some(self.current_item);
                    break;
                }
                _ => {}
            }
        }

        Ok(result)
    }
}

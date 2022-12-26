//! A stateful list implementation.

use tui::widgets::ListState;

/// The stateful list.
///
/// Bind the [`ListState`] and its state together.
pub struct StatefulList<'a, T> {
    /// The [`tui`] list state.
    state: ListState,
    /// The list that are being managed.
    items: &'a [T],
}

impl<'a, T> StatefulList<'a, T> {
    /// Construct a stateful list with the given `items` and the `current_item`.
    ///
    /// The `current_item` selection on construction is required by design.
    pub fn new(items: &'a [T], current_item: usize) -> Self {
        let mut state = ListState::default();
        state.select(Some(current_item));
        Self { state, items }
    }

    /// Select next list item.
    pub fn next(&mut self) {
        let new = match self.state.selected() {
            Some(current) => {
                if current >= self.items.len() - 1 {
                    0
                } else {
                    current + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(new));
    }

    /// Select previous list item.
    pub fn previous(&mut self) {
        let new = match self.state.selected() {
            Some(current) => {
                if current > 0 {
                    current - 1
                } else {
                    self.items.len() - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(new));
    }

    /// Return the currently selected index.
    pub fn selected_index(&self) -> usize {
        self.state.selected().unwrap()
    }

    /// Expose the params required for rendering.
    pub fn render_params(&mut self) -> (std::slice::Iter<'_, T>, &mut ListState) {
        let state = &mut self.state;
        let iter = self.items.iter();
        (iter, state)
    }
}

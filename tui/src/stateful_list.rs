use tui::widgets::ListState;

pub struct StatefulList<'a, T> {
    state: ListState,
    items: &'a [T],
}

impl<'a, T> StatefulList<'a, T> {
    pub fn new(items: &'a [T], current_item: usize) -> Self {
        let mut state = ListState::default();
        state.select(Some(current_item));
        Self { state, items }
    }

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

    pub fn selected_index(&self) -> usize {
        self.state.selected().unwrap()
    }

    pub fn render_params(&mut self) -> (std::slice::Iter<'_, T>, &mut ListState) {
        let state = &mut self.state;
        let iter = self.items.iter();
        (iter, state)
    }
}

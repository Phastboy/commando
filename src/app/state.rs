use crate::features::CommitType;

pub struct AppState {
    pub running: bool,
    pub commit_types: Vec<CommitType>,
    pub selected: usize,
}

impl AppState {
    pub fn new(commit_types: Vec<CommitType>) -> Self {
        Self {
            running: false,
            commit_types,
            selected: 0,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

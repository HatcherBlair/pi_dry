#[derive(Debug, PartialEq)]
pub enum DisplayState {
    Idle,
    Menu,
}

#[derive(Debug)]
pub struct Display {
    state: DisplayState,
}

impl Display {
    pub fn new() -> Self {
        Self {
            state: DisplayState::Idle,
        }
    }
}

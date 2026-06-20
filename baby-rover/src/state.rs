#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Forward,
    Reverse,
    Left,
    Right,
    Idle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    Forward,
    Reverse,
    Left,
    Right,
    Stop,
    Timeout,
}

pub struct StateMachine {
    state: State,
}

impl StateMachine {
    pub fn new() -> Self {
        Self { state: State::Idle }
    }

    pub fn next(&mut self, event: Event) {
        self.state = match event {
            Event::Forward => State::Forward,
            Event::Reverse => State::Reverse,
            Event::Left => State::Left,
            Event::Right => State::Right,
            Event::Stop | Event::Timeout => State::Idle,
        }
    }

    pub fn current(&self) -> State {
        self.state
    }
}

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
}

pub struct StateMachine {
    state: State,
}

impl Default for StateMachine {
    fn default() -> StateMachine {
        StateMachine { state: State::Idle }
    }
}

impl StateMachine {
    pub fn next(&mut self, event: Event) {
        self.state = match event {
            Event::Forward => State::Forward,
            Event::Reverse => State::Reverse,
            Event::Left => State::Left,
            Event::Right => State::Right,
            Event::Stop => State::Idle,
        }
    }

    pub fn current(&self) -> State {
        self.state
    }
}

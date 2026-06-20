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

struct StateMachine {
    state: State,
}

impl StateMachine {
    fn new() -> Self {
        Self { state: State::Idle }
    }

    fn next(&mut self, event: Event) {
        self.state = match event {
            Event::Forward => State::Forward,
            Event::Reverse => State::Reverse,
            Event::Left => State::Left,
            Event::Right => State::Right,
            Event::Stop | Event::Timeout => State::Idle,
        }
    }

    fn current(&self) -> State {
        self.state
    }
}

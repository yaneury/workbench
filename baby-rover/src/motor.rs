use crate::error;
use arduino_hal::port::Pin;
use arduino_hal::port::{mode::Output, D4, D5, D6, D7};

pub enum Direction {
    Forward,
    Reverse,
    Left,
    Right,
}

pub struct Motor {
    left_forward: Pin<Output, D7>,  // IN1
    left_reverse: Pin<Output, D6>,  // IN2
    right_forward: Pin<Output, D5>, // IN3
    right_reverse: Pin<Output, D4>, // IN4
}

impl Motor {
    pub fn new(
        d4: Pin<Output, D4>,
        d5: Pin<Output, D5>,
        d6: Pin<Output, D6>,
        d7: Pin<Output, D7>,
    ) -> Motor {
        Motor {
            left_forward: d7,
            left_reverse: d6,
            right_forward: d5,
            right_reverse: d4,
        }
    }

    pub fn drive(&mut self, direction: Direction) -> Result<(), error::Error> {
        match direction {
            Direction::Forward => self.forward(),
            Direction::Reverse => self.reverse(),
            Direction::Left => self.skid_left(),
            Direction::Right => self.skid_right(),
        }

        Ok(())
    }

    pub fn stop(&mut self) {
        self.left_forward.set_low();
        self.left_reverse.set_low();
        self.right_forward.set_low();
        self.right_reverse.set_low();
    }

    fn forward(&mut self) {
        self.left_forward.set_high();
        self.left_reverse.set_low();
        self.right_forward.set_high();
        self.right_reverse.set_low();
    }

    fn reverse(&mut self) {
        self.left_forward.set_low();
        self.left_reverse.set_high();
        self.right_forward.set_low();
        self.right_reverse.set_high();
    }

    fn skid_left(&mut self) {
        self.left_forward.set_low();
        self.left_reverse.set_high();
        self.right_forward.set_high();
        self.right_reverse.set_low();
    }

    fn skid_right(&mut self) {
        self.left_forward.set_high();
        self.left_reverse.set_low();
        self.right_forward.set_low();
        self.right_reverse.set_high();
    }
}

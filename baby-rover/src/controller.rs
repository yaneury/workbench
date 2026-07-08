use crate::motor::{Direction, Motor};
use crate::state::{self, Event, State, StateMachine};
use crate::transport::{Command, Transport};
use crate::{board, config, debug, error, log, motor, timer, transport};

pub struct Controller<S> {
    inner: S,
}

pub struct Uninit {
    board: board::Board,
}

impl Controller<Uninit> {
    pub fn new(board: board::Board) -> Controller<Uninit> {
        Controller {
            inner: Uninit { board },
        }
    }

    pub fn setup(&mut self, cfg: &config::Config) -> Result<Controller<Ready>, error::Error> {
        timer::millis_init(self.inner.board.take_millis_timer().unwrap());
        log::init_logger(self.inner.board.take_serial_tx().unwrap());

        let transport = transport::SerialTransport::new(self.inner.board.take_serial_rx().unwrap());
        let led = self.inner.board.take_led().unwrap();
        let (d4, d5, d6, d7) = self.inner.board.take_motor_pins().unwrap();

        debug!("Initiazed. Entering loop.");

        Ok(Controller {
            inner: Ready {
                led_pin: led,
                command_transport: transport,
                blink_interval: cfg.blink_interval(),
                state_machine: StateMachine::default(),
                last_toggle_time: 0,
                motor: Motor::new(d4, d5, d6, d7),
            },
        })
    }
}

pub struct Ready {
    state_machine: state::StateMachine,
    motor: motor::Motor,
    led_pin: board::LedPin,
    command_transport: transport::SerialTransport<board::UsartRx>,
    blink_interval: u32,
    last_toggle_time: u32,
}

impl Controller<Ready> {
    pub fn run(&mut self) -> ! {
        loop {
            let current_time = timer::millis();

            if current_time - self.inner.last_toggle_time >= self.inner.blink_interval {
                self.inner.led_pin.toggle();
                self.inner.last_toggle_time = current_time;
            }

            if let Ok(Some(command)) = self.inner.command_transport.receive() {
                let event = match command {
                    Command::Forward => Event::Forward,
                    Command::Reverse => Event::Reverse,
                    Command::Left => Event::Left,
                    Command::Right => Event::Right,
                };

                self.inner.state_machine.next(event);
            } else {
                self.inner.state_machine.next(Event::Stop)
            }

            let () = match self.inner.state_machine.current() {
                State::Forward => self.inner.motor.drive(Direction::Forward).unwrap(),
                State::Reverse => self.inner.motor.drive(Direction::Reverse).unwrap(),
                State::Left => self.inner.motor.drive(Direction::Left).unwrap(),
                State::Right => self.inner.motor.drive(Direction::Right).unwrap(),
                State::Idle => self.inner.motor.stop(),
            };
        }
    }
}

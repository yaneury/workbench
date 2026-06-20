use crate::state;
use crate::transport::{Command, Transport};
use crate::{board, config, debug, error, log, timer, transport};

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

        debug!("Initiazed. Entering loop.");

        Ok(Controller {
            inner: Ready {
                led_pin: led,
                command_transport: transport,
                blink_interval: cfg.blink_interval(),
                state: state::State::Idle,
                last_toggle_time: 0,
            },
        })
    }
}

pub struct Ready {
    state: state::State,
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
                match command {
                    Command::Forward => debug!("Forward"),
                    Command::Reverse => debug!("Reverse"),
                    Command::Left => debug!("Left"),
                    Command::Right => debug!("Right"),
                }
            }
        }
    }
}

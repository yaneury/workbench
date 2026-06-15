use crate::transport::{Command, Transport};
use crate::{board, config, debug, error, log, timer, transport};

struct ControllerInner {
    led_pin: board::LedPin,
    command_transport: transport::SerialTransport<board::UsartRx>,
    blink_interval: u32,
    last_toggle_time: u32,
}

impl ControllerInner {
    fn new(
        led_pin: board::LedPin,
        command_transport: transport::SerialTransport<board::UsartRx>,
        blink_interval: u32,
    ) -> ControllerInner {
        ControllerInner {
            led_pin,
            command_transport,
            blink_interval,
            last_toggle_time: 0,
        }
    }

    fn run(&mut self) -> ! {
        loop {
            let current_time = timer::millis();

            if current_time - self.last_toggle_time >= self.blink_interval {
                self.led_pin.toggle();
                self.last_toggle_time = current_time;
            }

            if let Ok(Some(command)) = self.command_transport.receive() {
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

pub struct Controller {
    board: board::Board,
    inner: Option<ControllerInner>,
}

impl Controller {
    pub fn new(board: board::Board) -> Controller {
        Controller { board, inner: None }
    }

    pub fn setup(&mut self, cfg: &config::Config) -> Result<(), error::Error> {
        timer::millis_init(self.board.take_millis_timer().unwrap());
        log::init_logger(self.board.take_serial_tx().unwrap());

        let transport = transport::SerialTransport::new(self.board.take_serial_rx().unwrap());
        let led = self.board.take_led().unwrap();

        self.inner = Some(ControllerInner::new(led, transport, cfg.blink_interval()));
        debug!("Initiazed. Entering loop.");

        Ok(())
    }

    pub fn run(&mut self) -> ! {
        if let Some(mut inner) = self.inner.take() {
            inner.run();
        }

        loop {}
    }
}

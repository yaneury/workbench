use crate::config;
use crate::error;
use arduino_hal::hal;
use arduino_hal::port::mode::Output;
use arduino_hal::port::{Pin, D4, D5, D6, D7};

pub type UsartRx = hal::usart::UsartReader<
    hal::pac::USART0,                                        // USART peripheral
    hal::port::Pin<hal::port::mode::Input, hal::port::PD0>,  // RX pin
    hal::port::Pin<hal::port::mode::Output, hal::port::PD1>, // TX pin
    hal::clock::MHz16,                                       // CLOCK
>;

pub type UsartTx = hal::usart::UsartWriter<
    hal::pac::USART0,
    hal::port::Pin<hal::port::mode::Input, hal::port::PD0>,
    hal::port::Pin<hal::port::mode::Output, hal::port::PD1>,
    hal::clock::MHz16,
>;

// Mappins of Atmega Hal pins to Ardunio Uno can be found at
// https://rahix.github.io/avr-hal/src/arduino_hal/port/uno.rs.html.
pub type LedPin = hal::port::Pin<hal::port::mode::Output, hal::port::PB5>;

pub struct Board {
    serial_rx: Option<UsartRx>,
    serial_tx: Option<UsartTx>,
    led_pin: Option<LedPin>,
    millis_timer: Option<arduino_hal::pac::TC0>,
    motor_d4: Option<Pin<Output, D4>>,
    motor_d5: Option<Pin<Output, D5>>,
    motor_d6: Option<Pin<Output, D6>>,
    motor_d7: Option<Pin<Output, D7>>,
}

impl Board {
    pub fn new(cfg: &config::Config) -> Result<Self, error::Error> {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);

        let led_pin = pins.d13.into_output();

        let serial = arduino_hal::default_serial!(dp, pins, cfg.baud_rate());
        let (serial_rx, serial_tx) = serial.split();

        Ok(Board {
            serial_rx: Some(serial_rx),
            serial_tx: Some(serial_tx),
            led_pin: Some(led_pin),
            millis_timer: Some(dp.TC0),
            motor_d4: Some(pins.d4.into_output()),
            motor_d5: Some(pins.d5.into_output()),
            motor_d6: Some(pins.d6.into_output()),
            motor_d7: Some(pins.d7.into_output()),
        })
    }

    pub fn take_serial_rx(&mut self) -> Option<UsartRx> {
        self.serial_rx.take()
    }

    pub fn take_serial_tx(&mut self) -> Option<UsartTx> {
        self.serial_tx.take()
    }

    pub fn take_led(&mut self) -> Option<LedPin> {
        self.led_pin.take()
    }

    pub fn take_millis_timer(&mut self) -> Option<arduino_hal::pac::TC0> {
        self.millis_timer.take()
    }

    pub fn take_motor_pins(
        &mut self,
    ) -> Option<(
        Pin<Output, D4>,
        Pin<Output, D5>,
        Pin<Output, D6>,
        Pin<Output, D7>,
    )> {
        Some((
            self.motor_d4.take()?,
            self.motor_d5.take()?,
            self.motor_d6.take()?,
            self.motor_d7.take()?,
        ))
    }
}

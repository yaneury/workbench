use crate::error;
use arduino_hal::hal;

type UsartRx = hal::usart::UsartReader<
    hal::pac::USART0,                                        // USART peripheral
    hal::port::Pin<hal::port::mode::Input, hal::port::PD0>,  // RX pin
    hal::port::Pin<hal::port::mode::Output, hal::port::PD1>, // TX pin
    hal::clock::MHz16,                                       // CLOCK
>;

type UsartTx = hal::usart::UsartWriter<
    hal::pac::USART0,
    hal::port::Pin<hal::port::mode::Input, hal::port::PD0>,
    hal::port::Pin<hal::port::mode::Output, hal::port::PD1>,
    hal::clock::MHz16,
>;

// Mappins of Atmega Hal pins to Ardunio Uno can be found at
// https://rahix.github.io/avr-hal/src/arduino_hal/port/uno.rs.html.
type LedPin = hal::port::Pin<hal::port::mode::Output, hal::port::PB5>;

pub struct Board {
    serial_rx: Option<UsartRx>,
    serial_tx: Option<UsartTx>,
    led_pin: Option<LedPin>,
    millis_timer: Option<arduino_hal::pac::TC0>,
}

impl Board {
    pub fn new() -> Result<Self, error::Error> {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);

        let led_pin = pins.d13.into_output();

        let serial = arduino_hal::default_serial!(dp, pins, 9600);
        let (serial_rx, serial_tx) = serial.split();

        Ok(Board {
            serial_rx: Some(serial_rx),
            serial_tx: Some(serial_tx),
            led_pin: Some(led_pin),
            millis_timer: Some(dp.TC0),
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
}

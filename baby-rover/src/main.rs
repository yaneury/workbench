#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod controller;
mod error;
mod motor;
mod state;
mod timer;
mod transport;

use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt::uwrite;

use crate::transport::{Command, Transport};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    timer::millis_init(dp.TC0);

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let (mut rx, mut tx) = serial.split();

    let mut transport = transport::SerialTransport::new(&mut rx);

    let mut led = pins.d13.into_output();
    let mut delay_multiplier: u8 = 10;

    let mut last_toggle_time: u32 = 0;
    let blink_interval: u32 = 1000; // 100ms

    loop {
        let current_time = timer::millis();

        if current_time - last_toggle_time >= blink_interval {
            led.toggle();
            last_toggle_time = current_time;
            uwrite!(&mut tx, "Toggling\r\n").unwrap();
        }

        if let Ok(Some(command)) = transport.receive() {
            match command {
                Command::Forward => uwrite!(&mut tx, "Forward\r\n").unwrap(),
                Command::Reverse => uwrite!(&mut tx, "Reverse\r\n").unwrap(),
                Command::Left => uwrite!(&mut tx, "Left\r\n").unwrap(),
                Command::Right => uwrite!(&mut tx, "Right\r\n").unwrap(),
            }
        }
    }
}

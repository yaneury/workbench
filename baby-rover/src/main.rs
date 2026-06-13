#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod board;
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
    let mut board = board::Board::new().unwrap();

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    timer::millis_init(board.take_millis_timer().unwrap());

    let mut transport = transport::SerialTransport::new(board.take_serial_rx().unwrap());
    let mut serial_tx = board.take_serial_tx().unwrap();

    let mut led = board.take_led().unwrap();
    let delay_multiplier: u8 = 10;

    let mut last_toggle_time: u32 = 0;
    let blink_interval: u32 = 1000; // 100ms

    loop {
        let current_time = timer::millis();

        if current_time - last_toggle_time >= blink_interval {
            led.toggle();
            last_toggle_time = current_time;
            uwrite!(&mut serial_tx, "Toggling\r\n").unwrap();
        }

        if let Ok(Some(command)) = transport.receive() {
            match command {
                Command::Forward => uwrite!(&mut serial_tx, "Forward\r\n").unwrap(),
                Command::Reverse => uwrite!(&mut serial_tx, "Reverse\r\n").unwrap(),
                Command::Left => uwrite!(&mut serial_tx, "Left\r\n").unwrap(),
                Command::Right => uwrite!(&mut serial_tx, "Right\r\n").unwrap(),
            }
        }
    }
}

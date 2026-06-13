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

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    timer::millis_init(dp.TC0);

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    let mut led = pins.d13.into_output();
    let mut delay_multiplier: u8 = 10;

    let mut last_toggle_time: u32 = 0;
    let blink_interval: u32 = 1000; // 100ms

    loop {
        let current_time = timer::millis();

        if current_time - last_toggle_time >= blink_interval {
            led.toggle();
            last_toggle_time = current_time;
            uwrite!(&mut serial, "Toggling\r\n").unwrap();
        }

        if let Ok(byte) = serial.read() {
            match byte {
                b'0'..=b'9' => {
                    delay_multiplier = byte - b'0';
                    uwrite!(&mut serial, "Delay: {}\r\n", delay_multiplier).unwrap();
                }
                b'\n' | b'\r' => {
                    // Ignore newlines/carriage returns
                }
                _ => {
                    uwrite!(&mut serial, "Invalid: {}\r\n", byte).unwrap();
                }
            }
        }
    }
}

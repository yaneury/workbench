#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut in1 = pins.d4.into_output();
    let mut in2 = pins.d5.into_output();
    let mut in3 = pins.d6.into_output();
    let mut in4 = pins.d7.into_output();

    loop {
        // Forward: both sides forward
        in1.set_high();
        in2.set_low();
        in3.set_high();
        in4.set_low();
        arduino_hal::delay_ms(2000);

        // Stop
        in1.set_low();
        in2.set_low();
        in3.set_low();
        in4.set_low();
        arduino_hal::delay_ms(1000);

        // Reverse: both sides reverse
        in1.set_low();
        in2.set_high();
        in3.set_low();
        in4.set_high();
        arduino_hal::delay_ms(2000);

        // Stop
        in1.set_low();
        in2.set_low();
        in3.set_low();
        in4.set_low();
        arduino_hal::delay_ms(1000);

        // Left: left reverse, right forward
        in1.set_low();
        in2.set_high();
        in3.set_high();
        in4.set_low();
        arduino_hal::delay_ms(2000);

        // Stop
        in1.set_low();
        in2.set_low();
        in3.set_low();
        in4.set_low();
        arduino_hal::delay_ms(1000);

        // Right: left forward, right reverse
        in1.set_high();
        in2.set_low();
        in3.set_low();
        in4.set_high();
        arduino_hal::delay_ms(2000);

        // Stop
        in1.set_low();
        in2.set_low();
        in3.set_low();
        in4.set_low();
        arduino_hal::delay_ms(1000);
    }
}

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod board;
mod config;
mod controller;
mod error;
mod log;
mod motor;
mod state;
mod timer;
mod transport;

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    // First things first, enable interrupts globally.
    unsafe { avr_device::interrupt::enable() };
    let cfg = &config::Config::default();
    let board = board::Board::new(&cfg).unwrap();

    let mut controller = controller::Controller::new(board);

    let mut controller = controller.setup(cfg).unwrap();
    controller.run();
}

#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
    spi::{
        Mode,
        master::{Config, Spi},
    },
    time::Rate,
};
use esp_println::println;
use marginilia::{display::Display, model::Quote};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    println!("Marginilia: starting...");

    let dc = Output::new(peripherals.GPIO27, Level::Low, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO26, Level::Low, OutputConfig::default());
    let cs = Output::new(peripherals.GPIO15, Level::High, OutputConfig::default());
    let busy = Input::new(
        peripherals.GPIO25,
        InputConfig::default().with_pull(Pull::None),
    );

    let spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_mhz(4))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(peripherals.GPIO13)
    .with_mosi(peripherals.GPIO14);

    let spi_dev = ExclusiveDevice::new(spi, cs, Delay::new()).unwrap();
    let mut display = Display::new(spi_dev, busy, dc, rst, Delay::new()).unwrap();

    let quote = Quote {
        body: "Well-run libraries are filled with people because what a good library offers \
               cannot be easily found elsewhere: an indoor public space in which you do not \
               have to buy anything in order to say.",
        author: "Zadie Smith",
        work: "Northwest London Blues",
    };

    display.show_quote(&quote).unwrap();

    println!("Marginilia: display updated.");

    loop {}
}

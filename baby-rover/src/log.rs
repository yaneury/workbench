use arduino_hal::hal;
use core::cell::RefCell;
use ufmt::uwrite;

type SerialTx = hal::usart::UsartWriter<
    hal::pac::USART0,
    hal::port::Pin<hal::port::mode::Input, hal::port::PD0>,
    hal::port::Pin<hal::port::mode::Output, hal::port::PD1>,
    hal::clock::MHz16,
>;

static mut SERIAL: Option<SerialTx> = None;

pub fn init_logger(serial: SerialTx) {
    unsafe {
        SERIAL = Some(serial);
    }
}

pub fn log_impl(msg: &str) {
    unsafe {
        if let Some(ref mut serial) = SERIAL.as_mut() {
            ufmt::write!(serial, "{}", msg).ok()
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::log(&ufmt::uformat_as_str!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::log(&ufmt::uformat_as_str!($($arg)*));
    };
}

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {
        $crate::debug!($($arg)*);
        $crate::debug!("\r\n");
    };
}


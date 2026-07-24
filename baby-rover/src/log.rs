use arduino_hal::hal;

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
            ufmt::uwriteln!(serial, "{}", msg).ok();
        }
    }
}

pub fn log_bytes(bytes: &[u8]) {
    unsafe {
        if let Some(ref mut serial) = SERIAL.as_mut() {
            for byte in bytes {
                ufmt::uwrite!(serial, "{} ", byte).ok();
            }
            ufmt::uwriteln!(serial, "").ok();
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::log_impl(&$($arg)*);
    };
}

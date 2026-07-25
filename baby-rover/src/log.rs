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

#[cfg(debug_assertions)]
pub fn log_impl(msg: &str) {
    unsafe {
        if let Some(ref mut serial) = *(&raw mut SERIAL) {
            ufmt::uwriteln!(serial, "{}", msg).ok();
        }
    }
}

#[cfg(not(debug_assertions))]
pub fn log_impl(_: &str) {}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::log_impl(&$($arg)*);
    };
}

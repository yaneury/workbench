use arduino_hal::hal;
use heapless::String;

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
            static mut BUFFER: String<256> = String::new();
            ufmt::uwrite!(BUFFER, "{}\r\n", msg).ok();
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::log_impl(&$($arg)*);
    };
}

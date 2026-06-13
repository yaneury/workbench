use crate::error;
use arduino_hal::hal;
use arduino_hal::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    Forward,
    Reverse,
    Left,
    Right,
}

pub trait Transport {
    fn connect(&mut self) -> Result<(), error::Error>;
    fn receive(&mut self) -> Result<Option<Command>, error::Error>;
}

type ArdunioUnoSerial = hal::usart::Usart0<hal::clock::MHz16>;

pub struct SerialTransport {
    serial: ArdunioUnoSerial,
}

impl SerialTransport {
    fn new(serial: ArdunioUnoSerial) -> SerialTransport {
        SerialTransport { serial: serial }
    }
}

impl Transport for SerialTransport {
    fn connect(&mut self) -> Result<(), error::Error> {
        Ok(())
    }

    fn receive(&mut self) -> Result<Option<Command>, error::Error> {
        if let Ok(byte) = self.serial.read() {
            return match byte {
                b'K' | b'k' => Ok(Some(Command::Forward)),
                b'J' | b'j' => Ok(Some(Command::Reverse)),
                b'H' | b'h' => Ok(Some(Command::Left)),
                b'L' | b'l' => Ok(Some(Command::Right)),
                b'\n' | b'\r' => Ok(None),
                // Ignore newlines/carriage returns
                _ => Ok(None), // TODO: Handle, I guess.
            };
        }

        Ok(None)
    }
}

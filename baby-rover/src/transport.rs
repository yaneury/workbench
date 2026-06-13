use crate::error;
use arduino_hal::hal;
use arduino_hal::prelude::*;
use embedded_hal_v0::serial::Read;

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

pub struct SerialTransport<'a, S> {
    serial: &'a mut S,
}

impl<'a, S> SerialTransport<'a, S> {
    pub fn new(serial: &'a mut S) -> Self {
        SerialTransport { serial: serial }
    }
}

impl<'a, S> Transport for SerialTransport<'a, S>
where
    S: Read<u8>,
{
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

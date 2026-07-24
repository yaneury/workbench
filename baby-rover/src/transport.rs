use crate::error;
use embedded_hal_v0::serial::Read;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    Forward,
    Reverse,
    Left,
    Right,
    Stop,
}

pub trait Transport {
    fn receive(&mut self) -> Result<Option<Command>, error::Error>;
}

pub struct SerialTransport<S> {
    serial: S,
}

#[allow(dead_code)]
impl<S> SerialTransport<S> {
    pub fn new(serial: S) -> Self {
        SerialTransport { serial: serial }
    }
}

impl<S> Transport for SerialTransport<S>
where
    S: Read<u8>,
{
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

// Object able to parse basic BT commands coming from Dabble iOS app.
pub struct DabbleBTTransport<S> {
    serial: S,
}

impl<S> DabbleBTTransport<S> {
    pub fn new(serial: S) -> Self {
        DabbleBTTransport { serial: serial }
    }
}

impl<S> Transport for DabbleBTTransport<S>
where
    S: Read<u8>,
{
    fn receive(&mut self) -> Result<Option<Command>, error::Error> {
        // debug!("Receive called");
        let mut buf = [0u8; 8];
        let mut read = 0;

        // Header byte must be 0xff, otherwise we ignore.
        if let Ok(0xff) = self.serial.read() {
            // debug!("Header read");
            buf[0] = 0xff;
            read += 1;

            for i in 1..=7 {
                loop {
                    if let Ok(byte) = self.serial.read() {
                        buf[i] = byte;
                        read += 1;
                        break;
                    }
                }
            }
        }

        // debug!("Serial read. Do not know if header.");

        if read != 8 {
            // debug!("Did not read 8 bytes!");
            return Ok(None);
        }

        if let Ok(Some(command)) = decode_dabble_message(&buf) {
            // debug!("Read 8 bytes!");
            // Dabble app sends a "Release" command immediately after sending a direction
            // command. We ignore that here explicitly so it doesn't interferece with driver
            // logic.
            // for _ in 0..8 {
            //     loop {
            //         if let Ok(_byte) = self.serial.read() {
            //             break;
            //         }
            //     }
            // }

            return Ok(Some(command));
        }

        Ok(None)
    }
}

/*

Partial reverse engineering of Dabble iOS app message.

Full 8 Byte Message

┌───────┬───────────┬───────────────────┐
│ Index │   Value   │      Meaning      │
├───────┼───────────┼───────────────────┤
│ 0     │ 0xFF      │ Header            │
├───────┼───────────┼───────────────────┤
│ 1     │ 0x01      │ Fixed             │
├───────┼───────────┼───────────────────┤
│ 2     │ 0x01      │ Fixed             │
├───────┼───────────┼───────────────────┤
│ 3     │ 0x01      │ Fixed             │
├───────┼───────────┼───────────────────┤
│ 4     │ 0x02      │ Fixed             │
├───────┼───────────┼───────────────────┤
│ 5     │ 0x00      │ Fixed             │
├───────┼───────────┼───────────────────┤
│ 6     │ 0x00-0x08 │ Direction bitmask │
├───────┼───────────┼───────────────────┤
│ 7     │ 0x00      │ Fixed             │
└───────┴───────────┴───────────────────┘

Direction (Index 6)

┌───────────┬────────────┐
│ Direction │ Byte value │
├───────────┼────────────┤
│ Up        │ 0x01       │
├───────────┼────────────┤
│ Down      │ 0x02       │
├───────────┼────────────┤
│ Left      │ 0x04       │
├───────────┼────────────┤
│ Right     │ 0x08       │
├───────────┼────────────┤
│ Release   │ 0x00       │
└───────────┴────────────┘

*/

fn decode_dabble_message(bytes: &[u8; 8]) -> Result<Option<Command>, error::Error> {
    // debug!("Decoding dabble message!");
    crate::log::log_bytes(bytes);

    if bytes[5] != 0 {
        return Ok(Some(Command::Stop));
    }

    Ok(Some(match bytes[6] {
        0x01 => Command::Forward,
        0x02 => Command::Reverse,
        0x04 => Command::Left,
        0x08 => Command::Right,
        _ => {
            return Err(error::Error::Parsing);
        }
    }))
}

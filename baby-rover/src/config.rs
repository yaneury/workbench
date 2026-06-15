// Program config 
pub struct Config {
    serial_baud_rate: u32,
}

impl Config {
    pub fn default() -> Config {
        Config {
            serial_baud_rate: 9600,
        }
    }

    pub fn baud_rate(&self) -> u32 {
        self.serial_baud_rate
    }
}



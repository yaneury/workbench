// Program config
pub struct Config {
    baud_rate: u32,
    blink_interval: u32,
}

impl Config {
    pub fn default() -> Config {
        Config {
            baud_rate: 9600,
            blink_interval: 1000, // 1 sec
        }
    }

    pub fn baud_rate(&self) -> u32 {
        self.baud_rate
    }

    pub fn blink_interval(&self) -> u32 {
        self.blink_interval
    }
}

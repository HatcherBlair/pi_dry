use rppal::i2c::I2c;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct TempSensor {
    addr: u16,
}

#[derive(Debug)]
pub enum SHTAddr {
    Default,
    Alternate,
}

const SHT_DEFAULT_ADDR: u16 = 0x44;
const SHT_ALTERNATE_ADDR: u16 = 0x45;

impl TempSensor {
    pub fn new(addr: SHTAddr) -> Self {
        Self {
            addr: match addr {
                SHTAddr::Default => SHT_DEFAULT_ADDR,
                SHTAddr::Alternate => SHT_ALTERNATE_ADDR,
            },
        }
    }

    pub fn read(&self, i2c: &mut I2c) -> (f32, f32) {
        let _ = i2c.set_slave_address(self.addr);

        // High repeatability, single shot measure command
        // Clock stretching disable bc Pi doesn't support properly
        let cmd: [u8; 2] = [0x24, 0x00];

        let mut buf = [0u8; 6];

        // Send cmd
        let _ = i2c.write(&cmd);

        // Wait for sensor to take measurment
        thread::sleep(Duration::from_millis(20));

        // Data format is temp MSB, temp LSB, CRC, Hum MSB, Hum LSB, CRC
        let _ = i2c.read(&mut buf);

        if !TempSensor::crc(&buf[0..2], buf[2]) {
            println!("Temperature CRC not valid");
        }

        if !TempSensor::crc(&buf[3..5], buf[5]) {
            println!("Humidity CRC not valid");
        }

        let temp_raw = u16::from_be_bytes([buf[0], buf[1]]);
        let hum_raw = u16::from_be_bytes([buf[3], buf[4]]);

        // -45 + 175 * (temp / (2^16-1))
        let temperature = -45.0 + 175.0 * (temp_raw as f32) / 65535.0;

        // 100 * (hum / (2^16-1))
        let humidity = 100.0 * (hum_raw as f32) / 65535.0;

        (temperature, humidity)
    }

    // Verifies the CRC for the read temperature and humidity
    fn crc(data: &[u8], crc: u8) -> bool {
        let polynomial: u8 = 0x31;
        let mut init: u8 = 0xFF;

        for byte in data {
            init ^= byte;
            for _ in 0..8 {
                if init & 0x80 != 0 {
                    init = (init << 1) ^ polynomial;
                } else {
                    init <<= 1;
                }
            }
        }

        init == crc
    }
}

use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use rppal::system::DeviceInfo;

// blinking LED example
fn led_example() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink every 500ms
    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();

    Ok(())
}

// Temp and Humid Sensor Skeleton
fn temp_sensor_callback() {}

fn main() {
    println!("Hello, world!");
}

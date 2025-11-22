mod button_cluster;
mod display;
mod dry_table;
mod lcd_interface;
mod shared_data;
mod temp_sensor;

use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use rppal::{
    gpio::{Gpio, OutputPin},
    i2c::I2c,
};

use button_cluster::ButtonCluster;
use shared_data::SharedData;
use temp_sensor::{SHTAddr, TempSensor};

use display::{Display, DisplayState};

use crate::dryer::dry_table::Material;

#[derive(Debug)]
enum HeaterState {
    Idle,
    Running,
}

#[derive(Debug)]
pub struct Dryer {
    display: Display,
    i2c: I2c,
    near_sensor: TempSensor,
    far_sensor: TempSensor,
    buttons: ButtonCluster,
    fan: OutputPin,
    heater: OutputPin,
    data: Arc<Mutex<SharedData>>,
    last_temp: f32,
    last_hum: f32,
    last_reading: Instant,
    display_update: Instant,
}

impl Dryer {
    pub fn new() -> Self {
        // Data shared by callback functions
        let data = Arc::new(Mutex::new(SharedData::new()));

        // Creates the Input pins and sets callbacks for them
        let buttons = ButtonCluster::new(&data);

        // Create the output pins
        let gpio = Gpio::new().unwrap();

        // Relay is open on low
        let fan_pin = gpio.get(14).unwrap().into_output_high(); // Physical Pin 8
        let heater_pin = gpio.get(15).unwrap().into_output_high(); // Phsyical Pin 10

        // Create the sensors
        let near_sensor = TempSensor::new(SHTAddr::Default);
        let far_sensor = TempSensor::new(SHTAddr::Alternate);

        // One I2c instance is passed around because
        // I've had issues with each I2c device holding their own instance
        let mut i2c = I2c::new().unwrap();

        // Initialize the display
        let _ = lcd_interface::init(&mut i2c);

        // First reading of the temperature and humidity sensors
        let near_reading = near_sensor.read(&mut i2c);
        let far_reading = far_sensor.read(&mut i2c);
        // Just averaging the values of the two sensors for now
        let last_temp = (near_reading.0 + far_reading.0) / 2.0;
        let last_hum = (near_reading.1 + far_reading.1) / 2.0;
        let last_reading = Instant::now();

        // Initialize with a time
        let display_update = Instant::now();

        Self {
            display: Display::new(),
            i2c,
            near_sensor,
            far_sensor,
            buttons,
            fan: fan_pin,
            heater: heater_pin,
            data,
            last_temp,
            last_hum,
            last_reading,
            display_update,
        }
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let now = Instant::now();
        let mut shared_data = self.data.lock().unwrap();
        match shared_data.heater_state {
            HeaterState::Idle => {
                if self.heater.is_set_low() {
                    self.heater.set_high();
                }
                if self.fan.is_set_low() {
                    self.fan.set_high();
                }
                // When Idle, only update temperature every 30 seconds
                if now - self.last_reading > Duration::from_secs(30) {
                    let near_reading = self.near_sensor.read(&mut self.i2c);
                    let far_reading = self.far_sensor.read(&mut self.i2c);

                    // Average two sensors together
                    self.last_temp = (near_reading.0 + far_reading.0) / 2.0;
                    self.last_hum = (near_reading.1 + far_reading.1) / 2.0;

                    self.last_reading = Instant::now();
                }
            }
            HeaterState::Running => {
                if shared_data.material == Material::None {
                    if self.fan.is_set_low() {
                        self.fan.set_high();
                    }
                    if self.heater.is_set_low() {
                        self.heater.set_high();
                    }
                } else {
                    let near_reading = self.near_sensor.read(&mut self.i2c);
                    let far_reading = self.far_sensor.read(&mut self.i2c);

                    // Just averaging the values of the two sensors for now
                    self.last_temp = (near_reading.0 + far_reading.0) / 2.0;
                    self.last_hum = (near_reading.1 + far_reading.1) / 2.0;

                    self.last_reading = Instant::now();

                    // Poll fan, ensure running
                    if self.fan.is_set_high() {
                        self.fan.set_low();
                    }

                    // Temperature target has a 3 degree window
                    // Heater is on for +1.5 degrees
                    // Heater is off for -1.5 degrees
                    let target_temp = shared_data.material.get().temp as f32;
                    if self.last_temp < target_temp - 1.5 {
                        if self.heater.is_set_high() {
                            self.heater.set_low();
                        }
                    } else if self.last_temp > target_temp + 1.5 {
                        if self.heater.is_set_low() {
                            self.heater.set_high();
                        }
                    }

                    // Shutdown at end of time
                    if (now - shared_data.heater_started) > shared_data.material.get().time {
                        shared_data.material = Material::None;
                        shared_data.hovered_material = Material::None;
                    }
                }
            }
        }

        // Update display
        // Only update the display at 1Hz
        // Change to a diffing function later to update display whenever the internal state changes
        // Display holds stored value forever
        if now - self.display_update > Duration::from_secs(1) {
            // Write out material and Arrows
            // write out current reading
            // write out menu
            match shared_data.display_state {
                DisplayState::Idle => {
                    let line1: String;

                    if shared_data.material == Material::None {
                        line1 = format!("Idle");
                    } else {
                        let remaining =
                            shared_data.material.get().time - (now - shared_data.heater_started);

                        line1 = format!(
                            "{}: {}:{:.2}:{:.2}",
                            shared_data.material.get().name,
                            remaining.as_secs() / 3600,
                            (remaining.as_secs() % 3600) / 60,
                            remaining.as_secs() % 60
                        );
                    }

                    // Temperature C Humidity %rh
                    let line2 = format!("{:.2}C {:.2}%rh", self.last_temp, self.last_hum);

                    // Reset the display
                    lcd_interface::clear(&mut self.i2c)?;
                    lcd_interface::home(&mut self.i2c)?;

                    // Line 1
                    lcd_interface::print(&mut self.i2c, line1.as_str())?;

                    // Line 2
                    lcd_interface::set_cursor(&mut self.i2c, 0, 1)?;
                    lcd_interface::print(&mut self.i2c, line2.as_str())?;
                }
                DisplayState::Menu => {
                    // Show current hovered option
                    let line1 = format!("<   {}     >", shared_data.hovered_material.get().name);
                    let line2 = format!(
                        "{}C {}hrs  ",
                        shared_data.hovered_material.get().temp,
                        shared_data.hovered_material.get().time.as_secs() / 3600
                    );

                    let _ = lcd_interface::clear(&mut self.i2c);
                    let _ = lcd_interface::home(&mut self.i2c);

                    let _ = lcd_interface::print(&mut self.i2c, line1.as_str());

                    let _ = lcd_interface::set_cursor(&mut self.i2c, 0, 1);
                    let _ = lcd_interface::print(&mut self.i2c, line2.as_str());
                }
            }
        }
        // Printing for debugging purposes
        println!("Current Dryer State:");
        println!(
            "Heater: {} Fan: {}",
            self.heater.is_set_low(),
            self.fan.is_set_low()
        );

        println!("Current Material: {}", shared_data.material.get().name);
        println!(
            "Hovered Material: {}",
            shared_data.hovered_material.get().name
        );
        println!();
        Ok(())
    }
}

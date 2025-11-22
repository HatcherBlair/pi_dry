use crate::dryer::HeaterState;
use crate::dryer::display::DisplayState;
use crate::dryer::shared_data::SharedData;

use rppal::gpio::{Gpio, InputPin, Trigger};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct ButtonCluster {
    back: InputPin,
    confirm: InputPin,
    right: InputPin,
    left: InputPin,
}

impl ButtonCluster {
    pub fn new(data: &Arc<Mutex<SharedData>>) -> Self {
        let gpio = Gpio::new().unwrap();

        // Back Button
        let back_data = data.clone();
        let mut back_pin = gpio.get(17).unwrap().into_input_pulldown(); // Physical Pin 11
        let _ = back_pin.set_async_interrupt(
            Trigger::FallingEdge,
            Some(Duration::from_millis(50)),
            move |_event| {
                let mut shared_state = back_data.lock().unwrap();
                // Go back to the menu, otherwise do nothing
                if shared_state.display_state == DisplayState::Idle {
                    shared_state.display_state = DisplayState::Menu;
                }
                println!("Back Pressed");
            },
        );

        // Confirm Button
        let confirm_data = data.clone();
        let mut confirm_pin = gpio.get(27).unwrap().into_input_pulldown(); // Physical Pin 13
        let _ = confirm_pin.set_async_interrupt(
            Trigger::FallingEdge,
            Some(Duration::from_millis(50)),
            move |_event| {
                let mut shared_state = confirm_data.lock().unwrap();
                // Select the current hovered material as current one, turn on heater module, and
                // change menu to idle state
                if shared_state.display_state == DisplayState::Menu {
                    shared_state.material = shared_state.hovered_material;
                    shared_state.heater_state = HeaterState::Running;
                    shared_state.display_state = DisplayState::Idle;
                    shared_state.heater_started = Instant::now();
                }
                println!("Confirm Pressed");
            },
        );

        // Right Button
        let right_data = data.clone();
        let mut right_pin = gpio.get(10).unwrap().into_input_pulldown(); // Physical Pin 19
        let _ = right_pin.set_async_interrupt(
            Trigger::FallingEdge,
            Some(Duration::from_millis(50)),
            move |_event| {
                let mut shared_state = right_data.lock().unwrap();
                shared_state.hovered_material = shared_state.hovered_material.next();
                println!("Right Pressed");
            },
        );

        // Left Button
        let left_data = data.clone();
        let mut left_pin = gpio.get(9).unwrap().into_input_pulldown(); // Physical Pin 21
        let _ = left_pin.set_async_interrupt(
            Trigger::FallingEdge,
            Some(Duration::from_millis(50)),
            move |_event| {
                let mut shared_state = left_data.lock().unwrap();
                shared_state.hovered_material = shared_state.hovered_material.prev();
                println!("Left Pressed");
            },
        );

        Self {
            back: back_pin,
            confirm: confirm_pin,
            right: right_pin,
            left: left_pin,
        }
    }
}

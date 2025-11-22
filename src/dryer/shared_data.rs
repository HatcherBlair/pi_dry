use std::time::Instant;

use crate::dryer::{HeaterState, display::DisplayState, dry_table::Material};

#[derive(Debug)]
pub struct SharedData {
    pub heater_state: HeaterState,
    pub display_state: DisplayState,
    pub material: Material,
    pub hovered_material: Material,
    pub heater_started: Instant,
}

impl SharedData {
    pub fn new() -> Self {
        Self {
            heater_state: HeaterState::Idle,
            display_state: DisplayState::Idle,
            material: Material::None,
            hovered_material: Material::None,
            heater_started: Instant::now(),
        }
    }
}

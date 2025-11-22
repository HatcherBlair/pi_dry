mod dryer;

use std::{thread, time::Duration};

use dryer::Dryer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dryer = Dryer::new();

    loop {
        dryer.update();
        thread::sleep(Duration::from_millis(1000));
    }
}

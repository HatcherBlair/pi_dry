use rppal::i2c::I2c;

use crate::dryer::lcd_interface;

#[derive(Debug, PartialEq)]
pub enum DisplayState {
    Idle,
    Menu,
}

struct DiffContainer {
    idx: u8,
    text: String,
}

#[derive(Debug)]
pub struct Display {
    state: DisplayState,
    // 1st row is 0x00 -> 0x27
    // 39 Chars? Is that right?
    line1_chars: [char; 39],
    line2_chars: [char; 39],
}

impl Display {
    pub fn new() -> Self {
        Self {
            state: DisplayState::Idle,
            line1_chars: [' '; 39],
            line2_chars: [' '; 39],
        }
    }

    pub fn update(&mut self, i2c: &mut I2c, line1: &str, line2: &str) {
        // Diff the line1 against the chars
        // Write the difference
        // Update the char array

        //
        let mut diffs: Vec<DiffContainer> = Vec::new();
        let mut to_write = String::new();
        let mut cur_writing: bool = false;
        let mut start_idx: u8 = 0;
        for (i, char) in line1.chars().enumerate() {
            if char != self.line1_chars[i] {
                if !cur_writing {
                    cur_writing = true;
                    start_idx = i as u8;
                }
                // Want to continue until we find a different char but I don't know ho
                // how to do that in Rust
                // Need a way to determine if we are currently building a string or not...
                // What if we pushed onto a string and had a bool?
                to_write.push(char);
            } else {
                if cur_writing {
                    cur_writing = false;
                    diffs.push(DiffContainer {
                        idx: start_idx,
                        text: to_write.clone(),
                    });
                    to_write.clear();
                }
            }
        }

        // Assume that is working, now we need to loop over the above and write the text
        for diff in diffs.into_iter() {
            lcd_interface::set_cursor(i2c, diff.idx, 0);
            lcd_interface::print(i2c, diff.text.as_str());
        }

        // Do the same thing for line2

        // Overwrite the current lines?
        // This might cause some length issues?
        for (i, c) in line1.chars().enumerate() {
            self.line1_chars[i] = c;
        }

        for (i, c) in line2.chars().enumerate() {
            self.line2_chars[i] = c;
        }
    }
}

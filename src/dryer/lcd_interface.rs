use rppal::i2c::I2c;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

// Slave address of the display module
const ADDR: u16 = 0x27;

// Command Table
const CLEAR: u8 = 0x01;
const HOME: u8 = 0x02;

// Clears the display and sets RAM address to 0
pub fn clear(i2c: &mut I2c) -> Result<(), Box<dyn Error>> {
    write_command(i2c, CLEAR)?;
    sleep(Duration::from_millis(2));
    Ok(())
}

// Returns the cursor to (0, 0)
pub fn home(i2c: &mut I2c) -> Result<(), Box<dyn Error>> {
    write_command(i2c, HOME)?;
    sleep(Duration::from_millis(2));
    Ok(())
}

// Moves the Display Data RAM Address, or where data is going to be written to
pub fn set_cursor(i2c: &mut I2c, col: u8, row: u8) -> Result<(), Box<dyn Error>> {
    let offsets = [0x00, 0x40];
    // 0x80 is set Display Data RAM Address
    // 1st row is 0x00 -> 0x27
    // 2nd row is 0x40 -> 0x67
    // Both of these are more characters than fit no screen, maybe used for scrolling???
    let cmd = 0x80 | (col + offsets[row as usize]);
    write_command(i2c, cmd)?;
    Ok(())
}

// Writes text to display RAM
pub fn print(i2c: &mut I2c, text: &str) -> Result<(), Box<dyn Error>> {
    for c in text.chars() {
        if c.is_ascii() {
            write_data(i2c, c as u8)?;
        } else {
            // Black Square, Error case
            println!("non ascii char");
            write_data(i2c, 0xFF)?;
        }
    }
    Ok(())
}

// Send initialize sequence from Data-Sheet
pub fn init(i2c: &mut I2c) -> Result<(), Box<dyn Error>> {
    // Takes 20ms to start, shouldn't be possible to get here faster than that but just in case
    sleep(Duration::from_millis(20));

    for _ in 0..3 {
        pulse_enable(i2c, 0x03, false)?;
        sleep(Duration::from_millis(5));
    }

    // Switch to 4 bit mode
    pulse_enable(i2c, 0x02, false)?;
    sleep(Duration::from_millis(5));

    // Function set: 0b0 0 1 DL N F X X
    // DL 1 -> 8bit data
    // DL 0 -> 4bit data
    // N 0 -> One-line display
    // N 1 -> Two-line display
    // F 0 -> 5x8 dots font
    // F 1 -> 5x10 dots font
    // Setting: 4bit data, 2 lines, 5x8 font
    write_command(i2c, 0x28)?;

    // Display off
    write_command(i2c, 0x08)?;

    // Clear display
    write_command(i2c, 0x01)?;
    sleep(Duration::from_millis(2));

    // Entry Mode: 0b0000 0 1 I/D S
    // I/D, I=1=Increment ; D=0=Decrement
    // S, S=1=Display Shift ; S=0=No Shift
    // Setting: Cursor increment, no shift
    write_command(i2c, 0x06)?;

    // Display On/OFF: 0b0000 1DCB
    // D, D=1=Display On ; D=0=Display Off
    // C, C=1=Cursor On ; C=0=Cursor Off
    // B, B=1=Blinks on ; B=0=Blinks Off
    write_command(i2c, 0x0C)?;

    Ok(())
}

// Sends a nibble across the wire
// Data is latched on falling edge of Enable bit, which is why it is sent twice
fn pulse_enable(i2c: &mut I2c, nibble: u8, rs: bool) -> Result<(), Box<dyn Error>> {
    i2c.set_slave_address(ADDR)?;

    let byte_en1 = build_byte(nibble, rs, false, true);
    i2c.write(&[byte_en1])?;
    sleep(Duration::from_micros(1));

    let byte_en0 = build_byte(nibble, rs, false, false);
    i2c.write(&[byte_en0])?;
    sleep(Duration::from_micros(50));

    Ok(())
}

fn build_byte(nibble: u8, rs: bool, rw: bool, en: bool) -> u8 {
    let mut byte = (nibble & 0x0F) << 4;

    if rs {
        byte |= 1 << 0;
    }
    if rw {
        byte |= 1 << 1;
    }
    if en {
        byte |= 1 << 2;
    }

    // Backlight always on
    byte |= 1 << 3;

    byte
}

// Sends a byte across the wire
fn write_byte(i2c: &mut I2c, byte: u8, rs: bool) -> Result<(), Box<dyn Error>> {
    let high = byte >> 4;
    let low = byte & 0x0F;

    pulse_enable(i2c, high, rs)?;
    pulse_enable(i2c, low, rs)?;

    Ok(())
}

// Sends a byte with RS=true(data)
fn write_data(i2c: &mut I2c, byte: u8) -> Result<(), Box<dyn Error>> {
    write_byte(i2c, byte, true)
}

// Sends a byte with RS=false(cmd)
fn write_command(i2c: &mut I2c, byte: u8) -> Result<(), Box<dyn Error>> {
    write_byte(i2c, byte, false)
}

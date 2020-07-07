//reference https://os.phil-opp.com/vga-text-mode/
//Because of the repr(u8) attribute each enum variant is stored as an u8

//VGA buffer is 25 rows and 80 columns
//need 2 bytes 
// - first bytes represents character
// - 2nd byte defines how character is displayed
use volatile::Volatile;
use core::fmt;
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)] //https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent
struct ColorCode(u8); // this is a tupule struct without named fields

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

//now structure for screen character
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)] // field ordering is not guranteed in Rust, so we use this to define like in C
//gurantees fields are laid out in correct order
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer, //static lifetime; lasts for whole program
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe), //For unprintable bytes, 
                    // we print a ■ character, which has the hex code 0xfe on the VGA hardware
            }

        }
    }

    fn new_line(&mut self) {/* TODO */}
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}


use super::io;
use super::traits::ToString;
#[derive(Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    OK = 0,
    ERROR = 1,
    INFO = 2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub column_position: usize,
    pub row_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[self.row_position][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
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
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_number<T: ToString>(&mut self, n: T) {
        self.write_string(n.to_string().as_str());
    }

    pub fn new_line(&mut self) {
        // shift all lines up if on the last line
        // otherwise, move to the next line
        // and reset the column position
        if BUFFER_HEIGHT - 1 == self.row_position {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col];
                    self.buffer.chars[row - 1][col] = character;
                }
            }
            self.clear_row(self.row_position);
        } else {
            self.row_position += 1;
            self.column_position = 0;
        }
    }

    pub fn set_color(&mut self, bg: Color, fg: Color) {
        self.color_code = ColorCode::new(fg, bg);
    }

    pub fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            };
        }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                };
            }
        }
    }

    pub fn println(&mut self, message: &str) {
        self.write_string(message);
        self.new_line();
    }

    pub fn print_status(&mut self, status: Status, message: &str) {
        self.set_color(Color::Black, Color::White);
        self.write_string("[");
        match status {
            Status::OK => {
                self.set_color(Color::Black, Color::Green);
                self.write_string("OK");
            }
            Status::ERROR => {
                self.set_color(Color::Black, Color::LightRed);
                self.write_string("ERROR");
            }
            Status::INFO => {
                self.set_color(Color::Black, Color::LightBlue);
                self.write_string("INFO");
            }
        }
        self.set_color(Color::Black, Color::White);
        self.write_string("] ");
        self.write_string(message);
        self.new_line();
        // self.update_cursor();
    }

    #[expect(dead_code)]
    pub fn set_cursor(&mut self, row: usize, column: usize) {
        io::outb(0x3D4, 0x0F);
        io::outb(0x3D5, (row * BUFFER_WIDTH + column) as u8);
        io::outb(0x3D4, 0x0E);
        io::outb(0x3D5, ((row * BUFFER_WIDTH + column) >> 8) as u8);
    }

    #[expect(dead_code)]
    pub fn update_cursor(&mut self) {
        self.set_cursor(self.row_position, self.column_position);
    }
}

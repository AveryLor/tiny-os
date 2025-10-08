use core::fmt;
use volatile::Volatile;
use spin::Mutex;
use core::ptr::Unique;


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
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


#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
struct ColorCode(u8);


impl ColorCode {
fn new(fg: Color, bg: Color) -> ColorCode {
ColorCode((bg as u8) << 4 | (fg as u8))
}
}


#[repr(C)]
#[derive(Clone, Copy)]
struct ScreenChar {
ascii_character: u8,
color_code: ColorCode,
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
buffer: Unique<Buffer>,
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
unsafe {
self.buffer.as_ptr().as_mut().unwrap().chars[row][col].write(ScreenChar {
ascii_character: byte,
color_code,
});
}
self.column_position += 1;
}
}
}


fn new_line(&mut self) {
for row in 1..BUFFER_HEIGHT {
for col in 0..BUFFER_WIDTH {
unsafe {
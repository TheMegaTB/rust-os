use spin::Mutex;
use core::ptr::Unique;

pub mod color;
pub mod char;
pub mod writer;

pub static WRITER: Mutex<writer::Writer> = Mutex::new(writer::Writer {
    column_position: 0,
    color_code: color::ColorCode::new(color::Color::LightGreen, color::Color::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});

#[allow(unused_macros)]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

use core::fmt;
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[allow(unused_macros)]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[allow(dead_code)]
pub fn clear_screen() {
    for _ in 0..writer::BUFFER_HEIGHT {
        println!("");
    }
}

pub use font::*;

pub const GREEN: u32 = 0x33ee55;
pub const BLUE: u32 = 0x3377ee;
pub const RED: u32 = 0xee3377;
pub const YELLOW: u32 = 0xddcc11;
pub const WHITE: u32 = 0xffffff;

pub struct GPrinter<'buf> {
    buffer: &'buf mut [u32],
    width: usize,
}

impl<'buf> GPrinter<'buf> {
    pub fn new(buffer: &'buf mut [u32], width: usize) -> Self {
        GPrinter { buffer, width }
    }

    /// Note: Only supports Ascii
    /// TODO: Implement bound cheching
    pub fn write(&mut self, ox: usize, oy: usize, colour: u32, text: &str) {
        for (index, byte) in text.bytes().enumerate() {
            for x in 0..FONT_WIDTH {
                let col = FONT[byte as usize][x as usize];
                for y in 0..FONT_HEIGHT {
                    if let Some(c) = self.buffer.get_mut(
                        (oy + y) * self.width + (index * FONT_WIDTH) +
                            (ox + x),
                    )
                    {
                        *c = if col & (1 << y) != 0 { colour } else { 0 };
                    }
                }
            }
        }
    }
}

macro_rules! gprint {
    ( $buf:expr, $x:expr, $y:expr, $colour:expr, $fmt:expr ) => {
        GPrinter::write($buf, $x, $y, $colour, &format!($fmt))
    };
    ( $buf:expr, $x:expr, $y:expr, $colour:expr, $fmt:expr, $($arg:expr),* ) => {
        GPrinter::write($buf, $x, $y, $colour, &format!($fmt, $($arg),*))
    };
}

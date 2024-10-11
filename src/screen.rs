#[derive(Debug, Clone)]
enum PixelSet {
    PixelOn,
    PixelOff,
}
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug)]
pub struct Screen {
    width: u32,
    height: u32,
    color: [u8; 4],
    screen: Vec<PixelSet>, // pixels is a flat array, with 4 values for color
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        Screen {
            width: width,
            height: height,
            color: [0xFF, 0xFF, 0xFF, 0xFF],
            screen: vec![PixelSet::PixelOff; (width * height) as usize],
        }
    }

    // returns true if pixel overlaps
    pub fn set_pixel(&mut self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            panic!("TRYING TO SET PIXEL OUT OF SCREEN RANGE");
        }
        let pos = (y * self.width) + x;
        match self.screen[pos as usize] {
            PixelSet::PixelOff => {
                self.screen[pos as usize] = PixelSet::PixelOn;
                return false;
            }
            PixelSet::PixelOn => {
                self.screen[pos as usize] = PixelSet::PixelOff;
                return true;
            }
        }
    }

    // Take a byte and a starting (x, y) position and draw the pixels to the screen.
    // If a sprite goes off screen, it is wrapped around to the other side.
    // Returns true if pixel overlap.
    pub fn set_byte_pixels(&mut self, byte: u8, x: u32, y: u32) -> bool {
        let mut overlap: bool = false;
        for i in 0..8 {
            // if bit is 1, set pixel.
            // if overlap from setting pixel, set overlap to true
            if ((byte << i) & 0x80) == 0x80 {
                if self.set_pixel((x + i) % self.width, y % self.height) {
                    overlap = true;
                }
            }
        }
        overlap
    }

    pub fn clear_screen(&mut self) {
        self.screen
            .iter_mut()
            .for_each(|item| *item = PixelSet::PixelOff);
    }

    pub fn screen_to_render(&self) -> Vec<[u8; 4]> {
        self.screen
            .iter()
            .map(|item| match item {
                PixelSet::PixelOn => self.color,
                PixelSet::PixelOff => [0, 0, 0, 0],
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_screen_output() {
        let mut s = Screen::new(2, 2);

        s.set_pixel(1, 1);
        let output = s.screen_to_render();
        let comparison: Vec<[u8; 4]> = vec![[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], s.color];
        assert_eq!(output, comparison);
    }

    #[test]
    fn render_screen() {
        let mut s = Screen::new(8, 8);
        let c = s.color;
        let z: [u8; 4] = [0, 0, 0, 0];
        s.screen[0] = PixelSet::PixelOn;
        s.screen[9] = PixelSet::PixelOn;
        s.screen[10] = PixelSet::PixelOn;

        let output = s.screen_to_render();
        let mut comparison: Vec<[u8; 4]> = vec![z; 8 * 8];
        comparison[0] = c;
        comparison[9] = c;
        comparison[10] = c;
        assert_eq!(output, comparison);
    }

    #[test]
    fn test_clear_screen() {
        let mut s = Screen::new(2, 2);
        let c = s.color;
        s.set_pixel(1, 1);
        s.set_pixel(0, 0);
        let output = s.screen_to_render();
        let comparison: Vec<[u8; 4]> = vec![c, [0; 4], [0; 4], c];
        assert_eq!(output, comparison);
        s.clear_screen();
        let output = s.screen_to_render();
        let comparison: Vec<[u8; 4]> = vec![[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        assert_eq!(output, comparison);
    }

    #[test]
    fn test_byte_set() {
        let mut s = Screen::new(8, 2);
        let c = s.color;
        let z: [u8; 4] = [0, 0, 0, 0];
        let byte_to_set = 0xFF;
        s.set_byte_pixels(byte_to_set, 0, 0);
        let output = s.screen_to_render();
        let comparison: Vec<[u8; 4]> = vec![c, c, c, c, c, c, c, c, z, z, z, z, z, z, z, z];
        assert_eq!(output, comparison);
    }
}

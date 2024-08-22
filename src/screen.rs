#[derive(Debug, Clone)]
enum PixelSet {
    PixelOn,
    PixelOff
}

#[derive(Debug)]
pub struct Screen {
    width: u8,
    height: u8,
    color: [u8; 4],
    screen: Vec<PixelSet> // pixels is a flat array, with 4 values for color
}

impl Screen {
    pub fn new(width: u8, height: u8) -> Self {
        Screen {
            width: width,
            height: height,
            color: [0x1F, 0x1F, 0x1F, 0x1F],
            screen: vec![PixelSet::PixelOff; (width as u32 * height as u32) as usize],
        }
    }

    // returns true if pixel overlaps
    pub fn set_pixel(&mut self, x: u8, y: u8) -> bool {
        if x >= self.width || y >= self.height {
            panic!("TRYING TO SET PIXEL OUT OF SCREEN RANGE");
            return false;
        }
        let pos = (y * self.width) + x;
        match self.screen[pos as usize] {
            PixelSet::PixelOff => {
                self.screen[pos as usize] = PixelSet::PixelOn;
                return false;
            },
            PixelSet::PixelOn => {
                self.screen[pos as usize] = PixelSet::PixelOff;
                return true;
            }
        }
    }

    pub fn clear_screen(&mut self) {
        self.screen.iter_mut().for_each(|item| *item = PixelSet::PixelOff);
    }

    pub fn screen_to_render(&self) -> Vec<[u8; 4]> {
        let c = self.color;
        self.screen.iter().map(|item| {
            match item {
                PixelSet::PixelOn => self.color,
                PixelSet::PixelOff => [0, 0, 0, 0]
            }
        }).collect()
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



    }
}
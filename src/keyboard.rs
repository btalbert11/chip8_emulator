#[derive(Copy, Clone, Debug)]
pub enum Key {
    Up,
    Down,
}

pub struct Keyboard {
    keys: [Key; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [Key::Up; 16],
        }
    }

    pub fn is_key_down(&self, index: u8) -> bool {
        match self.keys[index as usize] {
            Key::Up => false,
            Key::Down => true,
        }
    }

    pub fn get_first_key_down(&self) -> Option<u8> {
        for (i, key) in self.keys.iter().enumerate() {
            match key {
                Key::Down => return Some(i as u8),
                _ => (),
            }
        }
        None
    }

    pub fn set_key(&mut self, index: u8, state: Key) {
        if index > 15 {
            panic!("Key index greater than 15, only 16 keys exist");
        }
        self.keys[index as usize] = state;
    }
}

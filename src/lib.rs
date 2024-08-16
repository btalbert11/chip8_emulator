
use std::fmt;
use rand::Rng;

use instruction::Instruction;

pub mod instruction;
pub mod emulator;


 pub struct Display {
     display: [[bool; 64]; 48]
 }
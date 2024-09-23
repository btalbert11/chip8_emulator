use chip8_emulator::run;



fn main() {
    pollster::block_on(run());

}

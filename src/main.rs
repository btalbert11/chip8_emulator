use chip8_emulator::run;
use chip8_emulator::init_loggers;
use chip8_emulator::emulator::Emulator;


fn main() {
    init_loggers();
    let e = Emulator::new();
    pollster::block_on(run(e));

}

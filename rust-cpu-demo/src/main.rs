/// A demo for emulating cpu specisically CHIP-8 used by first indie game programmer,
/// Joyce Weisbecker, who was a lady
fn main() {
    let mut cpu = CPU::new();

    cpu.current_operation = 0x8014; // opcode that the cpu is to interpret
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
}

struct CPU {
    current_operation: u16,
    registers: [u8; 2]
}

impl CPU {
    fn new() -> CPU {
        CPU {
            current_operation: 0,
            registers: [0; 2]
        }
    }
}

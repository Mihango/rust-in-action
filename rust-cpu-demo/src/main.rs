/// A demo for emulating cpu specisically CHIP-8 used by first indie game programmer,
/// Joyce Weisbecker, who was a lady
fn main() {
    let mut cpu = CPU::new();

    // 0x8014 indicates:
    // 8 signifies that the operation involves to registers
    // 0 maps to cpu.register[0]
    // 1 maps to cpu.register[1]
    // s indicates addition
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

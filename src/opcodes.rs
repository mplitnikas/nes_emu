use crate::cpu::Mem;
use crate::cpu::CPU;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect,
    Indirect_X,
    Indirect_Y,
    Relative,
    NoneAddressing,
}

pub struct OpCode {
    pub code: u8,
    pub name: &'static str,
    pub length: u16,
    pub cycles: usize,
    pub mode: AddressingMode,
}

impl OpCode {
    pub const fn new(
        code: u8,
        name: &'static str,
        length: u16,
        cycles: usize,
        mode: AddressingMode,
    ) -> Self {
        OpCode {
            code,
            name,
            length,
            cycles,
            mode,
        }
    }
}

pub fn format_instruction(cpu: &CPU) -> String {
    let opcode = CPU_OPCODES.get(&cpu.mem_read(cpu.program_counter)).unwrap();
    let name = opcode.name;
    let address = cpu.mem_read(cpu.program_counter + 1);
    let operand = match opcode.mode {
        AddressingMode::Immediate => format!("#${:02X}", address),
        AddressingMode::ZeroPage => {
            format!("${:02X} = {:02X}", address, cpu.mem_read(address as u16))
        }
        AddressingMode::ZeroPage_X => {
            format!("${:02X},X = {:02X}", address, cpu.mem_read(address as u16))
        }
        AddressingMode::ZeroPage_Y => {
            format!("${:02X},Y = {:02X}", address, cpu.mem_read(address as u16))
        }
        AddressingMode::Absolute => format!(
            "${:02X}{:02X}",
            cpu.mem_read(cpu.program_counter + 2),
            cpu.mem_read(cpu.program_counter + 1)
        ),
        AddressingMode::Absolute_X => format!(
            "${:02X}{:02X},X",
            cpu.mem_read(cpu.program_counter + 2),
            cpu.mem_read(cpu.program_counter + 1)
        ),
        AddressingMode::Absolute_Y => format!(
            "${:02X}{:02X},Y",
            cpu.mem_read(cpu.program_counter + 2),
            cpu.mem_read(cpu.program_counter + 1)
        ),
        AddressingMode::Indirect => {
            let op_addr = cpu.get_operand_address(&opcode.mode);
            format!(
                "(${:02X}) = {:04X} @ {:04X} = {:02X}",
                cpu.mem_read(cpu.program_counter + 1),
                op_addr,
                op_addr,
                cpu.mem_read(op_addr)
            )
        }
        AddressingMode::Indirect_X => {
            let op_addr = cpu.get_operand_address(&opcode.mode);
            format!(
                "(${:02X}),X = {:04X} @ {:04X} = {:02X}",
                cpu.mem_read(cpu.program_counter + 1),
                op_addr,
                op_addr,
                cpu.mem_read(op_addr)
            )
        }
        AddressingMode::Indirect_Y => {
            let op_addr = cpu.get_operand_address(&opcode.mode);
            format!(
                "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                cpu.mem_read(cpu.program_counter + 1),
                op_addr,
                op_addr,
                cpu.mem_read(op_addr)
            )
        }
        AddressingMode::Relative => {
            // +2 for opcode length
            let op_addr = cpu.get_operand_address(&opcode.mode) + 2;
            format!("${:04X}", op_addr)
        }
        AddressingMode::NoneAddressing => "".to_string(),
    };
    let full_opcode = (0..opcode.length)
        .map(|i| format!("{:02X}", cpu.mem_read(cpu.program_counter + i)))
        .collect::<Vec<String>>()
        .join(" ");
    format!("{:<8} {:>4} {:<26}", full_opcode, name, operand)
}

#[rustfmt::skip]
lazy_static! {
    pub static ref CPU_OPCODES: HashMap<u8, OpCode> = HashMap::from([
        (0x69, OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate)),
        (0x65, OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage)),
        (0x75, OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X)),
        (0x6D, OpCode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute)),
        (0x7D, OpCode::new(0x7D, "ADC", 3, 4, AddressingMode::Absolute_X)), // +1
        (0x79, OpCode::new(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0x61, OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X)),
        (0x71, OpCode::new(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0x29, OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate)),
        (0x25, OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage)),
        (0x35, OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X)),
        (0x2D, OpCode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute)),
        (0x3D, OpCode::new(0x3D, "AND", 3, 4, AddressingMode::Absolute_X)), // +1
        (0x39, OpCode::new(0x39, "AND", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0x21, OpCode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X)),
        (0x31, OpCode::new(0x31, "AND", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0x0A, OpCode::new(0x0A, "ASL", 1, 2, AddressingMode::NoneAddressing)),
        (0x06, OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage)),
        (0x16, OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X)),
        (0x0E, OpCode::new(0x0E, "ASL", 3, 6, AddressingMode::Absolute)),
        (0x1E, OpCode::new(0x1E, "ASL", 3, 7, AddressingMode::Absolute_X)), // +1

        (0x24, OpCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage)),
        (0x2C, OpCode::new(0x2C, "BIT", 3, 4, AddressingMode::Absolute)),

        // A branch not taken requires two machine cycles.
        // Add one if the branch is taken and add one more if the branch crosses a page boundary.
        (0x10, OpCode::new(0x10, "BPL", 2, 2, AddressingMode::Relative)),
        (0x30, OpCode::new(0x30, "BMI", 2, 2, AddressingMode::Relative)),
        (0x50, OpCode::new(0x50, "BVC", 2, 3, AddressingMode::Relative)),
        (0x70, OpCode::new(0x70, "BVS", 2, 2, AddressingMode::Relative)),
        (0x90, OpCode::new(0x90, "BCC", 2, 2, AddressingMode::Relative)),
        (0xB0, OpCode::new(0xB0, "BCS", 2, 2, AddressingMode::Relative)),
        (0xD0, OpCode::new(0xD0, "BNE", 2, 2, AddressingMode::Relative)),
        (0xF0, OpCode::new(0xF0, "BEQ", 2, 2, AddressingMode::Relative)),

        (0x00, OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing)),

        (0xC9, OpCode::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate)),
        (0xC5, OpCode::new(0xC5, "CMP", 2, 3, AddressingMode::ZeroPage)),
        (0xD5, OpCode::new(0xD5, "CMP", 2, 4, AddressingMode::ZeroPage_X)),
        (0xCD, OpCode::new(0xCD, "CMP", 3, 4, AddressingMode::Absolute)),
        (0xDD, OpCode::new(0xDD, "CMP", 3, 4, AddressingMode::Absolute_X)), // +1
        (0xD9, OpCode::new(0xD9, "CMP", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0xC1, OpCode::new(0xC1, "CMP", 2, 6, AddressingMode::Indirect_X)),
        (0xD1, OpCode::new(0xD1, "CMP", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0xE0, OpCode::new(0xE0, "CPX", 2, 2, AddressingMode::Immediate)),
        (0xE4, OpCode::new(0xE4, "CPX", 2, 3, AddressingMode::ZeroPage)),
        (0xEC, OpCode::new(0xEC, "CPX", 3, 4, AddressingMode::Absolute)),

        (0xC0, OpCode::new(0xC0, "CPY", 2, 2, AddressingMode::Immediate)),
        (0xC4, OpCode::new(0xC4, "CPY", 2, 3, AddressingMode::ZeroPage)),
        (0xCC, OpCode::new(0xCC, "CPY", 3, 4, AddressingMode::Absolute)),

        (0xC6, OpCode::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage)),
        (0xD6, OpCode::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPage_X)),
        (0xCE, OpCode::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute)),
        (0xDE, OpCode::new(0xDE, "DEC", 3, 7, AddressingMode::Absolute_X)),

        (0x49, OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate)),
        (0x45, OpCode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage)),
        (0x55, OpCode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPage_X)),
        (0x4D, OpCode::new(0x4D, "EOR", 3, 4, AddressingMode::Absolute)),
        (0x5D, OpCode::new(0x5D, "EOR", 3, 4, AddressingMode::Absolute_X)), // +1
        (0x59, OpCode::new(0x59, "EOR", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0x41, OpCode::new(0x41, "EOR", 2, 6, AddressingMode::Indirect_X)),
        (0x51, OpCode::new(0x51, "EOR", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0x18, OpCode::new(0x18, "CLC", 1, 2, AddressingMode::NoneAddressing)),
        (0x38, OpCode::new(0x38, "SEC", 1, 2, AddressingMode::NoneAddressing)),
        (0x58, OpCode::new(0x58, "CLI", 1, 2, AddressingMode::NoneAddressing)),
        (0x78, OpCode::new(0x78, "SEI", 1, 2, AddressingMode::NoneAddressing)),
        (0xB8, OpCode::new(0xB8, "CLV", 1, 2, AddressingMode::NoneAddressing)),
        (0xD8, OpCode::new(0xD8, "CLD", 1, 2, AddressingMode::NoneAddressing)),
        (0xF8, OpCode::new(0xF8, "SED", 1, 2, AddressingMode::NoneAddressing)),

        (0xE6, OpCode::new(0xE6, "INC", 2, 5, AddressingMode::ZeroPage)),
        (0xF6, OpCode::new(0xF6, "INC", 2, 6, AddressingMode::ZeroPage_X)),
        (0xEE, OpCode::new(0xEE, "INC", 3, 6, AddressingMode::Absolute)),
        (0xFE, OpCode::new(0xFE, "INC", 3, 7, AddressingMode::Absolute_X)),

        (0x4C, OpCode::new(0x4C, "JMP", 3, 3, AddressingMode::Absolute)),
        (0x6C, OpCode::new(0x6C, "JMP", 3, 5, AddressingMode::Indirect)),

        (0x20, OpCode::new(0x20, "JSR", 3, 6, AddressingMode::Absolute)),

        (0xA9, OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate)),
        (0xA5, OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage)),
        (0xB5, OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X)),
        (0xAD, OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute)),
        (0xBD, OpCode::new(0xBD, "LDA", 3, 4, AddressingMode::Absolute_X)), // +1 if page crossed
        (0xB9, OpCode::new(0xB9, "LDA", 3, 4, AddressingMode::Absolute_Y)), // +1 if page crossed
        (0xA1, OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::Indirect_X)),
        (0xB1, OpCode::new(0xB1, "LDA", 2, 5, AddressingMode::Indirect_Y)), // +1 if page crossed

        (0xA2, OpCode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate)),
        (0xA6, OpCode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage)),
        (0xB6, OpCode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPage_Y)),
        (0xAE, OpCode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute)),
        (0xBE, OpCode::new(0xBE, "LDX", 3, 4, AddressingMode::Absolute_Y)), // +1

        (0xA0, OpCode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate)),
        (0xA4, OpCode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage)),
        (0xB4, OpCode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPage_X)),
        (0xAC, OpCode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute)),
        (0xBC, OpCode::new(0xBC, "LDY", 3, 4, AddressingMode::Absolute_X)), // +1

        (0x4A, OpCode::new(0x4A, "LSR", 1, 2, AddressingMode::NoneAddressing)),
        (0x46, OpCode::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage)),
        (0x56, OpCode::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPage_X)),
        (0x4E, OpCode::new(0x4E, "LSR", 3, 6, AddressingMode::Absolute)),
        (0x5E, OpCode::new(0x5E, "LSR", 3, 7, AddressingMode::Absolute_X)),

        (0xEA, OpCode::new(0xEA, "NOP", 1, 2, AddressingMode::NoneAddressing)),

        (0x09, OpCode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate)),
        (0x05, OpCode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage)),
        (0x15, OpCode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPage_X)),
        (0x0D, OpCode::new(0x0D, "ORA", 3, 4, AddressingMode::Absolute)),
        (0x1D, OpCode::new(0x1D, "ORA", 3, 4, AddressingMode::Absolute_X)), // +1
        (0x19, OpCode::new(0x19, "ORA", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0x01, OpCode::new(0x01, "ORA", 2, 6, AddressingMode::Indirect_X)),
        (0x11, OpCode::new(0x11, "ORA", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0xAA, OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing)),
        (0x8A, OpCode::new(0x8A, "TXA", 1, 2, AddressingMode::NoneAddressing)),
        (0xCA, OpCode::new(0xCA, "DEX", 1, 2, AddressingMode::NoneAddressing)),
        (0xE8, OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing)),
        (0xA8, OpCode::new(0xA8, "TAY", 1, 2, AddressingMode::NoneAddressing)),
        (0x98, OpCode::new(0x98, "TYA", 1, 2, AddressingMode::NoneAddressing)),
        (0x88, OpCode::new(0x88, "DEY", 1, 2, AddressingMode::NoneAddressing)),
        (0xC8, OpCode::new(0xC8, "INY", 1, 2, AddressingMode::NoneAddressing)),

        (0x2A, OpCode::new(0x2A, "ROL", 1, 2, AddressingMode::NoneAddressing)),
        (0x26, OpCode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage)),
        (0x36, OpCode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPage_X)),
        (0x2E, OpCode::new(0x2E, "ROL", 3, 6, AddressingMode::Absolute)),
        (0x3E, OpCode::new(0x3E, "ROL", 3, 7, AddressingMode::Absolute_X)),

        (0x6A, OpCode::new(0x6A, "ROR", 1, 2, AddressingMode::NoneAddressing)),
        (0x66, OpCode::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage)),
        (0x76, OpCode::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPage_X)),
        (0x6E, OpCode::new(0x6E, "ROR", 3, 6, AddressingMode::Absolute)),
        (0x7E, OpCode::new(0x7E, "ROR", 3, 7, AddressingMode::Absolute_X)),

        (0x40, OpCode::new(0x40, "RTI", 1, 6, AddressingMode::NoneAddressing)),

        (0x60, OpCode::new(0x60, "RTS", 1, 6, AddressingMode::NoneAddressing)),

        (0xE9, OpCode::new(0xE9, "SBC", 2, 2, AddressingMode::Immediate)),
        (0xE5, OpCode::new(0xE5, "SBC", 2, 3, AddressingMode::ZeroPage)),
        (0xF5, OpCode::new(0xF5, "SBC", 2, 4, AddressingMode::ZeroPage_X)),
        (0xED, OpCode::new(0xED, "SBC", 3, 4, AddressingMode::Absolute)),
        (0xFD, OpCode::new(0xFD, "SBC", 3, 4, AddressingMode::Absolute_X)), // +1
        (0xF9, OpCode::new(0xF9, "SBC", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0xE1, OpCode::new(0xE1, "SBC", 2, 6, AddressingMode::Indirect_X)),
        (0xF1, OpCode::new(0xF1, "SBC", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0x85, OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage)),
        (0x95, OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X)),
        (0x8D, OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute)),
        (0x9D, OpCode::new(0x9D, "STA", 3, 5, AddressingMode::Absolute_X)),
        (0x99, OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y)),
        (0x81, OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X)),
        (0x91, OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y)),

        (0x9A, OpCode::new(0x9A, "TXS", 1, 2, AddressingMode::NoneAddressing)),
        (0xBA, OpCode::new(0xBA, "TSX", 1, 2, AddressingMode::NoneAddressing)),
        (0x48, OpCode::new(0x48, "PHA", 1, 3, AddressingMode::NoneAddressing)),
        (0x68, OpCode::new(0x68, "PLA", 1, 4, AddressingMode::NoneAddressing)),
        (0x08, OpCode::new(0x08, "PHP", 1, 3, AddressingMode::NoneAddressing)),
        (0x28, OpCode::new(0x28, "PLP", 1, 4, AddressingMode::NoneAddressing)),

        (0x86, OpCode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage)),
        (0x96, OpCode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPage_Y)),
        (0x8E, OpCode::new(0x8E, "STX", 3, 4, AddressingMode::Absolute)),

        (0x84, OpCode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage)),
        (0x94, OpCode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPage_X)),
        (0x8C, OpCode::new(0x8C, "STY", 3, 4, AddressingMode::Absolute)),
    ]);
}

use lazy_static::lazy_static;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

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
    NoneAddressing,
}

pub struct OpCode {
    code: u8,
    name: &'static str,
    length: u16,
    cycles: usize,
    mode: AddressingMode,
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
        (0x10, OpCode::new(0x10, "BPL", 2, 2, AddressingMode::Immediate)),
        (0x30, OpCode::new(0x30, "BMI", 2, 2, AddressingMode::Immediate)),
        (0x50, OpCode::new(0x50, "BVC", 2, 3, AddressingMode::Immediate)),
        (0x70, OpCode::new(0x70, "BVS", 2, 2, AddressingMode::Immediate)),
        (0x90, OpCode::new(0x90, "BCC", 2, 2, AddressingMode::Immediate)),
        (0xB0, OpCode::new(0xB0, "BCS", 2, 2, AddressingMode::Immediate)),
        (0xD0, OpCode::new(0xD0, "BNE", 2, 2, AddressingMode::Immediate)),
        (0xF0, OpCode::new(0xF0, "BEQ", 2, 2, AddressingMode::Immediate)),

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

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0b0010_0000, // NV1B_DIZC
            program_counter: 0,
            stack_pointer: 0xFF,
            memory: [0; 0xFFFF],
        }
    }

    pub const STACK_ADDRESS: u16 = 0x0100;

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter + 1,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter + 1) as u16,
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter + 1);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter + 1);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter + 1),
            AddressingMode::Absolute_X => {
                let pos = self.mem_read_u16(self.program_counter + 1);
                let addr = pos.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let pos = self.mem_read_u16(self.program_counter + 1);
                let addr = pos.wrapping_add(self.register_y as u16);
                addr
            }
            AddressingMode::Indirect => {
                let ptr = self.mem_read(self.program_counter + 1);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter + 1);
                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter + 1);
                let ptr: u8 = base.wrapping_add(self.register_y);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::NoneAddressing => 0,
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        hi << 8 | lo
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFFFF) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
        self.stack_pointer = 0xFF;
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn add(&mut self, value: u8) -> u8 {
        let result = self.register_a as u16 + value as u16 + (self.status & 0b0000_0001) as u16;
        self.set_carry_flag(result > 0xFF);
        let result = result as u8;
        self.set_overflow_flag((self.register_a ^ result) & (value ^ result) & 0b1000_0000 != 0);
        result
    }

    pub fn push_to_stack(&mut self, value: u8) {
        self.mem_write(CPU::STACK_ADDRESS + self.stack_pointer as u16, value);
        self.stack_pointer -= 1;
    }

    pub fn pull_from_stack(&mut self) -> u8 {
        let value = self.mem_read(CPU::STACK_ADDRESS + self.stack_pointer as u16);
        self.stack_pointer += 1;
        value
    }

    // instructions

    fn adc(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        let result = self.add(value);

        self.register_a = result;
        self.update_zero_and_negative_flags(result);
        self.program_counter += opcode.length;
    }

    fn and(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.register_a &= value;
        self.update_zero_and_negative_flags(self.register_a);
        self.program_counter += opcode.length;
    }

    fn asl(&mut self, opcode: &OpCode) {
        if opcode.mode == AddressingMode::NoneAddressing {
            if self.register_a & 0b1000_0000 != 0 {
                self.set_carry_flag(true);
            } else {
                self.set_carry_flag(false);
            }
            self.register_a = self.register_a << 1;
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(&opcode.mode);
            let value = self.mem_read(addr);

            if value & 0b1000_0000 != 0 {
                self.set_carry_flag(true);
            } else {
                self.set_carry_flag(false);
            }
            let result = value << 1;
            self.mem_write(addr, result);
            self.update_zero_and_negative_flags(result);
        }
        self.program_counter += opcode.length;
    }

    fn bcc(&mut self, opcode: &OpCode) {
        self.branch(opcode, self.status & 0b0000_0001 == 0);
    }

    fn bcs(&mut self, opcode: &OpCode) {
        self.branch(opcode, self.status & 0b0000_0001 != 0);
    }

    fn beq(&mut self, opcode: &OpCode) {
        self.branch(opcode, self.status & 0b0000_0010 != 0);
    }

    fn bit(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        let result = self.register_a & value;
        self.update_zero_and_negative_flags(result);
        self.status |= value & 0b1100_0000;
        self.status &= value | 0b1100_0000;
    }

    fn bmi(&mut self, opcode: &OpCode) {
        self.branch(opcode, self.status & 0b1000_0000 != 0);
    }

    fn bne(&mut self, opcode: &OpCode) {
        self.branch(opcode, self.status & 0b0000_0010 == 0);
    }

    fn bpl(&mut self, opcode: &OpCode) {
        self.branch(opcode, self.status & 0b1000_0000 == 0);
    }

    fn brk(&mut self, opcode: &OpCode) {
        self.program_counter += opcode.length;
        self.set_break_flag(true);
        // TODO: trigger interrupt
    }

    fn bvc(&mut self, opcode: &OpCode) {
        self.branch(opcode, self.status & 0b0100_0000 == 0);
    }

    fn bvs(&mut self, opcode: &OpCode) {
        self.branch(opcode, self.status & 0b0100_0000 != 0);
    }

    fn clc(&mut self, opcode: &OpCode) {
        self.set_carry_flag(false);
        self.program_counter += opcode.length;
    }

    fn cld(&mut self, opcode: &OpCode) {
        self.set_decimal_flag(false);
        self.program_counter += opcode.length;
    }

    fn cli(&mut self, opcode: &OpCode) {
        self.set_interrupt_flag(false);
        self.program_counter += opcode.length;
    }

    fn clv(&mut self, opcode: &OpCode) {
        self.set_overflow_flag(false);
        self.program_counter += opcode.length;
    }

    fn cmp(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        if self.register_a >= value {
            self.set_carry_flag(true);
        } else {
            self.set_carry_flag(false);
        }

        // get result for setting flags, but don't use it
        let result = self.register_a.wrapping_sub(value);
        self.update_zero_and_negative_flags(result);

        self.program_counter += opcode.length;
    }

    fn cpx(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        if self.register_x >= value {
            self.set_carry_flag(true);
        } else {
            self.set_carry_flag(false);
        }

        // get result for setting flags, but don't use it
        let result = self.register_x.wrapping_sub(value);
        self.update_zero_and_negative_flags(result);

        self.program_counter += opcode.length;
    }

    fn cpy(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        if self.register_y >= value {
            self.set_carry_flag(true);
        } else {
            self.set_carry_flag(false);
        }

        // get result for setting flags, but don't use it
        let result = self.register_y.wrapping_sub(value);
        self.update_zero_and_negative_flags(result);

        self.program_counter += opcode.length;
    }

    fn dec(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.mem_write(addr, value.wrapping_sub(1));
        self.update_zero_and_negative_flags(value);
        self.program_counter += opcode.length;
    }

    fn dex(&mut self, opcode: &OpCode) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
        self.program_counter += opcode.length;
    }

    fn dey(&mut self, opcode: &OpCode) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
        self.program_counter += opcode.length;
    }

    fn eor(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.register_a ^= value;
        self.update_zero_and_negative_flags(self.register_a);
        self.program_counter += opcode.length;
    }

    fn inc(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.mem_write(addr, value.wrapping_add(1));
        self.update_zero_and_negative_flags(value);
        self.program_counter += opcode.length;
    }

    fn inx(&mut self, opcode: &OpCode) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
        self.program_counter += opcode.length;
    }

    fn iny(&mut self, opcode: &OpCode) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
        self.program_counter += opcode.length;
    }

    fn jmp(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.program_counter = value as u16;
    }

    fn jsr(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn lda(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
        self.program_counter += &opcode.length;
    }

    fn ldx(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
        self.program_counter += &opcode.length;
    }

    fn ldy(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
        self.program_counter += &opcode.length;
    }

    fn lsr(&mut self, opcode: &OpCode) {
        if opcode.mode == AddressingMode::NoneAddressing {
            if self.register_a & 0b0000_0001 != 0 {
                self.set_carry_flag(true);
            } else {
                self.set_carry_flag(false);
            }
            self.register_a = self.register_a >> 1;
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(&opcode.mode);
            let value = self.mem_read(addr);

            if value & 0b0000_0001 != 0 {
                self.set_carry_flag(true);
            } else {
                self.set_carry_flag(false);
            }
            let result = value >> 1;
            self.mem_write(addr, result);
            self.update_zero_and_negative_flags(result);
        }
        self.program_counter += opcode.length;
    }

    fn nop(&mut self, opcode: &OpCode) {
        self.program_counter += opcode.length;
    }

    fn ora(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.register_a |= value;
        self.update_zero_and_negative_flags(self.register_a);
        self.program_counter += opcode.length;
    }

    fn pha(&mut self, opcode: &OpCode) {
        self.push_to_stack(self.register_a);
        self.program_counter += opcode.length;
    }

    fn php(&mut self, opcode: &OpCode) {
        self.push_to_stack(self.status);
        self.program_counter += opcode.length;
    }

    fn pla(&mut self, opcode: &OpCode) {
        self.register_a = self.pull_from_stack();
        self.program_counter += opcode.length;
    }

    fn plp(&mut self, opcode: &OpCode) {
        self.status = self.pull_from_stack();
        self.program_counter += opcode.length;
    }

    fn rol(&mut self, opcode: &OpCode) {
        if opcode.mode == AddressingMode::NoneAddressing {
            let carry = self.register_a & 0b1000_0000 != 0;
            self.register_a = self.register_a << 1;
            self.register_a |= self.status & 0b0000_0001;
            self.set_carry_flag(carry);
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(&opcode.mode);
            let value = self.mem_read(addr);

            let carry = value & 0b1000_0000 != 0;
            let mut result = value << 1;
            result |= self.status & 0b0000_0001;
            self.mem_write(addr, result);
            self.set_carry_flag(carry);
            self.update_zero_and_negative_flags(result);
        }
        self.program_counter += opcode.length;
    }

    fn ror(&mut self, opcode: &OpCode) {
        if opcode.mode == AddressingMode::NoneAddressing {
            let carry = self.register_a & 0b0000_0001 != 0;
            self.register_a = self.register_a >> 1;
            self.register_a |= (self.status & 0b0000_0001) << 7;
            self.set_carry_flag(carry);
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(&opcode.mode);
            let value = self.mem_read(addr);

            let carry = value & 0b0000_0001 != 0;
            let mut result = value >> 1;
            result |= (self.status & 0b0000_0001) << 7;
            self.mem_write(addr, result);
            self.set_carry_flag(carry);
            self.update_zero_and_negative_flags(result);
        }
        self.program_counter += opcode.length;
    }

    fn rti(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn rts(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn sbc(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        let subtrahend = !value + 1;
        let result = self.add(subtrahend);

        self.register_a = result;
        self.update_zero_and_negative_flags(result);
        self.program_counter += opcode.length;
    }

    fn sec(&mut self, opcode: &OpCode) {
        self.set_carry_flag(true);
        self.program_counter += opcode.length;
    }

    fn sed(&mut self, opcode: &OpCode) {
        self.set_decimal_flag(true);
        self.program_counter += opcode.length;
    }

    fn sei(&mut self, opcode: &OpCode) {
        self.set_interrupt_flag(true);
        self.program_counter += opcode.length;
    }

    fn sta(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        self.mem_write(addr, self.register_a);
        self.program_counter += opcode.length;
    }

    fn stx(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        self.mem_write(addr, self.register_x);
        self.program_counter += opcode.length;
    }

    fn sty(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        self.mem_write(addr, self.register_y);
        self.program_counter += opcode.length;
    }

    fn tax(&mut self, opcode: &OpCode) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
        self.program_counter += opcode.length;
    }

    fn tay(&mut self, opcode: &OpCode) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
        self.program_counter += opcode.length;
    }

    fn tsx(&mut self, opcode: &OpCode) {
        self.register_x = self.pull_from_stack();
        self.stack_pointer -= 1; // undo stack pointer change
        self.program_counter += opcode.length;
    }

    fn txa(&mut self, opcode: &OpCode) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
        self.program_counter += opcode.length;
    }

    fn txs(&mut self, opcode: &OpCode) {
        self.push_to_stack(self.register_x);
        self.stack_pointer += 1; // undo stack pointer change
        self.program_counter += opcode.length;
    }

    fn tya(&mut self, opcode: &OpCode) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
        self.program_counter += opcode.length;
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // set Z flag
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
        // set N flag
        if result & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    fn set_carry_flag(&mut self, carry: bool) {
        if carry {
            self.status |= 0b0000_0001;
        } else {
            self.status &= 0b1111_1110;
        }
    }

    fn set_overflow_flag(&mut self, overflow: bool) {
        if overflow {
            self.status |= 0b0100_0000;
        } else {
            self.status &= 0b1011_1111;
        }
    }

    fn set_break_flag(&mut self, break_flag: bool) {
        if break_flag {
            self.status |= 0b0001_0000;
        } else {
            self.status &= 0b1110_1111;
        }
    }

    fn set_interrupt_flag(&mut self, interrupt: bool) {
        if interrupt {
            self.status |= 0b0000_0100;
        } else {
            self.status &= 0b1111_1011;
        }
    }

    fn set_decimal_flag(&mut self, decimal: bool) {
        if decimal {
            self.status |= 0b0000_1000;
        } else {
            self.status &= 0b1111_0111;
        }
    }

    fn branch(&mut self, opcode: &OpCode, conditional: bool) {
        if conditional {
            let addr = self.get_operand_address(&opcode.mode);
            let value = self.mem_read(addr);
            self.program_counter = self.program_counter.wrapping_add(value as i8 as i16 as u16);
        }
        self.program_counter += opcode.length;
    }

    pub fn run(&mut self) {
        while self.status & 0b0001_0000 == 0 {
            println!("running");
            let byte = self.mem_read(self.program_counter);
            let opcode = CPU_OPCODES
                .get(&byte)
                .expect(format!("opcode {:X} not found", byte).as_str());

            match &*opcode.name.to_lowercase() {
                "adc" => self.adc(opcode),
                "and" => self.and(opcode),
                "asl" => self.asl(opcode),
                "bcc" => self.bcc(opcode),
                "bcs" => self.bcs(opcode),
                "beq" => self.beq(opcode),
                "bit" => self.bit(opcode),
                "bmi" => self.bmi(opcode),
                "bne" => self.bne(opcode),
                "bpl" => self.bpl(opcode),
                "brk" => self.brk(opcode),
                "bvc" => self.bvc(opcode),
                "bvs" => self.bvs(opcode),
                "clc" => self.clc(opcode),
                "cld" => self.cld(opcode),
                "cli" => self.cli(opcode),
                "clv" => self.clv(opcode),
                "cmp" => self.cmp(opcode),
                "cpx" => self.cpx(opcode),
                "cpy" => self.cpy(opcode),
                "dec" => self.dec(opcode),
                "dex" => self.dex(opcode),
                "dey" => self.dey(opcode),
                "eor" => self.eor(opcode),
                "inc" => self.inc(opcode),
                "inx" => self.inx(opcode),
                "iny" => self.iny(opcode),
                "jmp" => self.jmp(opcode),
                "jsr" => self.jsr(opcode),
                "lda" => self.lda(opcode),
                "ldx" => self.ldx(opcode),
                "ldy" => self.ldy(opcode),
                "lsr" => self.lsr(opcode),
                "nop" => self.nop(opcode),
                "ora" => self.ora(opcode),
                "pha" => self.pha(opcode),
                "php" => self.php(opcode),
                "pla" => self.pla(opcode),
                "plp" => self.plp(opcode),
                "rol" => self.rol(opcode),
                "ror" => self.ror(opcode),
                "rti" => self.rti(opcode),
                "rts" => self.rts(opcode),
                "sbc" => self.sbc(opcode),
                "sec" => self.sec(opcode),
                "sed" => self.sed(opcode),
                "sei" => self.sei(opcode),
                "sta" => self.sta(opcode),
                "stx" => self.stx(opcode),
                "sty" => self.sty(opcode),
                "tax" => self.tax(opcode),
                "tay" => self.tay(opcode),
                "tsx" => self.tsx(opcode),
                "txa" => self.txa(opcode),
                "txs" => self.txs(opcode),
                "tya" => self.tya(opcode),
                other => panic!("unrecognized opcode {other}"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adc() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x02]);
        cpu.reset();
        cpu.register_a = 0xFF;
        cpu.run();

        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.status, 0b0001_0001);
    }

    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x29, 0x0F]);
        cpu.reset();
        cpu.register_a = 0x11;
        cpu.run();

        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.status, 0b0001_0000);
    }

    #[test]
    fn test_asl_register() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x0A]);
        cpu.reset();
        cpu.register_a = 0b1000_0001;
        cpu.run();

        assert_eq!(cpu.register_a, 0b0000_0010);
        assert_eq!(cpu.status, 0b0001_0001);
    }
    #[test]
    fn test_asl_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC0, 0b1000_0001);
        cpu.load(vec![0x06, 0xC0]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.mem_read(0xC0), 0b0000_0010);
        assert_eq!(cpu.status, 0b0001_0001);
    }

    #[test]
    fn test_bcc() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x90, 0x10]);
        cpu.reset();
        let orig_pc = cpu.program_counter;
        cpu.status = 0b1110_1110;
        cpu.run();

        // pc + offset + instr length + 1 for BRK
        assert_eq!(cpu.program_counter, orig_pc + 0x12 + 1);
    }
    #[test]
    fn test_bcc_negative() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x90, (-16 as i8 as u8)]);
        cpu.reset();
        let orig_pc = cpu.program_counter;
        cpu.status = 0b1110_1110;
        cpu.run();

        assert_eq!(cpu.program_counter, orig_pc - 14 + 1);
    }

    #[test]
    fn test_bcs() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xB0, 0x10]);
        cpu.reset();
        let orig_pc = cpu.program_counter;
        cpu.status = 0b0000_0001;
        cpu.run();

        assert_eq!(cpu.program_counter, orig_pc + 0x12 + 1);
    }

    #[test]
    fn test_beq() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xF0, 0x10]);
        cpu.reset();
        let orig_pc = cpu.program_counter;
        cpu.status = 0b0000_0010;
        cpu.run();

        assert_eq!(cpu.program_counter, orig_pc + 0x12 + 1);
    }

    #[test]
    fn test_bit() {
        return;
    }

    #[test]
    fn test_bmi() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x30, 0x10]);
        cpu.reset();
        let orig_pc = cpu.program_counter;
        cpu.status = 0b1000_0000;
        cpu.run();

        assert_eq!(cpu.program_counter, orig_pc + 0x12 + 1);
    }

    #[test]
    fn test_bne() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xD0, 0x10]);
        cpu.reset();
        let orig_pc = cpu.program_counter;
        cpu.status = 0b1110_1101;
        cpu.run();

        assert_eq!(cpu.program_counter, orig_pc + 0x12 + 1);
    }

    #[test]
    fn test_bpl() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x10, 0x10]);
        cpu.reset();
        let orig_pc = cpu.program_counter;
        cpu.status = 0b0110_1111;
        cpu.run();

        assert_eq!(cpu.program_counter, orig_pc + 0x12 + 1);
    }

    #[test]
    fn test_brk() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x00]);
        cpu.reset();
        let orig_pc = cpu.program_counter;
        cpu.status = 0b0000_0000;
        cpu.run();

        assert_eq!(cpu.program_counter, orig_pc + 1);
        assert_eq!(cpu.status, 0b0001_0000);
    }

    #[test]
    fn test_bvc() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x50, 0x10]);
        cpu.reset();
        let orig_pc = cpu.program_counter;
        cpu.status = 0b1010_1111;
        cpu.run();

        assert_eq!(cpu.program_counter, orig_pc + 0x12 + 1);
    }

    #[test]
    fn test_bvs() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x70, 0x10]);
        cpu.reset();
        let orig_pc = cpu.program_counter;
        cpu.status = 0b0100_0000;
        cpu.run();

        assert_eq!(cpu.program_counter, orig_pc + 0x12 + 1);
    }

    #[test]
    fn test_clc() {
        return;
    }

    #[test]
    fn test_cld() {
        return;
    }

    #[test]
    fn test_cli() {
        return;
    }

    #[test]
    fn test_clv() {
        return;
    }

    #[test]
    fn test_cmp() {
        return;
    }

    #[test]
    fn test_cpx() {
        return;
    }

    #[test]
    fn test_cpy() {
        return;
    }

    #[test]
    fn test_dec() {
        return;
    }

    #[test]
    fn test_dex() {
        return;
    }

    #[test]
    fn test_dey() {
        return;
    }

    #[test]
    fn test_eor() {
        return;
    }

    #[test]
    fn test_inc() {
        return;
    }

    // INX
    #[test]
    fn test_inx() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE8, 0x00]);
        cpu.reset();
        cpu.register_x = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_x, 0x1D);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_inx_set_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xF4;
        cpu.run();

        assert_eq!(cpu.register_x, 0xF5);
        assert_eq!(cpu.status, 0b1001_0000);
    }
    #[test]
    fn test_inx_set_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xFF;
        cpu.run();

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0001_0010);
    }
    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE8, 0xE8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xFF;
        cpu.run();

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_iny() {
        return;
    }

    #[test]
    fn test_jmp() {
        return;
    }

    #[test]
    fn test_jsr() {
        return;
    }

    // LDA (also tests all addressing modes)
    #[test]
    fn test_lda_sets_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA9, 0x00]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status, 0b0001_0010);
    }
    #[test]
    fn test_lda_sets_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA9, 0xF4]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0xF4);
        assert_eq!(cpu.status, 0b1001_0000);
    }
    #[test]
    fn test_lda_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA9, 0x1C]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_lda_zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC0, 0x1C);
        cpu.load(vec![0xA5, 0xC0]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_lda_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC5, 0x1C);
        cpu.load(vec![0xB5, 0xC0]);
        cpu.reset();
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_lda_absolute() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xC601, 0x1C);
        cpu.load(vec![0xAD, 0x01, 0xC6]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_lda_absolute_x() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xC606, 0x1C);
        cpu.load(vec![0xBD, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_lda_absolute_y() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xC606, 0x1C);
        cpu.load(vec![0xB9, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_y = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_lda_indirect_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC6, 0x1C);
        cpu.mem_write(0xBA, 0xC6);
        cpu.load(vec![0xA1, 0xB5]);
        cpu.reset();
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_lda_indirect_y() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC6, 0x1C);
        cpu.mem_write(0xBA, 0xC6);
        cpu.load(vec![0xB1, 0xB5]);
        cpu.reset();
        cpu.register_y = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }

    #[test]
    fn test_ldx() {
        return;
    }

    #[test]
    fn test_ldy() {
        return;
    }

    #[test]
    fn test_lsr() {
        return;
    }

    #[test]
    fn test_nop() {
        return;
    }

    #[test]
    fn test_ora() {
        return;
    }

    #[test]
    fn test_pha() {
        return;
    }

    #[test]
    fn test_php() {
        return;
    }

    #[test]
    fn test_pla() {
        return;
    }

    #[test]
    fn test_plp() {
        return;
    }

    #[test]
    fn test_rol() {
        return;
    }

    #[test]
    fn test_ror() {
        return;
    }

    #[test]
    fn test_rti() {
        return;
    }

    #[test]
    fn test_rts() {
        return;
    }

    #[test]
    fn test_sbc() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE9, 0x10]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_a, 0x0C);
        assert_eq!(cpu.status, 0b0001_0001);
    }

    #[test]
    fn test_sec() {
        return;
    }

    #[test]
    fn test_sed() {
        return;
    }

    #[test]
    fn test_sei() {
        return;
    }

    // STA
    #[test]
    fn test_sta_zero_page() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x85, 0xB5]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xB5), 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_sta_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x95, 0xB0]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xB1), 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_sta_absolute() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x8D, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xC601), 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_sta_absolute_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x9D, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xC602), 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_sta_absolute_y() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x99, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_y = 0x01;
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xC602), 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_sta_indirect_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x81, 0xBA]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_sta_indirect_y() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x91, 0xBA]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }

    #[test]
    fn test_stx() {
        return;
    }

    #[test]
    fn test_sty() {
        return;
    }

    // TAX
    #[test]
    fn test_tax() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xAA]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_x, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }
    #[test]
    fn test_tax_transfer_zero() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xAA]);
        cpu.reset();
        cpu.register_a = 0x00;
        cpu.run();

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0001_0010);
    }
    #[test]
    fn test_tax_transfer_negative() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xAA]);
        cpu.reset();
        cpu.register_a = 0xF4;
        cpu.run();

        assert_eq!(cpu.register_x, 0xF4);
        assert_eq!(cpu.status, 0b1001_0000);
    }

    #[test]
    fn test_tay() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA8]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_y, 0x1C);
        assert_eq!(cpu.status, 0b0001_0000);
    }

    #[test]
    fn test_tsx() {
        return;
    }

    #[test]
    fn test_txa() {
        return;
    }

    #[test]
    fn test_txs() {
        return;
    }

    #[test]
    fn test_tya() {
        return;
    }

    // integration, misc
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);
        // LDA 0xC0
        // TAX
        // INX
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_x, 0xC1)
    }
}

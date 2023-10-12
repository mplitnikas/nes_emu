use lazy_static::lazy_static;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
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
    // code: u8,
    name: &'static str,
    length: u16,
    cycles: usize,
    mode: AddressingMode,
}

impl OpCode {
    pub const fn new(
        // code: u8,
        name: &'static str,
        length: u16,
        cycles: usize,
        mode: AddressingMode,
    ) -> Self {
        OpCode {
            // code,
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
        (0x69, OpCode::new("ADC", 2, 2, AddressingMode::Immediate)),
        (0x65, OpCode::new("ADC", 2, 3, AddressingMode::ZeroPage)),
        (0x75, OpCode::new("ADC", 2, 4, AddressingMode::ZeroPage_X)),
        (0x6D, OpCode::new("ADC", 3, 4, AddressingMode::Absolute)),
        (0x7D, OpCode::new("ADC", 3, 4, AddressingMode::Absolute_X)), // +1
        (0x79, OpCode::new("ADC", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0x61, OpCode::new("ADC", 2, 6, AddressingMode::Indirect_X)),
        (0x71, OpCode::new("ADC", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0x29, OpCode::new("AND", 2, 2, AddressingMode::Immediate)),
        (0x25, OpCode::new("AND", 2, 3, AddressingMode::ZeroPage)),
        (0x35, OpCode::new("AND", 2, 4, AddressingMode::ZeroPage_X)),
        (0x2D, OpCode::new("AND", 3, 4, AddressingMode::Absolute)),
        (0x3D, OpCode::new("AND", 3, 4, AddressingMode::Absolute_X)), // +1
        (0x39, OpCode::new("AND", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0x21, OpCode::new("AND", 2, 6, AddressingMode::Indirect_X)),
        (0x31, OpCode::new("AND", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0x0A, OpCode::new("ASL", 1, 2, AddressingMode::NoneAddressing)),
        (0x06, OpCode::new("ASL", 2, 5, AddressingMode::ZeroPage)),
        (0x16, OpCode::new("ASL", 2, 6, AddressingMode::ZeroPage_X)),
        (0x0E, OpCode::new("ASL", 3, 6, AddressingMode::Absolute)),
        (0x1E, OpCode::new("ASL", 3, 7, AddressingMode::Absolute_X)), // +1

        (0x24, OpCode::new("BIT", 2, 3, AddressingMode::ZeroPage)),
        (0x2C, OpCode::new("BIT", 3, 4, AddressingMode::Absolute)),

        // A branch not taken requires two machine cycles.
        // Add one if the branch is taken and add one more if the branch crosses a page boundary.
        (0x10, OpCode::new("BPL", 2, 2, AddressingMode::Immediate)), // +1
        (0x30, OpCode::new("BMI", 2, 2, AddressingMode::ZeroPage)), // +1
        (0x50, OpCode::new("BVC", 2, 3, AddressingMode::ZeroPage_X)),
        (0x70, OpCode::new("BVS", 2, 2, AddressingMode::Absolute)), // +1
        (0x90, OpCode::new("BCC", 2, 2, AddressingMode::Absolute_X)), // +1
        (0xB0, OpCode::new("BCS", 2, 2, AddressingMode::Absolute_Y)), // +1
        (0xD0, OpCode::new("BNE", 2, 2, AddressingMode::Indirect_X)), // +1
        (0xF0, OpCode::new("BEQ", 2, 2, AddressingMode::Indirect_Y)), // +1

        (0x00, OpCode::new("BRK", 1, 7, AddressingMode::NoneAddressing)),

        (0xC9, OpCode::new("CMP", 2, 2, AddressingMode::Immediate)),
        (0xC5, OpCode::new("CMP", 2, 3, AddressingMode::ZeroPage)),
        (0xD5, OpCode::new("CMP", 2, 4, AddressingMode::ZeroPage_X)),
        (0xCD, OpCode::new("CMP", 3, 4, AddressingMode::Absolute)),
        (0xDD, OpCode::new("CMP", 3, 4, AddressingMode::Absolute_X)), // +1
        (0xD9, OpCode::new("CMP", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0xC1, OpCode::new("CMP", 2, 6, AddressingMode::Indirect_X)),
        (0xD1, OpCode::new("CMP", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0xE0, OpCode::new("CPX", 2, 2, AddressingMode::Immediate)),
        (0xE4, OpCode::new("CPX", 2, 3, AddressingMode::ZeroPage)),
        (0xEC, OpCode::new("CPX", 3, 4, AddressingMode::Absolute)),

        (0xC0, OpCode::new("CPY", 2, 2, AddressingMode::Immediate)),
        (0xC4, OpCode::new("CPY", 2, 3, AddressingMode::ZeroPage)),
        (0xCC, OpCode::new("CPY", 3, 4, AddressingMode::Absolute)),

        (0xC6, OpCode::new("DEC", 2, 5, AddressingMode::ZeroPage)),
        (0xD6, OpCode::new("DEC", 2, 6, AddressingMode::ZeroPage_X)),
        (0xCE, OpCode::new("DEC", 3, 6, AddressingMode::Absolute)),
        (0xDE, OpCode::new("DEC", 3, 7, AddressingMode::Absolute_X)),

        (0x49, OpCode::new("EOR", 2, 2, AddressingMode::Immediate)),
        (0x45, OpCode::new("EOR", 2, 3, AddressingMode::ZeroPage)),
        (0x55, OpCode::new("EOR", 2, 4, AddressingMode::ZeroPage_X)),
        (0x4D, OpCode::new("EOR", 3, 4, AddressingMode::Absolute)),
        (0x5D, OpCode::new("EOR", 3, 4, AddressingMode::Absolute_X)), // +1
        (0x59, OpCode::new("EOR", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0x41, OpCode::new("EOR", 2, 6, AddressingMode::Indirect_X)),
        (0x51, OpCode::new("EOR", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0x18, OpCode::new("CLC", 1, 2, AddressingMode::NoneAddressing)),
        (0x38, OpCode::new("SEC", 1, 2, AddressingMode::NoneAddressing)),
        (0x58, OpCode::new("CLI", 1, 2, AddressingMode::NoneAddressing)),
        (0x78, OpCode::new("SEI", 1, 2, AddressingMode::NoneAddressing)),
        (0xB8, OpCode::new("CLV", 1, 2, AddressingMode::NoneAddressing)),
        (0xD8, OpCode::new("CLD", 1, 2, AddressingMode::NoneAddressing)),
        (0xF8, OpCode::new("SED", 1, 2, AddressingMode::NoneAddressing)),

        (0xE6, OpCode::new("INC", 2, 5, AddressingMode::ZeroPage)),
        (0xF6, OpCode::new("INC", 2, 6, AddressingMode::ZeroPage_X)),
        (0xEE, OpCode::new("INC", 3, 6, AddressingMode::Absolute)),
        (0xFE, OpCode::new("INC", 3, 7, AddressingMode::Absolute_X)),

        (0x4C, OpCode::new("JMP", 3, 3, AddressingMode::Absolute)),
        (0x6C, OpCode::new("JMP", 3, 5, AddressingMode::Indirect)),

        (0x20, OpCode::new("JSR", 3, 6, AddressingMode::Absolute)),

        (0xA9, OpCode::new("LDA", 2, 2, AddressingMode::Immediate)),
        (0xA5, OpCode::new("LDA", 2, 3, AddressingMode::ZeroPage)),
        (0xB5, OpCode::new("LDA", 2, 4, AddressingMode::ZeroPage_X)),
        (0xAD, OpCode::new("LDA", 3, 4, AddressingMode::Absolute)),
        (0xBD, OpCode::new("LDA", 3, 4, AddressingMode::Absolute_X)), // +1 if page crossed
        (0xB9, OpCode::new("LDA", 3, 4, AddressingMode::Absolute_Y)), // +1 if page crossed
        (0xA1, OpCode::new("LDA", 2, 6, AddressingMode::Indirect_X)),
        (0xB1, OpCode::new("LDA", 2, 5, AddressingMode::Indirect_Y)), // +1 if page crossed

        (0xA2, OpCode::new("LDX", 2, 2, AddressingMode::Immediate)),
        (0xA6, OpCode::new("LDX", 2, 3, AddressingMode::ZeroPage)),
        (0xB6, OpCode::new("LDX", 2, 4, AddressingMode::ZeroPage_Y)),
        (0xAE, OpCode::new("LDX", 3, 4, AddressingMode::Absolute)),
        (0xBE, OpCode::new("LDX", 3, 4, AddressingMode::Absolute_Y)), // +1

        (0xA0, OpCode::new("LDY", 2, 2, AddressingMode::Immediate)),
        (0xA4, OpCode::new("LDY", 2, 3, AddressingMode::ZeroPage)),
        (0xB4, OpCode::new("LDY", 2, 4, AddressingMode::ZeroPage_X)),
        (0xAC, OpCode::new("LDY", 3, 4, AddressingMode::Absolute)),
        (0xBC, OpCode::new("LDY", 3, 4, AddressingMode::Absolute_X)), // +1

        (0x4A, OpCode::new("LSR", 1, 2, AddressingMode::NoneAddressing)),
        (0x46, OpCode::new("LSR", 2, 5, AddressingMode::ZeroPage)),
        (0x56, OpCode::new("LSR", 2, 6, AddressingMode::ZeroPage_X)),
        (0x4E, OpCode::new("LSR", 3, 6, AddressingMode::Absolute)),
        (0x5E, OpCode::new("LSR", 3, 7, AddressingMode::Absolute_X)),

        (0xEA, OpCode::new("NOP", 1, 2, AddressingMode::NoneAddressing)),

        (0x09, OpCode::new("ORA", 2, 2, AddressingMode::Immediate)),
        (0x05, OpCode::new("ORA", 2, 3, AddressingMode::ZeroPage)),
        (0x15, OpCode::new("ORA", 2, 4, AddressingMode::ZeroPage_X)),
        (0x0D, OpCode::new("ORA", 3, 4, AddressingMode::Absolute)),
        (0x1D, OpCode::new("ORA", 3, 4, AddressingMode::Absolute_X)), // +1
        (0x19, OpCode::new("ORA", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0x01, OpCode::new("ORA", 2, 6, AddressingMode::Indirect_X)),
        (0x11, OpCode::new("ORA", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0xAA, OpCode::new("TAX", 1, 2, AddressingMode::NoneAddressing)),
        (0x8A, OpCode::new("TXA", 1, 2, AddressingMode::NoneAddressing)),
        (0xCA, OpCode::new("DEX", 1, 2, AddressingMode::NoneAddressing)),
        (0xE8, OpCode::new("INX", 1, 2, AddressingMode::NoneAddressing)),
        (0xA8, OpCode::new("TAY", 1, 2, AddressingMode::NoneAddressing)),
        (0x98, OpCode::new("TYA", 1, 2, AddressingMode::NoneAddressing)),
        (0x88, OpCode::new("DEY", 1, 2, AddressingMode::NoneAddressing)),
        (0xC8, OpCode::new("INY", 1, 2, AddressingMode::NoneAddressing)),

        (0x2A, OpCode::new("ROL", 1, 2, AddressingMode::NoneAddressing)),
        (0x26, OpCode::new("ROL", 2, 5, AddressingMode::ZeroPage)),
        (0x36, OpCode::new("ROL", 2, 6, AddressingMode::ZeroPage_X)),
        (0x2E, OpCode::new("ROL", 3, 6, AddressingMode::Absolute)),
        (0x3E, OpCode::new("ROL", 3, 7, AddressingMode::Absolute_X)),

        (0x6A, OpCode::new("ROR", 1, 2, AddressingMode::NoneAddressing)),
        (0x66, OpCode::new("ROR", 2, 5, AddressingMode::ZeroPage)),
        (0x76, OpCode::new("ROR", 2, 6, AddressingMode::ZeroPage_X)),
        (0x6E, OpCode::new("ROR", 3, 6, AddressingMode::Absolute)),
        (0x7E, OpCode::new("ROR", 3, 7, AddressingMode::Absolute_X)),

        (0x40, OpCode::new("RTI", 1, 6, AddressingMode::NoneAddressing)),

        (0x60, OpCode::new("RTS", 1, 6, AddressingMode::NoneAddressing)),

        (0xE9, OpCode::new("SBC", 2, 2, AddressingMode::Immediate)),
        (0xE5, OpCode::new("SBC", 2, 3, AddressingMode::ZeroPage)),
        (0xF5, OpCode::new("SBC", 2, 4, AddressingMode::ZeroPage_X)),
        (0xED, OpCode::new("SBC", 3, 4, AddressingMode::Absolute)),
        (0xFD, OpCode::new("SBC", 3, 4, AddressingMode::Absolute_X)), // +1
        (0xF9, OpCode::new("SBC", 3, 4, AddressingMode::Absolute_Y)), // +1
        (0xE1, OpCode::new("SBC", 2, 6, AddressingMode::Indirect_X)),
        (0xF1, OpCode::new("SBC", 2, 5, AddressingMode::Indirect_Y)), // +1

        (0x85, OpCode::new("STA", 2, 3, AddressingMode::ZeroPage)),
        (0x95, OpCode::new("STA", 2, 4, AddressingMode::ZeroPage_X)),
        (0x8D, OpCode::new("STA", 3, 4, AddressingMode::Absolute)),
        (0x9D, OpCode::new("STA", 3, 5, AddressingMode::Absolute_X)),
        (0x99, OpCode::new("STA", 3, 5, AddressingMode::Absolute_Y)),
        (0x81, OpCode::new("STA", 2, 6, AddressingMode::Indirect_X)),
        (0x91, OpCode::new("STA", 2, 6, AddressingMode::Indirect_Y)),

        (0x9A, OpCode::new("TXS", 1, 2, AddressingMode::NoneAddressing)),
        (0xBA, OpCode::new("TSX", 1, 2, AddressingMode::NoneAddressing)),
        (0x48, OpCode::new("PHA", 1, 3, AddressingMode::NoneAddressing)),
        (0x68, OpCode::new("PLA", 1, 4, AddressingMode::NoneAddressing)),
        (0x08, OpCode::new("PHP", 1, 3, AddressingMode::NoneAddressing)),
        (0x28, OpCode::new("PLP", 1, 4, AddressingMode::NoneAddressing)),

        (0x86, OpCode::new("STX", 2, 3, AddressingMode::ZeroPage)),
        (0x96, OpCode::new("STX", 2, 4, AddressingMode::ZeroPage_Y)),
        (0x8E, OpCode::new("STX", 3, 4, AddressingMode::Absolute)),

        (0x84, OpCode::new("STY", 2, 3, AddressingMode::ZeroPage)),
        (0x94, OpCode::new("STY", 2, 4, AddressingMode::ZeroPage_X)),
        (0x8C, OpCode::new("STY", 3, 4, AddressingMode::Absolute)),
    ]);
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0, // NV_BDIZC
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    // returns (address, number of PC bytes consumed)
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
            AddressingMode::NoneAddressing => panic!("mode {:?} is not supported", mode),
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

    fn adc(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn and(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn asl(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn bcc(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn bcs(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn beq(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn bit(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn bmi(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn bne(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn bpl(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn brk(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn bvc(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn bvs(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn clc(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn cld(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn cli(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn clv(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn cmp(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn cpx(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn cpy(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn dec(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn dex(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn dey(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn eor(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn inc(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn inx(&mut self, opcode: &OpCode) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
        self.program_counter += opcode.length;
    }

    fn iny(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn jmp(&mut self, opcode: &OpCode) {
        todo!()
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
        todo!()
    }

    fn ldy(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn lsr(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn nop(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn ora(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn pha(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn php(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn pla(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn plp(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn rol(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn ror(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn rti(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn rts(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn sbc(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn sec(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn sed(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn sei(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn sta(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        self.mem_write(addr, self.register_a);
        self.program_counter += opcode.length;
    }

    fn stx(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn sty(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn tax(&mut self, opcode: &OpCode) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
        self.program_counter += opcode.length;
    }

    fn tay(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn tsx(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn txa(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn txs(&mut self, opcode: &OpCode) {
        todo!()
    }

    fn tya(&mut self, opcode: &OpCode) {
        todo!()
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

    pub fn run(&mut self) {
        loop {
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

    // LDA
    #[test]
    fn test_lda_sets_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA9, 0x00]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
    }
    #[test]
    fn test_lda_sets_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA9, 0xF4]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0xF4);
        assert_eq!(cpu.status, 0b1000_0000);
    }
    #[test]
    fn test_lda_immediate_0xa9() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA9, 0x1C]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_zero_page_0xa5() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC0, 0x1C);
        cpu.load(vec![0xA5, 0xC0]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_zero_page_x_0xb5() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC5, 0x1C);
        cpu.load(vec![0xB5, 0xC0]);
        cpu.reset();
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_absolute_0xad() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xC601, 0x1C);
        cpu.load(vec![0xAD, 0x01, 0xC6]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_absolute_x_0xbd() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xC606, 0x1C);
        cpu.load(vec![0xBD, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_absolute_y_0xb9() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xC606, 0x1C);
        cpu.load(vec![0xB9, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_y = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_indirect_x_0xa1() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC6, 0x1C);
        cpu.mem_write(0xBA, 0xC6);
        cpu.load(vec![0xA1, 0xB5]);
        cpu.reset();
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_indirect_y_0xb1() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC6, 0x1C);
        cpu.mem_write(0xBA, 0xC6);
        cpu.load(vec![0xB1, 0xB5]);
        cpu.reset();
        cpu.register_y = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }

    // STA
    #[test]
    fn test_sta_zero_page_0x85() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x85, 0xB5]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xB5), 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_sta_zero_page_x_0x95() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x95, 0xB0]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xB1), 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_sta_absolute_0x8d() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x8D, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xC601), 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_sta_absolute_x_0x9d() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x9D, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xC602), 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_sta_absolute_y_0x99() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x99, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_y = 0x01;
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xC602), 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
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
        assert_eq!(cpu.status, 0b0000_0000);
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
        assert_eq!(cpu.status, 0b0000_0000);
    }

    // TAX
    #[test]
    fn test_tax_0xaa() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xAA, 0x00]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_x, 0x1C);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_tax_transfer_zero() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xAA, 0x00]);
        cpu.reset();
        cpu.register_a = 0x00;
        cpu.run();

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
    }
    #[test]
    fn test_tax_transfer_negative() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xAA, 0x00]);
        cpu.reset();
        cpu.register_a = 0xF4;
        cpu.run();

        assert_eq!(cpu.register_x, 0xF4);
        assert_eq!(cpu.status, 0b1000_0000);
    }

    // INX
    #[test]
    fn test_inx_0xe8() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE8, 0x00]);
        cpu.reset();
        cpu.register_x = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_x, 0x1D);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_inx_set_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xF4;
        cpu.run();

        assert_eq!(cpu.register_x, 0xF5);
        assert_eq!(cpu.status, 0b1000_0000);
    }
    #[test]
    fn test_inx_set_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xFF;
        cpu.run();

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
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

    // misc
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

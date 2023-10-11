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
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
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
    fn get_operand_address(&self, mode: &AddressingMode) -> (u16, u16) {
        match mode {
            AddressingMode::Immediate => (self.program_counter, 1),
            AddressingMode::ZeroPage => (self.mem_read(self.program_counter) as u16, 1),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                (addr, 1)
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                (addr, 1)
            }
            AddressingMode::Absolute => (self.mem_read_u16(self.program_counter), 2),
            AddressingMode::Absolute_X => {
                let pos = self.mem_read_u16(self.program_counter);
                let addr = pos.wrapping_add(self.register_x as u16);
                (addr, 2)
            }
            AddressingMode::Absolute_Y => {
                let pos = self.mem_read_u16(self.program_counter);
                let addr = pos.wrapping_add(self.register_y as u16);
                (addr, 2)
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                ((hi as u16) << 8 | (lo as u16), 1)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);
                let ptr: u8 = base.wrapping_add(self.register_y);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                ((hi as u16) << 8 | (lo as u16), 1)
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
        // self.register_y = 0;
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

    fn lda(&mut self, mode: &AddressingMode) {
        let (addr, pc_increment) = self.get_operand_address(&mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
        self.program_counter += pc_increment;
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
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
            let opcode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opcode {
                // BRK
                0x00 => return,
                // LDA
                0xA9 => self.lda(&AddressingMode::Immediate),
                0xA5 => self.lda(&AddressingMode::ZeroPage),
                0xB5 => self.lda(&AddressingMode::ZeroPage_X),
                0xAD => self.lda(&AddressingMode::Absolute),
                0xBD => self.lda(&AddressingMode::Absolute_X),
                0xB9 => self.lda(&AddressingMode::Absolute_Y),
                0xA1 => self.lda(&AddressingMode::Indirect_X),
                0xB1 => self.lda(&AddressingMode::Indirect_Y),
                // TAX
                0xAA => self.tax(),
                // INX
                0xE8 => self.inx(),
                _ => todo!("opcode not implemented"),
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
        cpu.load(vec![0xA9, 0x05]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_zero_page_0xa5() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC0, 0x15);
        cpu.load(vec![0xA5, 0xC0]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x15);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_zero_page_x_0xb5() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC5, 0x15);
        cpu.load(vec![0xB5, 0xC0]);
        cpu.reset();
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x15);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_absolute_0xad() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xC601, 0x15);
        cpu.load(vec![0xAD, 0x01, 0xC6]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x15);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_absolute_x_0xbd() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xC606, 0x15);
        cpu.load(vec![0xBD, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x15);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_absolute_y_0xb9() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xC606, 0x15);
        cpu.load(vec![0xB9, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_y = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x15);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_indirect_x_0xa1() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC6, 0x15);
        cpu.mem_write(0xBA, 0xC6);
        cpu.load(vec![0xA1, 0xB5]);
        cpu.reset();
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x15);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_lda_indirect_y_0xb1() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC6, 0x15);
        cpu.mem_write(0xBA, 0xC6);
        cpu.load(vec![0xB1, 0xB5]);
        cpu.reset();
        cpu.register_y = 0x05;
        cpu.run();

        assert_eq!(cpu.register_a, 0x15);
        assert_eq!(cpu.status, 0b0000_0000);
    }

    // TAX
    #[test]
    fn test_tax_0xaa() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xAA, 0x00]);
        cpu.reset();
        cpu.register_a = 0x05;
        cpu.run();

        assert_eq!(cpu.register_x, 0x05);
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
        cpu.register_x = 0x04;
        cpu.run();

        assert_eq!(cpu.register_x, 0x05);
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
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_x, 0xC1)
    }
}

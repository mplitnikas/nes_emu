fn main() {
    println!("Hello, world!");
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    // pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            // register_y: 0,
            status: 0, // NV_BDIZC
            program_counter: 0,
            memory: [0; 0xFFFF],
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

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
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
        // PC is already updated by load()
        loop {
            let opcode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opcode {
                0x00 => return,
                0xA9 => {
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;
                    self.lda(param);
                }
                0xAA => self.tax(),
                0xE8 => self.inx(),
                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa9, 0x05, 0x00]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_0xa9_load_zero() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa9, 0x00, 0x00]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
    }
    #[test]
    fn test_0xa9_load_negative() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa9, 0xf4, 0x00]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0xf4);
        assert_eq!(cpu.status, 0b1000_0000);
    }

    #[test]
    fn test_0xaa_transfer_a_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xaa, 0x00]);
        cpu.reset();
        cpu.register_a = 0x05;
        cpu.run();

        assert_eq!(cpu.register_x, 0x05);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_0xaa_transfer_zero() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xaa, 0x00]);
        cpu.reset();
        cpu.register_a = 0x00;
        cpu.run();

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
    }
    #[test]
    fn test_0xaa_transfer_negative() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xaa, 0x00]);
        cpu.reset();
        cpu.register_a = 0xf4;
        cpu.run();

        assert_eq!(cpu.register_x, 0xf4);
        assert_eq!(cpu.status, 0b1000_0000);
    }

    #[test]
    fn test_0xe8_increment_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0x00]);
        cpu.reset();
        cpu.register_x = 0x04;
        cpu.run();

        assert_eq!(cpu.register_x, 0x05);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_0xe8_set_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xf4;
        cpu.run();

        assert_eq!(cpu.register_x, 0xf5);
        assert_eq!(cpu.status, 0b1000_0000);
    }
    #[test]
    fn test_0xe8_set_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xff;
        cpu.run();

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xff;
        cpu.run();

        assert_eq!(cpu.register_x, 1)
    }
}

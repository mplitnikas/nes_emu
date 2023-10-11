fn main() {
    println!("Hello, world!");
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    // pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0, // NV_BDIZC
            program_counter: 0,
        }
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

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                0x00 => return,
                0xA9 => {
                    let param = program[self.program_counter as usize];
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
        cpu.interpret(vec![0xa9, 0x05, 0x00]);

        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_0xa9_load_zero() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
    }
    #[test]
    fn test_0xa9_load_negative() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xf4, 0x00]);

        assert_eq!(cpu.register_a, 0xf4);
        assert_eq!(cpu.status, 0b1000_0000);
    }

    #[test]
    fn test_0xaa_transfer_a_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;

        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 0x05);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_0xaa_transfer_zero() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x00;

        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
    }
    #[test]
    fn test_0xaa_transfer_negative() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xf4;

        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 0xf4);
        assert_eq!(cpu.status, 0b1000_0000);
    }

    #[test]
    fn test_0xe8_increment_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x04;

        cpu.interpret(vec![0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0x05);
        assert_eq!(cpu.status, 0b0000_0000);
    }
    #[test]
    fn test_0xe8_set_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xf4;

        cpu.interpret(vec![0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xf5);
        assert_eq!(cpu.status, 0b1000_0000);
    }
    #[test]
    fn test_0xe8_set_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;

        cpu.interpret(vec![0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;

        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}

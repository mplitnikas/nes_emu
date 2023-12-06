use crate::bus::Bus;
use crate::opcodes::{AddressingMode, OpCode, CPU_OPCODES};

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    bus: Bus,
}

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);
    fn mem_read_u16(&self, addr: u16) -> u16;
    fn mem_write_u16(&mut self, addr: u16, data: u16);
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data);
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        self.bus.mem_read_u16(addr)
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        self.bus.mem_write_u16(addr, data)
    }
}

const STACK_ADDRESS: u16 = 0x0100;
const STACK_RESET: u8 = 0xFF;

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0b0010_0000, // NV1B_DIZC
            program_counter: 0,
            stack_pointer: STACK_RESET,
            bus: Bus::new(),
        }
    }

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
                let ptr = self.mem_read_u16(self.program_counter + 1);
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

    // pub fn mem_read(&self, addr: u16) -> u8 {
    //     self.memory[addr as usize]
    // }
    //
    // pub fn mem_write(&mut self, addr: u16, data: u8) {
    //     self.memory[addr as usize] = data;
    // }
    //
    // pub fn mem_read_u16(&self, pos: u16) -> u16 {
    //     let lo = self.mem_read(pos) as u16;
    //     let hi = self.mem_read(pos + 1) as u16;
    //     hi << 8 | lo
    // }
    //
    // pub fn mem_write_u16(&mut self, pos: u16, data: u16) {
    //     let hi = (data >> 8) as u8;
    //     let lo = (data & 0xFFFF) as u8;
    //     self.mem_write(pos, lo);
    //     self.mem_write(pos + 1, hi);
    // }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
        self.stack_pointer = 0xFF;
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for i in 0..(program.len() as u16) {
            self.mem_write(0x0600 + i, program[i as usize]);
        }
        self.mem_write_u16(0xFFFC, 0x0600);
    }

    pub fn add(&mut self, value: u8) -> u8 {
        let result = self
            .register_a
            .wrapping_add(value)
            .wrapping_add(self.status & 0b0000_0001);
        self.set_carry_flag(result < self.register_a);
        self.set_overflow_flag((self.register_a ^ result) & (value ^ result) & 0b1000_0000 != 0);
        result
    }

    pub fn push_to_stack(&mut self, value: u8) {
        self.stack_pointer -= 1;
        self.mem_write(STACK_ADDRESS + self.stack_pointer as u16, value);
    }

    pub fn pull_from_stack(&mut self) -> u8 {
        let value = self.mem_read(STACK_ADDRESS + self.stack_pointer as u16);
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

        let bitmask = value & 0b1100_0000;
        self.status &= 0b0011_1111;
        self.status |= bitmask;

        self.program_counter += opcode.length;
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
        self.set_interrupt_flag(true);
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
        let result = value.wrapping_sub(1);

        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
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
        self.program_counter = addr;
    }

    fn jsr(&mut self, opcode: &OpCode) {
        let jump_addr = self.get_operand_address(&opcode.mode);

        let next_instruction = self.program_counter + opcode.length - 1;
        let high_byte = (next_instruction >> 8) as u8;
        let low_byte = (next_instruction & 0xFF) as u8;
        self.push_to_stack(high_byte);
        self.push_to_stack(low_byte);

        self.program_counter = jump_addr;
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
        self.set_break_flag(true);
        self.program_counter += opcode.length;
    }

    fn pla(&mut self, opcode: &OpCode) {
        self.register_a = self.pull_from_stack();
        self.update_zero_and_negative_flags(self.register_a);
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

    fn rti(&mut self, _opcode: &OpCode) {
        self.status = self.pull_from_stack();
        let low_byte = self.pull_from_stack();
        let high_byte = self.pull_from_stack();

        self.program_counter = ((high_byte as u16) << 8) | (low_byte as u16);
    }

    fn rts(&mut self, _opcode: &OpCode) {
        let low_byte = self.pull_from_stack();
        let high_byte = self.pull_from_stack();
        let addr = ((high_byte as u16) << 8) | (low_byte as u16);

        self.program_counter = addr + 1;
    }

    fn sbc(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        let carry = self.status & 0b0000_0001;
        let subtrahend = !value + 1 - carry;
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
        self.register_x = self.mem_read(STACK_ADDRESS + self.stack_pointer as u16);
        self.program_counter += opcode.length;
    }

    fn txa(&mut self, opcode: &OpCode) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
        self.program_counter += opcode.length;
    }

    fn txs(&mut self, opcode: &OpCode) {
        self.mem_write(STACK_ADDRESS + self.stack_pointer as u16, self.register_x);
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
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        while self.status & 0b0001_0000 == 0 {
            callback(self);
            let byte = self.mem_read(self.program_counter);
            let opcode = CPU_OPCODES
                .get(&byte)
                .expect(format!("opcode {:X} not found", byte).as_str());
            // println!("===========");
            // println!("registers:");
            // println!("A: {:02X}", self.register_a);
            // println!("X: {:02X}", self.register_x);
            // println!("Y: {:02X}", self.register_y);
            // println!("last pressed key: {:02X}", self.mem_read(0xff));
            // let current_direction = self.mem_read(0x02);
            // match current_direction {
            //     0x01 => println!("current direction: up"),
            //     0x02 => println!("current direction: right"),
            //     0x04 => println!("current direction: down"),
            //     0x08 => println!("current direction: left"),
            //     _ => println!("current direction: unknown"),
            // }
            // println!("snake length: {}", self.mem_read(0x03));
            // println!(
            //     "snake head position: {:02X} {:02X}",
            //     self.mem_read(0x10),
            //     self.mem_read(0x11)
            // );
            // println!("status: {:08b}", self.status);
            // println!("opcode: {:02X}", byte);
            // println!("opcode: {:?}", opcode.name);
            // println!("PC: {:04X}", self.program_counter);
            // println!("stack contents:");
            // for byte in
            //     &self.memory[(STACK_ADDRESS + self.stack_pointer as u16) as usize..=0x01FF]
            // {
            //     println!("{:02X}", byte);
            // }

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
        let mut truth_table = vec![];
        truth_table.push((0x50, 0x10, 0x60, 0b0000_0000));
        truth_table.push((0x50, 0x50, 0xA0, 0b0100_0000));
        truth_table.push((0x50, 0x90, 0xE0, 0b0000_0000));
        truth_table.push((0x50, 0xD0, 0x20, 0b0000_0001));
        truth_table.push((0xD0, 0x10, 0xE0, 0b0000_0000));
        truth_table.push((0xD0, 0x50, 0x20, 0b0000_0001));
        truth_table.push((0xD0, 0x90, 0x60, 0b0100_0001));
        truth_table.push((0xD0, 0xD0, 0xA0, 0b0000_0001));

        for (a, b, expected, expected_status) in truth_table {
            cpu.load(vec![0x69, b]);
            cpu.reset();
            cpu.register_a = a;
            cpu.run();

            assert_eq!(
                cpu.register_a, expected,
                "a: {:X}, b: {:X}, register A: {:X}, expected: {:X}",
                a, b, cpu.register_a, expected
            );
            assert_eq!(
                cpu.status & expected_status,
                expected_status,
                "a: {:X}, b: {:X}, status: {:08b}, expected_status: {:08b}",
                a,
                b,
                cpu.status,
                expected_status
            );
        }
    }

    #[test]
    fn test_adc_with_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x69, 0x10]);
        cpu.reset();
        cpu.register_a = 0x50;
        cpu.status = 0b0000_0001;
        cpu.run();

        assert_eq!(cpu.register_a, 0x61);
    }

    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x29, 0x0F]);
        cpu.reset();
        cpu.register_a = 0x11;
        cpu.run();

        assert_eq!(cpu.register_a, 0x01);
        assert_eq!(cpu.status, 0b0001_0100);
    }

    #[test]
    fn test_asl_register() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x0A]);
        cpu.reset();
        cpu.register_a = 0b1000_0001;
        cpu.run();

        assert_eq!(cpu.register_a, 0b0000_0010);
        assert_eq!(cpu.status, 0b0001_0101);
    }
    #[test]
    fn test_asl_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC0, 0b1000_0001);
        cpu.load(vec![0x06, 0xC0]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.mem_read(0xC0), 0b0000_0010);
        assert_eq!(cpu.status, 0b0001_0101);
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
    fn test_beq_negative() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xF0, (-16 as i8 as u8)]);
        cpu.reset();
        let orig_pc = cpu.program_counter;
        cpu.status = 0b0000_0010;
        cpu.run();

        assert_eq!(cpu.program_counter, orig_pc - 14 + 1);
    }

    #[test]
    fn test_bit() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x24, 0xC0]);
        cpu.reset();
        cpu.mem_write(0xC0, 0b0100_0001);
        cpu.register_a = 0b0000_0001;
        cpu.status = 0b1000_0011;
        cpu.run();

        assert_eq!(cpu.status, 0b0101_0101);
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
        assert_eq!(cpu.status, 0b0001_0100);
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
        let mut cpu = CPU::new();
        cpu.load(vec![0x18]);
        cpu.reset();
        cpu.status = 0b0000_0001;
        cpu.run();

        assert_eq!(cpu.status, 0b0001_0100);
    }

    #[test]
    fn test_cld() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xD8]);
        cpu.reset();
        cpu.status = 0b0000_1000;
        cpu.run();

        assert_eq!(cpu.status, 0b0001_0100);
    }

    #[test]
    fn test_cli() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x58]);
        cpu.reset();
        cpu.status = 0b0000_0100;
        cpu.run();

        assert_eq!(cpu.status, 0b0001_0100);
    }

    #[test]
    fn test_clv() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xB8]);
        cpu.reset();
        cpu.status = 0b0100_0000;
        cpu.run();

        assert_eq!(cpu.status, 0b0001_0100);
    }

    #[test]
    fn test_cmp() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xC9, 0x01]);
        cpu.reset();
        cpu.register_a = 0x01;
        cpu.run();

        assert_eq!(cpu.status, 0b0001_0111);
    }

    #[test]
    fn test_cpx() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE0, 0x01]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.run();

        assert_eq!(cpu.status, 0b0001_0111);
    }

    #[test]
    fn test_cpy() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xC0, 0x01]);
        cpu.reset();
        cpu.register_y = 0x01;
        cpu.run();

        assert_eq!(cpu.status, 0b0001_0111);
    }

    #[test]
    fn test_dec() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xC6, 0xC0]);
        cpu.reset();
        cpu.mem_write(0xC0, 0x01);
        cpu.run();

        assert_eq!(cpu.mem_read(0xC0), 0x00);
        assert_eq!(cpu.status, 0b0001_0110);
    }

    #[test]
    fn test_dex() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xCA]);
        cpu.reset();
        cpu.register_x = 0x01;
        cpu.run();

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0001_0110);
    }

    #[test]
    fn test_dey() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x88]);
        cpu.reset();
        cpu.register_y = 0x01;
        cpu.run();

        assert_eq!(cpu.register_y, 0x00);
        assert_eq!(cpu.status, 0b0001_0110);
    }

    #[test]
    fn test_eor() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x49, 0x01]);
        cpu.reset();
        cpu.register_a = 0x01;
        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status, 0b0001_0110);
    }

    #[test]
    fn test_inc() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE6, 0xC0]);
        cpu.reset();
        cpu.mem_write(0xC0, 0x01);
        cpu.run();

        assert_eq!(cpu.mem_read(0xC0), 0x02);
        assert_eq!(cpu.status, 0b0001_0100);
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
        assert_eq!(cpu.status, 0b0001_0100);
    }
    #[test]
    fn test_inx_set_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xF4;
        cpu.run();

        assert_eq!(cpu.register_x, 0xF5);
        assert_eq!(cpu.status, 0b1001_0100);
    }
    #[test]
    fn test_inx_set_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xFF;
        cpu.run();

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0001_0110);
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
        let mut cpu = CPU::new();
        cpu.load(vec![0xC8, 0x00]);
        cpu.reset();
        cpu.register_y = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_y, 0x1D);
        assert_eq!(cpu.status, 0b0001_0100);
    }

    #[test]
    fn test_jmp_absolute() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x4C, 0x01, 0xC6]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.program_counter, 0xC601 + 1); // +1 for BRK
    }

    #[test]
    fn test_jmp_indirect() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x6C, 0xAA, 0x22]);
        cpu.reset();
        cpu.mem_write(0x22AA, 0xFC);
        cpu.mem_write(0x22AB, 0xBA);
        cpu.run();

        assert_eq!(cpu.program_counter, 0xBAFC + 1); // +1 for BRK
    }

    #[test]
    fn test_jsr() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x20, 0x01, 0xC6]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.program_counter, 0xC601 + 1); // +1 for BRK
        assert_eq!(cpu.mem_read(0x01FE), 0x06);
        assert_eq!(cpu.mem_read(0x01FD), 0x02);
        assert_eq!(cpu.stack_pointer, 0xFD);
    }

    // LDA (also tests all addressing modes)
    #[test]
    fn test_lda_sets_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA9, 0x00]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status, 0b0001_0110);
    }
    #[test]
    fn test_lda_sets_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA9, 0xF4]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0xF4);
        assert_eq!(cpu.status, 0b1001_0100);
    }
    #[test]
    fn test_lda_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA9, 0x1C]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0100);
    }
    #[test]
    fn test_lda_zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xC0, 0x1C);
        cpu.load(vec![0xA5, 0xC0]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0100);
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
        assert_eq!(cpu.status, 0b0001_0100);
    }
    #[test]
    fn test_lda_absolute() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xC601, 0x1C);
        cpu.load(vec![0xAD, 0x01, 0xC6]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.status, 0b0001_0100);
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
        assert_eq!(cpu.status, 0b0001_0100);
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
        assert_eq!(cpu.status, 0b0001_0100);
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
        assert_eq!(cpu.status, 0b0001_0100);
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
        assert_eq!(cpu.status, 0b0001_0100);
    }

    #[test]
    fn test_ldx() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA2, 0x1C]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_x, 0x1C);
    }

    #[test]
    fn test_ldy() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA0, 0x1C]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.register_y, 0x1C);
    }

    #[test]
    fn test_lsr() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x4A]);
        cpu.reset();
        cpu.register_a = 0b0000_0010;
        cpu.run();

        assert_eq!(cpu.register_a, 0b0000_0001);
        assert_eq!(cpu.status, 0b0001_0100);
    }

    #[test]
    fn test_nop() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xEA]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.status, 0b0001_0100);
    }

    #[test]
    fn test_ora() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x09, 0b1111_0000]);
        cpu.reset();
        cpu.register_a = 0x01;
        cpu.run();

        assert_eq!(cpu.register_a, 0b1111_0001);
        assert_eq!(cpu.status, 0b1001_0100);
    }

    #[test]
    fn test_pha() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x48]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0x01FE), 0x1C);
        assert_eq!(cpu.stack_pointer, 0xFE);
    }

    #[test]
    fn test_php() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x08]);
        cpu.reset();
        cpu.status = 0b1100_0001;
        cpu.run();

        assert_eq!(cpu.mem_read(0x01FE), 0b1100_0001);
        assert_eq!(cpu.stack_pointer, 0xFE);
    }

    #[test]
    fn test_pla() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x68]);
        cpu.reset();
        cpu.stack_pointer = 0xFC;
        cpu.mem_write(0x01FC, 0x1C);
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
        assert_eq!(cpu.stack_pointer, 0xFD);
    }

    #[test]
    fn test_plp() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x28]);
        cpu.reset();
        cpu.stack_pointer = 0xFC;
        cpu.mem_write(0x01FC, 0b0011_0001);
        cpu.run();

        assert_eq!(cpu.status, 0b0011_0001);
        assert_eq!(cpu.stack_pointer, 0xFD);
    }

    #[test]
    fn test_rol() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x2A]);
        cpu.reset();
        cpu.register_a = 0b0000_0010;
        cpu.status = 0b0000_0001;
        cpu.run();

        assert_eq!(cpu.register_a, 0b0000_0101);
        assert_eq!(cpu.status, 0b0001_0100);

        cpu.load(vec![0x2A]);
        cpu.reset();
        cpu.register_a = 0b1000_0010;
        cpu.status = 0b0000_0000;
        cpu.run();

        assert_eq!(cpu.register_a, 0b0000_0100);
        assert_eq!(cpu.status, 0b0001_0101);
    }

    #[test]
    fn test_ror() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x6A]);
        cpu.reset();
        cpu.register_a = 0b0000_0011;
        cpu.status = 0b0000_0000;
        cpu.run();

        assert_eq!(cpu.register_a, 0b0000_0001);
        assert_eq!(cpu.status, 0b0001_0101);

        cpu.load(vec![0x6A]);
        cpu.reset();
        cpu.register_a = 0b0000_0010;
        cpu.status = 0b0000_0001;
        cpu.run();

        assert_eq!(cpu.register_a, 0b1000_0001);
        assert_eq!(cpu.status, 0b1001_0100);
    }

    #[test]
    fn test_rti() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x40]);
        cpu.reset();
        cpu.stack_pointer = 0xFC;
        cpu.mem_write(0x01FC, 0b1110_1001);
        cpu.mem_write_u16(0x01FD, 0x01C6);
        cpu.run();

        assert_eq!(cpu.program_counter, 0x01C6 + 1); // +1 for BRK
        assert_eq!(cpu.status, 0b1111_1101); // BRK flag gets set too
    }

    #[test]
    fn test_rts() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x60]);
        cpu.reset();
        cpu.stack_pointer = 0xFC;
        cpu.mem_write_u16(0x01FC, 0x01C6);
        cpu.run();

        assert_eq!(cpu.program_counter, 0x01C7 + 1); // RTS adds 1, +1 for BRK
    }

    #[test]
    fn test_sbc() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE9, 0x10]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_a, 0x0C);
        assert_eq!(cpu.status, 0b0001_0101);
    }

    #[test]
    fn test_sec() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x38]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.status, 0b0001_0101);
    }

    #[test]
    fn test_sed() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xF8]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.status, 0b0001_1100);
    }

    #[test]
    fn test_sei() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x78]);
        cpu.reset();
        cpu.run();

        assert_eq!(cpu.status, 0b0001_0100);
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
        assert_eq!(cpu.status, 0b0001_0100);
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
        assert_eq!(cpu.status, 0b0001_0100);
    }
    #[test]
    fn test_sta_absolute() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x8D, 0x01, 0xC6]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xC601), 0x1C);
        assert_eq!(cpu.status, 0b0001_0100);
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
        assert_eq!(cpu.status, 0b0001_0100);
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
        assert_eq!(cpu.status, 0b0001_0100);
    }

    #[test]
    fn test_stx() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x86, 0xB5]);
        cpu.reset();
        cpu.register_x = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xB5), 0x1C);
    }

    #[test]
    fn test_sty() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x84, 0xB5]);
        cpu.reset();
        cpu.register_y = 0x1C;
        cpu.run();

        assert_eq!(cpu.mem_read(0xB5), 0x1C);
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
        assert_eq!(cpu.status, 0b0001_0100);
    }
    #[test]
    fn test_tax_transfer_zero() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xAA]);
        cpu.reset();
        cpu.register_a = 0x00;
        cpu.run();

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0001_0110);
    }
    #[test]
    fn test_tax_transfer_negative() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xAA]);
        cpu.reset();
        cpu.register_a = 0xF4;
        cpu.run();

        assert_eq!(cpu.register_x, 0xF4);
        assert_eq!(cpu.status, 0b1001_0100);
    }

    #[test]
    fn test_tay() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xA8]);
        cpu.reset();
        cpu.register_a = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_y, 0x1C);
    }

    #[test]
    fn test_tsx() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xBA]);
        cpu.reset();
        cpu.mem_write(STACK_ADDRESS + cpu.stack_pointer as u16, 0x1C);
        cpu.run();

        assert_eq!(cpu.register_x, 0x1C);
    }

    #[test]
    fn test_txa() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x8A]);
        cpu.reset();
        cpu.register_x = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
    }

    #[test]
    fn test_txs() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x9A]);
        cpu.reset();
        cpu.register_x = 0x1C;
        cpu.run();

        assert_eq!(cpu.stack_pointer, 0xFF);
        assert_eq!(cpu.mem_read(STACK_ADDRESS + cpu.stack_pointer as u16), 0x1C);
    }

    #[test]
    fn test_tya() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x98]);
        cpu.reset();
        cpu.register_y = 0x1C;
        cpu.run();

        assert_eq!(cpu.register_a, 0x1C);
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

    #[test]
    fn test_jsr_and_rts() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x20, 0x01, 0xC6, 0xAA]); // JSR 0xC601, TAX
        cpu.reset();
        cpu.mem_write(0xC601, 0xAA); // TAX - just filler
        cpu.mem_write(0xC602, 0x60); // RTS
        cpu.run();

        assert_eq!(cpu.program_counter, 0x605);
        assert_eq!(cpu.mem_read_u16(0x01FE), 0x06);
    }
}

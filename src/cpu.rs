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
    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
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
const STACK_RESET: u8 = 0xFD;
const RESET_VECTOR: u16 = 0xFFFC;

impl CPU {
    pub fn new(bus: Bus) -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0b0010_0100, // NV1B_DIZC
            program_counter: 0,
            stack_pointer: STACK_RESET,
            bus,
        }
    }

    pub fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
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
            AddressingMode::Relative => {
                // in this case the returned addr is the jump target
                // (not including the opcode length)
                let offset = self.mem_read(self.program_counter + 1);
                if offset & 0b1000_0000 != 0 {
                    self.program_counter
                        .wrapping_sub(((offset ^ 0b1111_1111) + 1) as u16)
                } else {
                    self.program_counter.wrapping_add(offset as u16)
                }
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
        self.status = 0b0010_0100;
        self.program_counter = self.mem_read_u16(RESET_VECTOR);
        // self.stack_pointer = STACK_RESET - 3;
        self.stack_pointer = STACK_RESET;
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

    // fn brk(&mut self, opcode: &OpCode) {
    //     self.program_counter += opcode.length;
    //     self.set_break_flag(true);
    //     self.set_interrupt_flag(true);
    // }

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
        self.push_to_stack(self.status | 0b0001_0000);
        self.program_counter += opcode.length;
    }

    fn pla(&mut self, opcode: &OpCode) {
        self.register_a = self.pull_from_stack();
        self.update_zero_and_negative_flags(self.register_a);
        self.program_counter += opcode.length;
    }

    fn plp(&mut self, opcode: &OpCode) {
        self.status = self.pull_from_stack() & 0b1110_1111 | 0b0010_0000;
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
        let subtrahend = (!value)
            .wrapping_add(1)
            .wrapping_sub(1 - carry)
            .wrapping_sub(carry);
        // subtract carry here since add() adds it
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
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flags(self.register_x);
        self.program_counter += opcode.length;
    }

    fn txa(&mut self, opcode: &OpCode) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
        self.program_counter += opcode.length;
    }

    fn txs(&mut self, opcode: &OpCode) {
        self.stack_pointer = self.register_x;
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
            self.program_counter = self.get_operand_address(&opcode.mode);
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
        loop {
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
                "brk" => return,
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

use crate::cpu::Mem;

pub struct Bus {
    cpu_vram: [u8; 2048],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cpu_vram: [0; 2048],
        }
    }
}

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
                todo!("ppu not implemented yet!")
            }
            _ => {
                println!("ignoring mem read from address {:04X}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize] = data
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
                todo!("ppu not implemented yet!")
            }
            _ => {
                println!("ignoring mem write to address {:04X}", addr);
            }
        }
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                let low = self.mem_read(mirror_down_addr);
                let high = self.mem_read(mirror_down_addr + 1);
                (high as u16) << 8 | low as u16
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
                todo!("ppu not implemented yet!")
            }
            _ => {
                println!("ignoring u16 mem read from address {:04X}", addr);
                0
            }
        }
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                let low = (data >> 8) as u8;
                let high = (data & 0xFFFF) as u8;
                self.mem_write(mirror_down_addr, low);
                self.mem_write(mirror_down_addr + 1, high);
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b0010_0000_0000_0111;
                todo!("ppu not implemented yet!")
            }
            _ => {
                println!("ignoring u16 mem write to address {:04X}", addr);
            }
        }
    }
}

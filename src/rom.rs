#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

const NES_TAG: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a];
const PRG_ROM_PAGE_SIZE: usize = 16384; // 16KB
const CHR_ROM_PAGE_SIZE: usize = 8192; // 8KB

impl Rom {
    pub fn new(raw: &Vec<u8>) -> Result<Self, String> {
        if &raw[0..4] != &NES_TAG {
            return Err("Invalid iNES file".to_string());
        }

        let mapper = (raw[7] & 0b1111_0000) | (raw[6] >> 4);
        if mapper != 0 {
            return Err(format!("Unsupported mapper: {}", mapper));
        }

        let ines_ver = (raw[7] >> 2) & 0b11;
        if ines_ver != 0 {
            return Err("NES v2 is not supported".to_string());
        }

        let four_screen = raw[6] & 0b1000 != 0;
        let vertical = raw[6] & 0b1 != 0;
        let screen_mirroring = match (four_screen, vertical) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let prg_rom_size = raw[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = raw[5] as usize * CHR_ROM_PAGE_SIZE;

        let skip_trainer = raw[6] & 0b100 != 0;

        let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        Ok(Rom {
            prg_rom: raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom: raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            mapper,
            screen_mirroring,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_load_rom() {
        let bytes = fs::read("roms/snake.nes").unwrap();

        let rom = Rom::new(&bytes).unwrap();
        assert_eq!(rom.mapper, 0);
    }
}

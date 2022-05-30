// 0x0000 - 0x3FFF : ROM Bank 0 | ROM0
// 0x4000 - 0x7FFF : ROM Bank 1 - Switchable | ROMX
// 0x8000 - 0x97FF : CHR RAM | VRAM
// 0x9800 - 0x9BFF : BG Map 1 | VRAM
// 0x9C00 - 0x9FFF : BG Map 2 | VRAM
// 0xA000 - 0xBFFF : Cartridge RAM | SRAM
// 0xC000 - 0xCFFF : RAM Bank 0 | WRAM0
// 0xD000 - 0xDFFF : RAM Bank 1-7 - switchable - Color only | WRAMX
// 0xE000 - 0xFDFF : Reserved - Echo RAM | ECHO
// 0xFE00 - 0xFE9F : Object Attribute Memory | OAM
// 0xFEA0 - 0xFEFF : Reserved - Unusable | USED
// 0xFF00 - 0xFF7F : I/O Registers | IO Registers
// 0xFF80 - 0xFFFE : Zero Page | HRAM
// 0xFFFF : IE Register
mod interrupts;

pub struct MMU {
    ram: [u8;65536], //0x0000 to 0xFFFF
}

impl MMU {
    pub fn new() -> MMU {
        let mut mem = MMU { ram: [0; 65536]};
        mem
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn from_rom_file(&mut self, rom_file: &[u8]) {
        let mut i: u16 = 0x0000;

        for &byte in rom_file.iter() {
            self.write_byte(i, byte);
            i += 1;
        }
    }

    pub fn mmu_section(&self, address: u16) -> &str {
        if address < 0x4000 {
            "ROM Bank 0 | ROM0"
        } else if address < 0x8000 {
            "ROM Bank 1 - Switchable | ROMX"
        } else if address < 0x9800 {
            "CHR RAM | VRAM"
        } else if address < 0x9C00 {
            "BG Map 1 | VRAM"
        } else if address < 0xA000 {
            "BG Map 2 | VRAM"
        } else if address < 0xC000 {
            "Cartridge RAM | SRAM"
        } else if address < 0xD000 {
            "RAM Bank 0 | WRAM0"
        } else if address < 0xE000 {
            "RAM Bank 1-7 - switchable - Color only | WRAMX"
        } else if address < 0xFE00 {
            "Reserved - Echo RAM | ECHO"
        } else if address < 0xFEA0 {
            "Object Attribute Memory | OAM"
        } else if address < 0xFF00 {
            "Reserved - Unusable | USED"
        } else if address < 0xFF80 {
            "I/O Registers | IO Registers"
        } else if address < 0xFFFF {
            "Zero Page | HRAM"
        } else {
            //address == 0xFFFF
            "IE Register"
        }
    }
}
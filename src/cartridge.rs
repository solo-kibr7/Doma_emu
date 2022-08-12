

pub struct Header {
    entry: [u8; 4],
    logo: [u8; 0x30],
    title: [u8; 16],
    new_lic_code: u16,
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    dest_code: u8,
    old_lic_code: u8,
    rom_version: u8,
    checksum: u8,
    global_checksum: u16,
}
impl Default for Header {
    fn default() -> Self {
        Self {
            entry: [0; 4],
            logo: [0; 0x30],
            title: [0; 16],
            new_lic_code: 0,
            sgb_flag: 0,
            cartridge_type: 0,
            rom_size: 0,
            ram_size: 0,
            dest_code: 0,
            old_lic_code: 0,
            rom_version: 0,
            checksum: 0,
            global_checksum: 0,
        }
    }
}

const NINTENDO_LOGO_DATA: &[u8; 48] = &[
    0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d,
    0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99,
    0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e,
];

#[derive(Default)]
pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    header: Header,
}
impl Cartridge {
    pub fn from_rom_file(&mut self, rom_file: &[u8]) {
        for &byte in rom_file.iter() {
            self.rom.push(byte);
        }
        let mut header_values: [u8; 0x50] = [0; 0x50];
        header_values.clone_from_slice(&self.rom[0x100..0x150]);
        self.load_header(&header_values);

        let title = self.get_title();
        let rom_type = self.get_type();
        let rom_size = self.rom_size();

        let mut x: u8 = 0;
        for &value in self.rom[0x0134..=0x014C].iter() {
            x = x.wrapping_sub(value).wrapping_sub(1);
        }

        let checksum_state = self.header.checksum == x as u8;

        println!("title:{} \ntype:{} \nrom_size:{}kb, ram_size:{} \nlic:{} \nversion:{}, checksum:{}", 
            title, rom_type, rom_size, self.header.ram_size, self.lic_name(), self.header.rom_version, checksum_state);
    }

    pub fn from_boot_file(&mut self, boot_file: &[u8]) {
        for (i, &byte) in boot_file.iter().enumerate() {
            if i == 0x100 {
                println!("bad");
                break;
            }
            self.rom[i] = byte;
        }
    }

    pub fn read_cart(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn load_header(&mut self, data: &[u8]) {
        for (i, value) in data.iter().enumerate() {
            if i < 4 {
                self.header.entry[i] = *value;
            } else if i < 0x34 {
                self.header.logo[i - 4] = *value;
            } else if i < 0x44 {
                self.header.title[i - 0x34] = *value;
            } else if i < 0x46 {
                if i == 0x44 {
                    self.header.new_lic_code = (*value as u16) << 8;
                } else {
                    self.header.new_lic_code += *value as u16;
                }
            } else if i < 0x47 {
                self.header.sgb_flag = *value;
            } else if i < 0x48 {
                self.header.cartridge_type = *value;
            } else if i < 0x49 {
                self.header.rom_size = *value;
            } else if i < 0x4A {
                self.header.ram_size = *value;
            } else if i < 0x4B {
                self.header.dest_code = *value;
            } else if i < 0x4C {
                self.header.old_lic_code = *value;
            } else if i < 0x4D {
                self.header.rom_version = *value;
            } else if i < 0x4E {
                self.header.checksum = *value;
            } else if i < 0x50 {
                if i == 0x4E {
                    self.header.global_checksum = (*value as u16) << 8;
                } else {
                    self.header.global_checksum += *value as u16;
                }
            }
        }
    }

    pub fn get_title(&self) -> String {
        let mut word = String::new();
        for num in self.header.title {
            let mut letter = '@';
            match char::from_u32(num as u32) {
                Some(x) => letter = x,
                None => println!("oops"),
            };
            
            word.push(letter);
        }
        word
    }

    pub fn lic_name(&self) -> &str {
        if self.header.old_lic_code == 0x33 {
            let num1 = self.header.new_lic_code >> 8;
            let num2 = self.header.new_lic_code as u8;

            let letter1;
            let letter2;
            match char::from_u32(num1 as u32) {
                Some(x) => letter1 = x,
                None => return "UNKNOWN",
            };
            match char::from_u32(num2 as u32) {
                Some(x) => letter2 = x,
                None => return "UNKNOWN",
            };

            if !letter1.to_digit(16).is_some() || !letter2.to_digit(16).is_some(){
                return "UNKNOWN";
            }

            let code1 = letter1.to_digit(16).unwrap();
            let code2 = letter2.to_digit(16).unwrap();

            self.get_lic((code1 << 4) as u8 + code2 as u8)

        } else {
            self.get_lic(self.header.old_lic_code)
        }
        
    }

    pub fn rom_size(&self) -> usize {
        32 * (1 << self.header.rom_size)
    }

    pub fn get_lic(&self, lic_code: u8) -> &str {
        match lic_code {
            0x00 => "None",
            0x01 => "Nintendo R&D1",
            0x08 => "Capcom",
            0x13 => "Electronic Arts",
            0x18 => "Hudson Soft",
            0x19 => "b-ai",
            0x20 => "kss",
            0x22 => "pow",
            0x24 => "PCM Complete",
            0x25 => "san-x",
            0x28 => "Kemco Japan",
            0x29 => "seta",
            0x30 => "Viacom",
            0x31 => "Nintendo",
            0x32 => "Bandai",
            0x33 => "Ocean/Acclaim",
            0x34 => "Konami",
            0x35 => "Hector",
            0x37 => "Taito",
            0x38 => "Hudson",
            0x39 => "Banpresto",
            0x41 => "Ubi Soft",
            0x42 => "Atlus",
            0x44 => "Malibu",
            0x46 => "angel",
            0x47 => "Bullet-Proof",
            0x49 => "irem",
            0x50 => "Absolute",
            0x51 => "Acclaim",
            0x52 => "Activision",
            0x53 => "American sammy",
            0x54 => "Konami",
            0x55 => "Hi tech entertainment",
            0x56 => "LJN",
            0x57 => "Matchbox",
            0x58 => "Mattel",
            0x59 => "Milton Bradley",
            0x60 => "Titus",
            0x61 => "Virgin",
            0x64 => "LucasArts",
            0x67 => "Ocean",
            0x69 => "Electronic Arts",
            0x70 => "Infogrames",
            0x71 => "Interplay",
            0x72 => "Broderbund",
            0x73 => "sculptured",
            0x75 => "sci",
            0x78 => "THQ",
            0x79 => "Accolade",
            0x80 => "misawa",
            0x83 => "lozc",
            0x86 => "Tokuma Shoten Intermedia",
            0x87 => "Tsukuda Original",
            0x91 => "Chunsoft",
            0x92 => "Video system",
            0x93 => "Ocean/Acclaim",
            0x95 => "Varie",
            0x96 => "Yonezawa/s’pal",
            0x97 => "Kaneko",
            0x99 => "Pack in soft",
            0xA4 => "Konami (Yu-Gi-Oh!)",
            _    => "UNKNOWN",
        }
    }

    pub fn get_type(&self) -> &str {
        match self.header.cartridge_type {
            0x00 => "ROM ONLY",
            0x01 => "MBC1",
            0x02 => "MBC1+RAM",
            0x03 => "MBC1+RAM+BATTERY",
            0x05 => "MBC2",
            0x06 => "MBC2+BATTERY",
            0x08 => "ROM+RAM",
            0x09 => "ROM+RAM+BATTERY",
            0x0B => "MMM01",
            0x0C => "MMM01+RAM",
            0x0D => "MMM01+RAM+BATTERY",
            0x0F => "MBC3+TIMER+BATTERY",
            0x10 => "MBC3+TIMER+RAM+BATTERY",
            0x11 => "MBC3",
            0x12 => "MBC3+RAM",
            0x13 => "MBC3+RAM+BATTERY",
            0x19 => "MBC5",
            0x1A => "MBC5+RAM",
            0x1B => "MBC5+RAM+BATTERY",
            0x1C => "MBC5+RUMBLE",
            0x1D => "MBC5+RUMBLE+RAM",
            0x1E => "MBC5+RUMBLE+RAM+BATTERY",
            0x20 => "MBC6",
            0x22 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
            0xFC => "POCKET CAMERA",
            0xFD => "BANDAI TAMA5",
            0xFE => "HuC3",
            0xFF => "HuC1+RAM+BATTERY",
            _    => "UNKNOWN",
        }
    }
}
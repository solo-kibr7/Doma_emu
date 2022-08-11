

pub struct Header {
    entry: [u8; 4],
    logo: [u8; 0x30],
    tile: [u8; 16],
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
            tile: [0; 16],
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
    }

    pub fn read_cart(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn load_header(&mut self, data: &[u8]) {
        for (i, value) in data.iter().enumerate() {
            if i < 4 {
                self.header.entry[i] = *value;
            } else if i < 0x34 {
                self.header.logo[i] = *value;
            } else if i < 0x44 {
                self.header.tile[i] = *value;
            } else if i < 0x46 {
                if i == 0x44 {
                    self.header.new_lic_code = (*value << 4) as u16;
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
                    self.header.global_checksum = (*value << 4) as u16;
                } else {
                    self.header.global_checksum += *value as u16;
                }
            }
        }
    }

    pub fn lic_name(&self, lic_code: u8) -> &str {
        if lic_code <= 0xA4 {
            self.get_lic(lic_code)
        } else {
            "UNKNOWN"
        }
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
}
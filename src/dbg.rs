pub use crate::mmu::MMU;
use ascii::AsciiCast;

#[derive(Debug)]
pub struct DBG {
    msg: [u8; 1024],
    counter: usize,
}

impl Default for DBG {
    fn default() -> Self {
        Self {
            msg: [0; 1024],
            counter: 0,
        }
    }
}

impl DBG {
    pub fn dbg_update(&mut self, mmu: &mut MMU) {
        //println!("{:?}", mmu.read_byte(0xFF01));
        if mmu.read_byte(0xFF02) == 0x81 {
            let c: u8 = mmu.read_byte(0xFF01);

            println!("DBG: {:#X} and {:?}", c, c.to_ascii());
    
            self.msg[self.counter] = c;
            self.counter += 1;
    
            mmu.write_byte(0xFF02, 0);
        }
    }
    pub fn dbg_print(&self) {

        if self.msg[0] != 0 {
            //println!("DBG: {:?}\n", self.msg);
        }
    }
}
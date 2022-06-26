pub use crate::mmu::MMU;
use ascii::AsciiCast;

#[derive(Debug)]
pub struct DBG {
    msg: [u8; 1024],
    counter: usize,
    word: String,
}

impl Default for DBG {
    fn default() -> Self {
        Self {
            msg: [0; 1024],
            counter: 0,
            word: String::new(),
        }
    }
}

impl DBG {
    pub fn dbg_update(&mut self, mmu: &mut MMU) {
        //println!("{:?}", mmu.read_byte(0xFF01));
        if mmu.read_byte(0xFF02) == 0x81 {
            let c: u8 = mmu.read_byte(0xFF01);

            let a = c.to_ascii();

            /*let mut letter = match a {
                Ok(x) => x,
                Err(_) => '*'.to_ascii(),
            };*/

            let mut letter = '@';
            match char::from_u32(c as u32) {
                Some(x) => letter = x,
                None => println!("oops"),
            };

            //let mut word = String::new();
            
            
            if letter.eq(&'\n') {
                if self.word.is_empty() {
                    println!("empty");
                } else {
                    println!("{}", self.word);
                }
                self.word = String::from("DBG: ");
            } else {
                self.word.push(letter);
                //println!("{}", self.word);
            }

            /* let letter = match a {
                Ok(x) => println!("DBG: {:?}", x),
                Err(_) => println!("DBG: None"),
            }; */

            //println!("DBG: {:#X} and {:?}", c, c.to_ascii());
    
            //self.msg[self.counter] = c;
            //self.counter += 1;
    
            mmu.write_byte(0xFF02, 0);
        }
    }
    pub fn dbg_print(&self) {

        if self.msg[0] != 0 {
            //println!("DBG: {:?}\n", self.msg);
        }
    }
}
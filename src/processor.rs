use crate::prelude::*;

use std::fs;
use std::io::Error;
use rand::Rng;

#[cfg(test)]
mod test{
use crate::prelude::*;

#[test]
    fn display_tests(){

    let display1 = vec![0;64*32];
    let display2 = vec![[vec![0;64]];32];

        
    let value1 = 0x3c & (0x80 >> 3) != 0;
    let value2 = 0x3c & (0x80 >> 2);

    dbg!(value1);
    dbg!(value2);


    }



}// mod test



pub struct Processor{

pub vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
pub vram_changed: bool,
pub ram: [u8;CHIP8_RAM],
stack: [usize;16],
pc: usize,
sp: usize,
i: usize,
pub v: [u8;16],
pub keypad: [bool;16],
pub keypad_waiting: bool,
pub keypad_register: usize,
pub delay_timer : u8
}

enum ProgramCounter {
    Next,
    Skip,
    Jump(usize),
}


impl ProgramCounter {
    fn skip_if(condition: bool) -> ProgramCounter {
        if condition {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }
}


impl Processor{

pub fn new() -> Self{

let mut ram = [0u8;CHIP8_RAM];

for (i,v) in FONT_SET.iter().enumerate(){
ram[i] = *v
}

Self{
    ram,
    stack: [0;16],
    v: [0;16],
    i:0,
    pc:0x200,
    sp:0,
    vram: [[0;CHIP8_WIDTH];CHIP8_HEIGHT],
    vram_changed: false,
    keypad: [false;16],
    keypad_waiting: false,
    keypad_register: 0,
    delay_timer: 0,
}


}

pub fn get_file(&mut self,path: &Path) -> Result<Vec<u8>,Error>{

let file = fs::read(path)?;

for(i,val) in file.iter().enumerate(){

self.ram[self.pc + i] = *val;


}

Ok(file)
}

pub fn fetch_op(&mut self) -> u16{

(self.ram[self.pc] as u16) << 8 | (self.ram[self.pc + 1] as u16)

//self.pc = self.pc.wrapping_add(2);

//op
}



pub fn run_opcode(&mut self,opcode: u16){
      let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

let pc_change = match nibbles { 
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xkk(x, kk),
            (0x07, _, _, _) => self.op_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8x06(x),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8x0e(x),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_annn(nnn),
            (0x0b, _, _, _) => self.op_bnnn(nnn),
            (0x0c, _, _, _) => self.op_cxkk(x, kk),
            (0x0d, _, _, _) => self.op_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            //(0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(x),
            _ => ProgramCounter::Next,
        };

     match pc_change {
            ProgramCounter::Next => self.pc = self.pc.wrapping_add(2),
            ProgramCounter::Skip => self.pc += 2 * 2,
            ProgramCounter::Jump(addr) => self.pc = addr,
        }



}



fn op_00e0(&mut self) -> ProgramCounter {
        for y in self.vram.iter_mut(){
            for x in y.iter_mut(){
                *x = 0;
            }
        }    
        self.vram_changed = true;
        ProgramCounter::Next
}

fn op_1nnn(&mut self, nnn: usize) -> ProgramCounter {
        ProgramCounter::Jump(nnn)
    }



fn op_dxyn(&mut self, x: usize, y: usize, n: usize) -> ProgramCounter {
              self.v[0x0f] = 0;
        for byte in 0..n {
            let pixel = self.ram[self.i+byte];
            let y = (self.v[y] as usize + byte) % CHIP8_HEIGHT;
            for bit in 0..8 {
                let x = (self.v[x] as usize + bit) % CHIP8_WIDTH;
                           // pixel >> (7 - bit) & 1;
                let color =  pixel & (0x80 >> bit);
                                    //0x80 = 10000000
                // what happens above is that im just verifying 1 bit per bit in a byte so as
                // example   101010 & 
                //         10000000  <-- it happens 8 times verifying bit per bit with the bit
                //                   mask(0x80)

                self.v[0x0f] |= color & self.vram[y][x];
                self.vram[y][x] ^= color;
            //    if color != 0{
               //     if self.vram[y][x] == 1{
                //        self.v[0x0f] = 1;
                 //   }
                  //  self.vram[y][x] ^= 1;
               // }
            }
        }
        self.vram_changed = true;
        ProgramCounter::Next     


    }

fn op_annn(&mut self, nnn: usize) -> ProgramCounter {
        self.i = nnn;
        ProgramCounter::Next
    }


 fn op_7xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        let vx = self.v[x] as u16;
        let val = kk as u16;
        let result = vx + val;
        self.v[x] = result as u8;
        ProgramCounter::Next
    }


fn op_6xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        self.v[x] = kk;
        ProgramCounter::Next
    }

// Returning from a subroutine is done with 00EE, and it does this by removing (“popping”) the last address from the stack and setting the PC to it.
fn op_00ee(&mut self) -> ProgramCounter{
    self.sp -= 1;
    ProgramCounter::Jump(self.stack[self.sp])
}


fn op_2nnn(&mut self,nnn: usize) -> ProgramCounter {
    self.stack[self.sp] = self.pc + 2;
    self.sp +=1;
    ProgramCounter::Jump(nnn)
}


fn op_3xkk(&self,x: usize,kk:u8) ->ProgramCounter{

ProgramCounter::skip_if(self.v[x] == kk)

}

fn op_4xkk(&self,x: usize, kk: u8) -> ProgramCounter{
ProgramCounter::skip_if(self.v[x]!= kk)
}


fn op_5xy0(&self, x: usize, y:usize) -> ProgramCounter{
    ProgramCounter::skip_if(self.v[x] == self.v[y])
}


fn op_8xy0(&mut self,x:usize,y:usize) ->ProgramCounter{
    self.v[x] = self.v[y];
    ProgramCounter::Next
}


fn op_8xy1(&mut self,x:usize,y:usize) -> ProgramCounter{
    self.v[x] |= self.v[y];
    ProgramCounter::Next
}

fn op_8xy2(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] &= self.v[y];
        ProgramCounter::Next
    }



fn op_8xy3(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] ^= self.v[y];
        ProgramCounter::Next
    }



fn op_8xy4(&mut self,x:usize,y:usize) ->ProgramCounter{
    let result = (self.v[x] as u16) + (self.v[y] as u16);
    self.v[x] = result as u8;
    self.v[0x0f] = if result > 0xFF {1} else {0};
    ProgramCounter::Next
}


 fn op_8xy5(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[0x0f] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        ProgramCounter::Next
    }



fn op_8x06(&mut self, x: usize) -> ProgramCounter {
        self.v[0x0f] = self.v[x] & 1;
        self.v[x] >>= 1;
        ProgramCounter::Next
    }




fn op_8xy7(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[0x0f] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        ProgramCounter::Next
    }



 fn op_8x0e(&mut self, x: usize) -> ProgramCounter {
        self.v[0x0f] = (self.v[x] & 0b10000000) >> 7;
        self.v[x] <<= 1;
        ProgramCounter::Next
    }


fn op_9xy0(&mut self, x: usize, y: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] != self.v[y])
    }



fn op_bnnn(&mut self, nnn: usize) -> ProgramCounter {
        ProgramCounter::Jump((self.v[0] as usize) + nnn)
    }


fn op_cxkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        let mut rng = rand::thread_rng();
        self.v[x] = rng.gen::<u8>() & kk;
        ProgramCounter::Next
    }

  fn op_ex9e(&mut self, x: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.keypad[self.v[x] as usize])
    }


fn op_exa1(&mut self, x: usize) -> ProgramCounter {
        ProgramCounter::skip_if(!self.keypad[self.v[x] as usize])
    }

fn op_fx0a(&mut self, x: usize) -> ProgramCounter {
        self.keypad_waiting = true;
        self.keypad_register = x;
        ProgramCounter::Next
    }


fn op_fx1e(&mut self, x: usize) -> ProgramCounter {
        self.i += self.v[x] as usize;
        self.v[0x0f] = if self.i > 0x0F00 { 1 } else { 0 };
        ProgramCounter::Next
    }


 fn op_fx07(&mut self, x: usize) -> ProgramCounter {
        self.v[x] = self.delay_timer;
        ProgramCounter::Next
    }

fn op_fx15(&mut self, x: usize) -> ProgramCounter {
        self.delay_timer = self.v[x];
        ProgramCounter::Next
    }
fn op_fx29(&mut self, x: usize) -> ProgramCounter {
        self.i = (self.v[x] as usize) * 5;
        ProgramCounter::Next
    }


fn op_fx33(&mut self, x: usize) -> ProgramCounter {
        self.ram[self.i] = self.v[x] / 100;
        self.ram[self.i + 1] = (self.v[x] % 100) / 10;
        self.ram[self.i + 2] = self.v[x] % 10;
        ProgramCounter::Next
    }




fn op_fx55(&mut self, x: usize) -> ProgramCounter {
        for i in 0..x + 1 {
            self.ram[self.i + i] = self.v[i];
        }
        ProgramCounter::Next
    }



fn op_fx65(&mut self, x: usize) -> ProgramCounter {
        for i in 0..x + 1 {
            self.v[i] = self.ram[self.i + i];
        }
        ProgramCounter::Next
    }

}// impl Processor

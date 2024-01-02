




use ggez::{
    event,graphics,
    input::keyboard::{KeyCode,KeyInput},
    Context,GameResult,
};

use crate::prelude::*;


pub struct Display{

pub ch8: Processor

}

impl Display{

pub fn new() ->Self{
    Self{
        ch8: Processor::new()
    }
}

pub fn draw(&mut self,canvas: &mut graphics::Canvas,pixel:[[u8;CHIP8_WIDTH];CHIP8_HEIGHT]){

        
        for(y,row) in pixel.iter().enumerate(){
            for(x,&col) in row.iter().enumerate(){
                let x = (x as u32) * 20;
                let y = (y as u32) * 20;
                let rec = graphics::Rect::new_i32(x as i32,y as i32, 20 as i32, 20 as i32);

                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                    .dest_rect(rec)
                    .color(color(col)),
                    )

            }
        }

}


}// impl Display

fn color(value: u8) -> ggez::graphics::Color {
    if value == 0 {
        ggez::graphics::Color::new(0.0,0.0,0.0,1.0)
    } else {
        ggez::graphics::Color::new(1.0,1.0,1.0,1.0)
    }
}



impl event::EventHandler<ggez::GameError> for Display{

fn update(&mut self,ctx: &mut Context) -> GameResult{

while ctx.time.check_update_time(444 as u32){

if self.ch8.keypad_waiting{
    for i in 0..self.ch8.keypad.len(){
        if self.ch8.keypad[i] {
            self.ch8.keypad_waiting = false;
            self.ch8.v[self.ch8.keypad_register] = i as u8;
            break;
        }
    }
}else{
if self.ch8.delay_timer > 0{
    self.ch8.delay_timer -=1
}

let opcode = self.ch8.fetch_op();
self.ch8.run_opcode(opcode);
}
}



Ok(())
}

fn draw(&mut self, ctx:&mut Context) -> GameResult{
    let mut canvas = graphics::Canvas::from_frame(ctx,graphics::Color::from([0.0,1.0,0.0,1.0]));

 if self.ch8.vram_changed{   
    self.draw(&mut canvas,self.ch8.vram);
 }
    canvas.finish(ctx)?;
    ggez::timer::yield_now();

    Ok(())
}

fn key_down_event(&mut self,ctx:&mut Context,input:KeyInput,_repeated:bool) -> GameResult{
match input.keycode{
Some(KeyCode::Numpad1) => self.ch8.keypad[0x1] = true,
Some(KeyCode::Numpad2) => self.ch8.keypad[0x2] = true,
Some(KeyCode::Numpad3) => self.ch8.keypad[0x3] = true,
Some(KeyCode::Numpad4) => self.ch8.keypad[0xc] = true,
Some(KeyCode::Q) => self.ch8.keypad[0x4] = true,
Some(KeyCode::W) => self.ch8.keypad[0x5] = true,
Some(KeyCode::E) => self.ch8.keypad[0x6] = true,
Some(KeyCode::R) => self.ch8.keypad[0xd] = true,
Some(KeyCode::A) => self.ch8.keypad[0x7] = true,
Some(KeyCode::S) =>  self.ch8.keypad[0x8] = true,
Some(KeyCode::D) =>  self.ch8.keypad[0x9] = true,
Some(KeyCode::F) =>  self.ch8.keypad[0xe] = true,
Some(KeyCode::Z) => self.ch8.keypad[0xa] = true,
Some(KeyCode::X) => self.ch8.keypad[0x0] = true,
Some(KeyCode::C) => self.ch8.keypad[0xb] = true,
Some(KeyCode::V) => self.ch8.keypad[0xf]= true,
Some(KeyCode::Escape) => ctx.request_quit(),
_ => (),
};


Ok(())
}


fn key_up_event(&mut self, _ctx: &mut Context,input: KeyInput) -> GameResult{
match input.keycode{
Some(KeyCode::Numpad1) => self.ch8.keypad[0x1] = false,
Some(KeyCode::Numpad2) => self.ch8.keypad[0x2] = false,
Some(KeyCode::Numpad3) => self.ch8.keypad[0x3] = false,
Some(KeyCode::Numpad4) => self.ch8.keypad[0xc] = false,
Some(KeyCode::Q) => self.ch8.keypad[0x4] = false,
Some(KeyCode::W) => self.ch8.keypad[0x5] = false,
Some(KeyCode::E) => self.ch8.keypad[0x6] = false,
Some(KeyCode::R) => self.ch8.keypad[0xd] = false,
Some(KeyCode::A) => self.ch8.keypad[0x7] = false,
Some(KeyCode::S) =>  self.ch8.keypad[0x8] = false,
Some(KeyCode::D) =>  self.ch8.keypad[0x9] = false,
Some(KeyCode::F) =>  self.ch8.keypad[0xe] = false,
Some(KeyCode::Z) => self.ch8.keypad[0xa] = false,
Some(KeyCode::X) => self.ch8.keypad[0x0] = false,
Some(KeyCode::C) => self.ch8.keypad[0xb] = false,
Some(KeyCode::V) => self.ch8.keypad[0xf]= false,
_ => ()
}
Ok(())
}




}// Event Handler for Display



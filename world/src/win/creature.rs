#![allow(unused_imports, dead_code, unused_variables)]

use glium::backend::glutin_backend::GlutinFacade;

pub struct Creature {
    position: (f32, f32),
}

impl Creature {
    pub fn new() -> Creature {
        Creature { position: (0.0, 0.0) }
    }

    pub fn draw(&self, display: &GlutinFacade) {
        
    }
}
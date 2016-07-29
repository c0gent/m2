#![allow(unused_imports, dead_code, unused_variables)]

use glium::backend::glutin_backend::GlutinFacade;
use win::creature::Creature;
use util::Vertex;

pub enum EntityKind {
    Creature(Creature),
    Food,
    Poison,
    Unknown,
}

pub struct Entity {
    kind: EntityKind,
    verts: Vec<Vertex>
}

impl Entity {
    pub fn draw(&self, display: &GlutinFacade) {
        match self.kind {
            EntityKind::Creature(ref c) => c.draw(display),
            EntityKind::Food => (),
            EntityKind::Poison => (),
            _ => (),
        }
    }

    fn gen_verts(&self) {

    }
}
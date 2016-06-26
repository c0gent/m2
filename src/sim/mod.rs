//! The core simulation snapshot.
//!
//!
//!

#![allow(dead_code)]

pub enum Object {
    Entity,
    Cube,
    Thing2,
}

pub struct Snapshot {
    objects: Vec<Object>,
    positions: Vec<[f32; 4]>,
    orientations: Vec<[f32; 4]>,
    velocities: Vec<[f32; 4]>,
}

impl Snapshot {
    pub fn new() -> Snapshot {
        Snapshot {
            objects: Vec::with_capacity(128),
            positions: Vec::with_capacity(128),
            orientations: Vec::with_capacity(128),
            velocities: Vec::with_capacity(128),
        }
    }

    pub fn new_entity(&mut self, kind: Object) {
        self.objects.push(kind);
        self.positions.push([0.0, 0.0, 0.0, 0.0]);
        self.orientations.push([0.0, 0.0, 0.0, 0.0]);
        self.velocities.push([0.0, 0.0, 0.0, 0.0]);
    }

    pub fn cycle() {

    }
}
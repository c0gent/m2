//! The core simulation snapshot.
//!
//!
//!

#![allow(dead_code)]

use containers::{NodeList, Node};
pub type ModelId = usize;


#[derive(Clone, Debug)]
pub enum Object {
    Unknown,
    Entity(Option<ModelId>),
    Cube,
    Thing2,
}

impl Default for Object {
    fn default() -> Object {
        Object::Unknown
    }
}


pub struct Snapshot {
    nodes: NodeList<Object>,
}

impl Snapshot {
    pub fn new() -> Snapshot {
        Snapshot {
            nodes: NodeList::new(),
        }
    }

    pub fn new_entity(&mut self, position: [f32; 3], size: f32, model_id: ModelId) {
        self.nodes.append(position, size, Object::Entity(Some(model_id)));
    }

    pub fn cycle() {

    }

    pub fn nodes(&self) -> &[Option<Node<Object>>] {
        &self.nodes.as_slice()
    }
}

// pub struct Snapshot {
//     objects: Vec<Object>,
//     positions: Vec<[f32; 4]>,
//     orientations: Vec<[f32; 4]>,
//     velocities: Vec<[f32; 4]>,
// }

// impl Snapshot {
//     pub fn new() -> Snapshot {
//         Snapshot {
//             objects: Vec::with_capacity(128),
//             positions: Vec::with_capacity(128),
//             orientations: Vec::with_capacity(128),
//             velocities: Vec::with_capacity(128),
//         }
//     }

//     pub fn new_entity(&mut self, kind: Object) {
//         self.objects.push(kind);
//         self.positions.push([0.0, 0.0, 0.0, 0.0]);
//         self.orientations.push([0.0, 0.0, 0.0, 0.0]);
//         self.velocities.push([0.0, 0.0, 0.0, 0.0]);
//     }

//     pub fn cycle() {

//     }
// }
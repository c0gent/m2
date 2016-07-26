//! The core simulation snapshot.
//!
//!
//!

#![allow(dead_code, unused_imports)]

use time::{self, Timespec, Duration};

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
    start_time: Timespec,
}

impl Snapshot {
    pub fn new() -> Snapshot {
        Snapshot {
            nodes: NodeList::new(),
            start_time: time::get_time(),
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

    pub fn start_time(&self) -> &Timespec {
        &self.start_time
    }

    pub fn elapsed_secs(&self) -> f32 {
        (time::get_time() - self.start_time).num_seconds() as f32
    }

    /// Returns microseconds elapsed since the window was created (mu = μ).
    pub fn elapsed_mus(&self) -> f64 {
        (time::get_time() - self.start_time).num_microseconds().unwrap() as f64
    }

    /// Returns milliseconds elapsed since the window was created.
    pub fn elapsed_ms(&self) -> f64 {
        (time::get_time() - self.start_time).num_milliseconds() as f64
    }

    // /// Increment the frame counter by one and calculate fps for previous frame.
    // pub fn incr(&mut self) {
    //     let now = time::get_time();

    //     let prev_frame_dur = now - self.prev_event;
    //     self.cur_fps = Duration::seconds(1).num_microseconds().unwrap() as f32
    //         / prev_frame_dur.num_microseconds().unwrap() as f32;

    //     self.frame_count += 1;
    //     self.prev_event = now;
    // }
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
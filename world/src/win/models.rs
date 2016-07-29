#![allow(dead_code)]

use std::path::Path;
use std::collections::HashMap;
use std::ops::Range;
use glium::vertex::{VertexBuffer, VertexBufferSlice};
use glium::backend::glutin_backend::GlutinFacade;
use util;
use util::Vertex;
use win::vertex;


// static MODEL_FILES: [&'static str; 2] = [
//     "/home/nick/models/cube_tri.obj",
//     "/home/nick/models/cube_tri.obj",
// ];


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ModelKind {
    None,
    Cube,
    CrazyShape,
    Cube2,
}

impl ModelKind {
    pub fn model_list() -> Vec<(ModelKind, Option<&'static str>)> {
        vec![
            (ModelKind::None, None),
            (ModelKind::Cube, Some("/home/nick/models/cube_tri.obj")),
            (ModelKind::Cube2, Some("/home/nick/models/cube_tri.obj")),
            (ModelKind::CrazyShape, Some("/home/nick/models/crazy_shape.obj")),
        ]
    }
}


pub struct Models {
    model_vbos: Vec<VertexBuffer<Vertex>>,
    model_kind_indices: HashMap<ModelKind, usize>,
}

impl Models {
    pub fn new(display: &GlutinFacade) -> Models {
        let mut model_vbos = Vec::with_capacity(128);
        let mut model_kind_indices = HashMap::with_capacity(128);

        for (kind, file_name) in ModelKind::model_list().into_iter() {
            if let Some(file_name) = file_name {
                model_kind_indices.insert(kind, model_vbos.len());
                model_vbos.push(VertexBuffer::immutable(display,
                    &util::load_wavefront(&Path::new(file_name))).unwrap());
            } else {
                model_kind_indices.insert(kind, model_vbos.len());
                model_vbos.push(VertexBuffer::empty_immutable(display, 0).unwrap());
            }
        }

        Models {
            model_vbos: model_vbos,
            model_kind_indices: model_kind_indices,
        }
    }

    pub fn vbo(&self, model_id: usize) -> &VertexBuffer<Vertex> {
        self.model_vbos.get(model_id).unwrap()
    }

    pub fn model_index(&self, kind: &ModelKind) -> usize {
        *self.model_kind_indices.get(kind).expect("Models::model_index: Internal error.")
    }

    pub fn count(&self) -> usize {
        self.model_vbos.len()
    }
}

// pub fn hexagon_panel(scl: f32, ew: f32, depth: f32, color: [f32; 4])
//             -> (Vec<vertex::Vertex>, Vec<u16>) {
//     use win::vertex::Vertex;

//     let a = scl * 0.5;
//     let s = scl * 0.577350269192; // 1/sqrt(3)
//     let hs = s / 2.0f32;

//     let vertices = vec![
//         Vertex::new([ 0.0,          0.0,      depth], color, [0.0, 0.0, -1.0], false),
//         Vertex::new([-(hs + ew),     a,       depth], color, [0.0, 0.0, -1.0], true),
//         Vertex::new([ hs + ew,      a,       depth], color, [0.0, 0.0, -1.0], true),
//         Vertex::new([ s + ew,      0.0,       depth], color, [0.0, 0.0, -1.0], true),
//         Vertex::new([ hs + ew,     -a,      depth], color, [0.0, 0.0, -1.0], true),
//         Vertex::new([-(hs + ew),     -a,       depth], color, [0.0, 0.0, -1.0], true),
//         Vertex::new([-(s + ew),       0.0,       depth], color, [0.0, 0.0, -1.0], true),
//     ];

//     let indices = vec![
//         0, 1, 2,
//         2, 3, 0,
//         0, 3, 4,
//         4, 5, 0,
//         0, 5, 6,
//         6, 1, 0u16,
//     ];

//     // let perim = vec![
//     //     1, 2, 3, 4, 5, 6,
//     // ];

//     // let radii = (ew + (s * 0.75), a);

//     (vertices, indices)
// }

// pub fn cube(scl: f32, color: [f32; 4]) -> (Vec<Vertex>, Vec<u16>) {
//     let vertices = vec![
//         Vertex::new([-1,    -1,    1], color, [0.0, 0.0, -1.0], true),
//         Vertex::new([-1,    -1,    1], color, [0.0, 0.0, -1.0], true),
//     ];

//     let indices = vec![
//         0, 1, 2,
//         2, 3, 0,
//         0, 3, 4,
//         4, 5, 0,
//         0, 5, 6,
//         6, 1, 0u16,
//     ];

//     // let perim = vec![
//     //     1, 2, 3, 4, 5, 6,
//     // ];

//     // let radii = (ew + (s * 0.75), a);

//     (vertices, indices)
// }
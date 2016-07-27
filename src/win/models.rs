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

    // pub fn model_file(&self) -> Option<&'static str> {
    //     match *self {
    //         ModelKind::None => None,
    //         ModelKind::Cube => Some("/home/nick/models/cube_tri.obj"),
    //     }
    // }
}


pub struct Models {
    model_vertex_ranges: Vec<Range<usize>>,
    model_kind_indices: HashMap<ModelKind, usize>,
    vbo: VertexBuffer<Vertex>,
}

impl Models {
    pub fn new(display: &GlutinFacade) -> Models {
        let mut vertices = Vec::with_capacity(4096);

        let mut model_vertex_ranges = Vec::with_capacity(128);
        let mut model_kind_indices = HashMap::with_capacity(128);
        let mut prev_idz = 0;

        for (kind, file_name) in ModelKind::model_list().into_iter() {
            if let Some(file_name) = file_name {
                vertices.extend_from_slice(&util::load_wavefront(&Path::new(file_name)));
                model_kind_indices.insert(kind, model_vertex_ranges.len());
                model_vertex_ranges.push(prev_idz..vertices.len());
                prev_idz = vertices.len();
            } else {
                model_kind_indices.insert(kind, model_vertex_ranges.len());
                model_vertex_ranges.push(prev_idz..vertices.len());
            }
        }

        let vbo = VertexBuffer::dynamic(display, &vertices).unwrap();

        Models {
            model_vertex_ranges: model_vertex_ranges,
            model_kind_indices: model_kind_indices,
            vbo: vbo,
        }
    }

    pub fn vbo(&self) -> &VertexBuffer<Vertex> {
        &self.vbo
    }

    pub fn verts(&self, model_id: usize) -> VertexBufferSlice<Vertex> {
        assert!(model_id < self.model_vertex_ranges.len(), "Models::model_verts: 'model_id' \
            out of range.");
        let model_vertex_range = self.model_vertex_ranges[model_id].clone();
        // println!("Slicing model VBO using range: {:?}", model_vertex_range);
        assert!(model_vertex_range.end <= self.vbo.len());
        self.vbo.slice(model_vertex_range)
            .expect("Models::model_verts: Internal error: invalid slice range.")
    }

    pub fn model_index(&self, kind: &ModelKind) -> usize {
        *self.model_kind_indices.get(kind).expect("Models::model_index: Internal error.")
    }

    pub fn vert_ranges(&self) -> &[Range<usize>] {
        &self.model_vertex_ranges.as_slice()
    }

    pub fn count(&self) -> usize {
        self.model_vertex_ranges.len()
    }
}



pub fn hexagon_panel(scl: f32, ew: f32, depth: f32, color: [f32; 4])
            -> (Vec<vertex::Vertex>, Vec<u16>) {
    use win::vertex::Vertex;

    let a = scl * 0.5;
    let s = scl * 0.577350269192; // 1/sqrt(3)
    let hs = s / 2.0f32;

    let vertices = vec![
        Vertex::new([ 0.0,          0.0,      depth], color, [0.0, 0.0, -1.0], false),
        Vertex::new([-(hs + ew),     a,       depth], color, [0.0, 0.0, -1.0], true),
        Vertex::new([ hs + ew,      a,       depth], color, [0.0, 0.0, -1.0], true),
        Vertex::new([ s + ew,      0.0,       depth], color, [0.0, 0.0, -1.0], true),
        Vertex::new([ hs + ew,     -a,      depth], color, [0.0, 0.0, -1.0], true),
        Vertex::new([-(hs + ew),     -a,       depth], color, [0.0, 0.0, -1.0], true),
        Vertex::new([-(s + ew),       0.0,       depth], color, [0.0, 0.0, -1.0], true),
    ];

    let indices = vec![
        0, 1, 2,
        2, 3, 0,
        0, 3, 4,
        4, 5, 0,
        0, 5, 6,
        6, 1, 0u16,
    ];

    // let perim = vec![
    //     1, 2, 3, 4, 5, 6,
    // ];

    // let radii = (ew + (s * 0.75), a);

    (vertices, indices)
}

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
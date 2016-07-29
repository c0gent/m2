#![allow(dead_code, unused_imports, unused_mut)]

use std::path::Path;
use obj::{self, SimplePolygon};
use genmesh::{self, Polygon};
// use glium::{Display};
// use glium::vertex::VertexBufferAny;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture: [f32; 2]
}

implement_vertex!(Vertex, position, normal, texture);

/// Returns a column-major perspective matrix.
pub fn persp_matrix(width: u32, height: u32, fov_zoom: f32) -> [[f32; 4]; 4] {
    let zfar = 1024.0;
    let znear = 0.1;

    // let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;
    let fov: f32 = 3.141592 / fov_zoom;
    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}

/// Returns a column-major view matrix.
pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s[0], u[0], f[0], 0.0],
        [s[1], u[1], f[1], 0.0],
        [s[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}


/// Returns a vec suitable for creation of a vertex buffer that should be
/// rendered as `TrianglesList`.
pub fn load_wavefront_obj_genmesh(data: &[u8]) -> Vec<Vertex> {
    let mut data = ::std::io::BufReader::new(data);
    let data = obj::Obj::load(&mut data);

    let mut vertex_data = Vec::new();

    for object in data.object_iter() {
        for shape in object.group_iter().flat_map(|g| g.indices().iter()) {
            match shape {
                &genmesh::Polygon::PolyTri(genmesh::Triangle { x: v1, y: v2, z: v3 }) => {
                    for v in [v1, v2, v3].iter() {
                        let position = data.position()[v.0];
                        let texture = v.1.map(|index| data.texture()[index]);
                        let normal = v.2.map(|index| data.normal()[index]);

                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        vertex_data.push(Vertex {
                            position: position,
                            normal: normal,
                            texture: texture,
                        })
                    }
                },
                _ => unimplemented!()
            }
        }
    }

    // glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into_vertex_buffer_any()
    vertex_data
}



/// Returns a vec suitable for creation of a vertex buffer that should be
/// rendered as `TrianglesList`.
pub fn load_wavefront(data_path: &Path) -> Vec<Vertex> {
    // let mut data = ::std::io::BufReader::new(data);
    // let data = obj::Obj::load::<SimplePolygon>(data).unwrap();

    let data_file = ::std::fs::File::open(data_path).expect("util::load_wavefront(): Invalid file path");
    let mut data_buf = ::std::io::BufReader::new(data_file);
    let data = obj::Obj::load(&mut data_buf);

    let mut vertices = Vec::new();

    for object in data.object_iter() {
        for shape in object.group_iter().flat_map(|group| group.indices().iter()) {
            match *shape {
                Polygon::PolyTri(genmesh::Triangle { x: v0, y: v1, z: v2 }) => {
                    for v in [v0, v1, v2].iter() {
                        let position = data.position()[v.0];
                        let texture = v.1.map(|index| data.texture()[index]);
                        let normal = v.2.map(|index| data.normal()[index]);

                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        vertices.push(Vertex {
                            position: position,
                            normal: normal,
                            texture: texture,
                        })
                    }
                },
                _ => unimplemented!()
            }
        }
    }

    // glium::vertex::VertexBuffer::new(display, &vertices).unwrap().into_vertex_buffer_any()
    vertices
}



//         #[derive(Copy, Clone)]
//         pub struct Vertex {
//             position: [f32; 3],
//             normal: [f32; 3],
//         }

//         implement_vertex!(Vertex, position, normal);

//         let obj = tobj::load_obj(&Path::new("/home/nick/models/cube_tri.obj"));
//         assert!(obj.is_ok());
//         let (models, mats) = obj.unwrap();
//         assert!(mats.is_empty());

//         let mesh = &models[0].mesh;

//         self.vbo = Some(VertexBuffer::dynamic(self.display, models.vertices).unwrap());
//         self.ibo = Some(IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList,
//             models.indices).unwrap());
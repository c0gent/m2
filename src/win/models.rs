#![allow(dead_code)]

use win::vertex::Vertex;

pub fn hexagon_panel(scl: f32, ew: f32, depth: f32, color: [f32; 4])
            -> (Vec<Vertex>, Vec<u16>) {
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

// [FIXME]: TODO: 
// - Seriously revamp this a fix all the extra allocations etc.
//    - ^ kinda halfway done...
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
    normal: [f32; 3],
    is_perimeter: bool,
}

implement_vertex!(Vertex, position, color, normal);

impl Vertex {
    #[allow(dead_code)]
    pub fn new(position: [f32; 3], color: [f32; 4], normal: [f32; 3], is_perimeter: bool) 
            -> Vertex 
    {
        Vertex { position: position, color: color, normal: normal, is_perimeter: is_perimeter }
    }

    #[allow(dead_code)]
    pub fn color(mut self, color: [f32; 4]) -> Vertex {
        self.color = color;
        self
    }

    #[allow(dead_code)]
    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    #[allow(dead_code)]
    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }

    #[allow(dead_code)]
    pub fn is_perimeter(&self) -> bool {
        self.is_perimeter
    }
}



// // TODO: Combine into transform().
// fn shift(position: &[f32; 3], shift_by: &[f32; 3]) -> [f32; 3] {
//     [
//         position[0] + shift_by[0],
//         position[1] + shift_by[1],
//         position[2] + shift_by[2],
//     ]
// }

// // TODO: Combine into transform().
// fn scale(position: &[f32; 3], scale_by: &[f32; 3]) -> [f32; 3] {
//     [
//         position[0] * scale_by[0],
//         position[1] * scale_by[1],
//         position[2] * scale_by[2],
//     ]
// }

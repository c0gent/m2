#![allow(dead_code)]

/// A cube-shaped node within 3d space with center position: `pos`, and
/// edge length: `size`.
///
#[derive(Clone, Debug)]
pub struct Node<T: Clone + Default> {
    pub position: [f32; 3],
    pub orientation: [f32; 4],
    pub velocity: [f32; 4],
    pub size: f32,
    pub payload: T,
}

impl<T: Clone + Default> Node<T> {
    pub fn new() -> Node<T> {
        Node::default()
    }
}

impl<T: Clone + Default> Default for Node<T> {
    fn default() -> Node<T> {
        Node {
            position: [0.0, 0.0, 0.0],
            size: 0.0,
            orientation: [0.0, 0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0, 0.0],
            payload: T::default(),
        }
    }
}


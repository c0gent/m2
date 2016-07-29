#![allow(dead_code)]

use containers::node::Node;

#[derive(Clone, Debug)]
pub struct NodeList<T: Clone + Default> {
    nodes: Vec<Option<Node<T>>>,
}

impl<T: Clone + Default> NodeList<T> {
    pub fn new() -> NodeList<T> {
        NodeList {
            nodes: Vec::with_capacity(256),
        }
    }

    pub fn append(&mut self, position: [f32; 3], size: f32, payload: T) {
        self.append_node(Node { position: position, size: size, payload: payload, ..Node::default() });
    }

    pub fn append_node(&mut self, node: Node<T>) {
        self.nodes.push(Some(node));
    }

    pub fn get(&self, idx: usize) -> Option<&Node<T>> {
        match self.nodes.get(idx) {
            Some(n) => {
                match *n {
                    Some(ref n) => Some(n),
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Node<T>> {
        match self.nodes.get_mut(idx) {
            Some(n) => {
                match *n {
                    Some(ref mut n) => Some(n),
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn remove(&mut self, idx: usize) -> Option<Node<T>> {
        if idx < self.nodes.len() {
            let node = self.nodes[idx].clone();

            // if idx happens to be the last index, just pop it, otherwise set
            // to `None` for later clean up.
            if idx == self.nodes.len() - 1 {
                self.nodes.pop();
            } else {
                self.nodes[idx] = None;
            }
            node
        } else {
            None
        }
    }

    // Condense the list in-place, removing any `None` nodes.
    pub fn condense(&mut self) {
        let mut new_len = 0usize;

        unsafe {
            for i in 0..self.nodes.len() {
                if self.nodes.get_unchecked_mut(i).is_some() {
                    *self.nodes.get_unchecked_mut(new_len) = self.nodes.get_unchecked(i).clone();
                    new_len += 1;
                }
            }
            self.nodes.set_len(new_len);
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn as_slice(&self) -> &[Option<Node<T>>] {
        &self.nodes[..]
    }
}

#[cfg(test)]
mod tests {
    use super::NodeList;

    #[test]
    fn test_node_list() {
        let mut node_list = NodeList::new();
        let payload = "I'm the payload.".to_owned();

        node_list.append([0.0, 0.0, 0.0], 0.0, payload.clone());
        node_list.append([1.0, 1.0, 1.0], 1.0, payload.clone());
        node_list.append([2.0, 2.0, 2.0], 2.0, payload.clone());
        node_list.append([3.0, 3.0, 3.0], 3.0, payload.clone());
        node_list.append([4.0, 4.0, 4.0], 4.0, payload.clone());
        assert!(node_list.len() == 5);

        node_list.remove(1);
        node_list.remove(3);
        assert!(node_list.len() == 5);

        node_list.remove(4);
        assert!(node_list.len() == 4);

        let node_list_with_gaps = node_list.clone();
        node_list.condense();
        assert!(node_list.len() == 2);

        let mut node_list_idx = 0usize;

        // Check that each `Some` entry in the cloned version matches the
        // corresponding entry in the condensed version:
        for node in node_list_with_gaps.nodes().iter() {
            match *node {
                Some(ref n) => {
                    assert!(n.size == node_list.get(node_list_idx).unwrap().size);
                    assert!(n.position == node_list.get(node_list_idx).unwrap().position);
                    node_list_idx += 1;
                },
                None => (),
            }
        }

        // Make sure we checked every node in the condensed version:
        assert!(node_list_idx == node_list.len());
    }
}
//! Octree (proper) raw pointer implementation.
//!
//! Various methods of implementing this structure (and others like it) such
//! as those suggested
//! [here](https://github.com/nrc/r4cppp/tree/master/graphs) and
//! [here](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/)
//! are simply inadequate and overly cumbersome. The borrow checker is not
//! quite up to handling structures like this.
//!
//!
//! [TODO]:
//!
//! - Build an allocator (as simply as possible).
//!
//!
//! [NOTES]:
//!
//! Design goals:
//! - Automatically spawn sub-octants when a maximum occupancy is reached.
//! - Automatically shrink back when occupancy reaches a minimum.
//! - (tenative) Be able to grow outside borders to handle objects outside the
//!   `root` size parameters.
//!

#![allow(dead_code)]

#[derive(Copy, Clone)]
struct Octant<T> {
    payload: Option<T>,
    parent: *mut Octant<T>,
    children: [*mut Octant<T>; 8],
}

impl<T> Octant<T> {
    pub fn new() -> Octant<T> {
        Octant {
            payload: None,
            parent: 0 as *mut Octant<T>,
            children: [0 as *mut Octant<T>; 8],
        }
    }

    pub fn parent(&self) -> *mut Octant<T> {
        self.parent
    }

    pub fn payload(&self) -> &Option<T> {
        &self.payload
    }

    pub fn payload_mut(&mut self) -> &mut Option<T> {
        &mut self.payload
    }

    fn set_children(&mut self, children: [*mut Octant<T>; 8]) {
        self.children = children;
    }
}

pub struct Octree<T> {
    octants: Vec<Option<Octant<T>>>,
    root: [*mut Octant<T>; 8],

}

impl<T> Octree<T> {
    pub fn new() -> Octree<T> {
        Octree {
            octants: Vec::with_capacity(2 << 16),
            root: [0 as *mut Octant<T>; 8],
        }
    }
}



#[cfg(test)]
pub mod tests {
    #![allow(dead_code, unused_imports)]
    use super::{Octant, Octree};

    #[test]
    pub fn test_create() {
        let octant = Octant::<u8>::new();

        assert!(octant.parent.is_null());

        for child in octant.children.iter() {
            assert!(child.is_null());
        }
    }

    #[test]
    pub fn test_access_payloads() {
        #[derive(Copy, Clone, PartialEq)]
        struct PayloadData(usize);

        // let mut octree = Octree::<PayloadData>::new();
        let mut octant = Octant::<PayloadData>::new();


        let mut top_level_children = [Octant::<PayloadData>::new(); 8];
        let mut top_level_children_ptrs = [0 as *mut Octant<PayloadData>; 8];

        for (child_ptr, child) in top_level_children_ptrs.iter_mut().zip(&mut top_level_children) {
            *child_ptr = child as *mut Octant<PayloadData>;
        }

        octant.children = top_level_children_ptrs;

        for &mut child_ptr in octant.children.iter_mut() {
            unsafe { (*(*child_ptr).payload_mut()) = Some(PayloadData(5)); }
            // unsafe { (**child_ptr).payload_mut(); }
        }

        for &child_ptr in octant.children.iter() {
            assert!(unsafe { (*child_ptr).payload == Some(PayloadData(5)) });
        }
    }
}


// use std::cell::UnsafeCell;
// use arena::TypedArena;

// struct Octant<'a, T: 'a> {
//     payload: Option<T>,
//     parent: Option<&'a Octant<'a, T>>,
//     children: UnsafeCell<[Option<&'a Octant<'a, T>>; 8]>,
// }

// /// An Octree.
// ///
// /// `root`? rename?
// pub struct Octree<'a, T: 'a> {
//     arena: TypedArena<Octant<'a, T>>,
//     // children: [UnsafeCell<Option<&'a Octant<'a, T>>>; 8],
//     // root: Octant<'a, T>,
//     root: Option<&'a mut Octant<'a, T>>,
// }

// impl<'a, T: 'a> Octree<'a, T> {
//     pub fn new() -> Octree<'a, T> {
//         // let arena: TypedArena<Octant<'a, T>> = TypedArena::with_capacity(1024);
//         // let root = Octant {
//         //     payload: None,
//         //     parent: None,
//         //     children: UnsafeCell::new([None, 8]),
//         // };

//         let mut octree = Octree {
//             arena: TypedArena::with_capacity(1024),
//             root: None,
//         };

//         octree.root = Some(octree.arena.alloc(Octant {
//             payload: None,
//             parent: None,
//             children: UnsafeCell::new([None; 8]),
//         }));

//         octree
//     }
// }






////// GAMEDEV.NET IMPLEMENTATION
//////
//////
//////
// public class OctTree
// {
//     BoundingBox m_region;

//     List<Physical> m_objects;

//     /// <summary>
//     /// These are items which we're waiting to insert into the data structure.
//     /// We want to accrue as many objects in here as possible before we inject them into the tree. This is slightly more cache friendly.
//     /// </summary>
//     static Queue<Physical> m_pendingInsertion = new Queue<Physical>();

//     /// <summary>
//     /// These are all of the possible child octants for this node in the tree.
//     /// </summary>
//     OctTree[] m_childNode = new OctTree[8];

//     /// <summary>
//     /// This is a bitmask indicating which child nodes are actively being used.
//     /// It adds slightly more complexity, but is faster for performance since there is only one comparison instead of 8.
//     /// </summary>
//     byte m_activeNodes = 0;

//     /// <summary>
//     /// The minumum size for enclosing region is a 1x1x1 cube.
//     /// </summary>
//     const int MIN_SIZE = 1;

//     /// <summary>
//     /// this is how many frames we'll wait before deleting an empty tree branch. Note that this is not a constant. The maximum lifespan doubles
//     /// every time a node is reused, until it hits a hard coded constant of 64
//     /// </summary>
//     int m_maxLifespan = 8;          //
//     int m_curLife = -1;             //this is a countdown time showing how much time we have left to live

//     /// <summary>
//     /// A reference to the parent node is nice to have when we're trying to do a tree update.
//     /// </summary>
//     OctTree _parent;

//     static bool m_treeReady = false;       //the tree has a few objects which need to be inserted before it is complete
//     static bool m_treeBuilt = false;       //there is no pre-existing tree yet.

// }
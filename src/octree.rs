//! Octree (oct-tree) implementation.
//!
//!
//!

#![allow(dead_code)]

use std::cell::UnsafeCell;
use arena::TypedArena;

struct Octant<'a, T: 'a> {
    payload: Option<T>,
    parent: Option<&'a Octant<'a, T>>,
    children: UnsafeCell<[Option<&'a Octant<'a, T>>; 8]>,
}

/// An Octree.
///
/// `root`? rename?
pub struct Octree<'a, T: 'a> {
    arena: TypedArena<Octant<'a, T>>,
    // children: [UnsafeCell<Option<&'a Octant<'a, T>>>; 8],
    // root: Octant<'a, T>,
    root: Option<&'a mut Octant<'a, T>>,
}

impl<'a, T: 'a> Octree<'a, T> {
    pub fn new() -> Octree<'a, T> {
        // let arena: TypedArena<Octant<'a, T>> = TypedArena::with_capacity(1024);
        // let root = Octant {
        //     payload: None,
        //     parent: None,
        //     children: UnsafeCell::new([None, 8]),
        // };

        let mut octree = Octree {
            arena: TypedArena::with_capacity(1024),
            root: None,
        };

        octree.root = Some(octree.arena.alloc(Octant {
            payload: None,
            parent: None,
            children: UnsafeCell::new([None; 8]),
        }));

        octree
    }
}






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
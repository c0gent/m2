//! An experimental, 27 bucket, raw pointer, performance focused icospetree.
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


// The 'payload'. Right now represented by just `T`.
struct Leaf(usize);


/// Component containers of an Icospetree
///
///
#[derive(Clone)]
struct Subspace<T> {
    payload: Option<Vec<T>>,
    parent: *mut Icoseptant<T>,
    children: Option<[*mut Icoseptant<T>; 8]>,
}





impl<T> Icoseptant<T> {
    pub fn new() -> Icoseptant<T> {
        Icoseptant {
            payload: Some(Vec::with_capacity(9)),
            parent: 0 as *mut Icoseptant<T>,
            children: Some([0 as *mut Icoseptant<T>; 8]),
        }
    }
}


/// A tree shaped graph used to represent 3d space where each node can have a
/// maximum of 27 children, one for each octant and one for each border
/// between octants.
///
///
#[derive(Clone)]
pub struct Icoseptree<T> {
    octants: Vec<Icoseptant<T>>,
    // borders: Vec<
    root: [*mut Icoseptant<T>; 8],

}

impl<T> Icoseptree<T> {
    pub fn new() -> Icoseptree<T> {
        Icoseptree {
            octants: Vec::with_capacity(2 << 16),
            root: [0 as *mut Icoseptant<T>; 8],
        }
    }
}





#[cfg(test)]
pub mod tests {
    #![allow(dead_code, unused_imports, unused_variables, unused_mut)]
    use super::{Icoseptant, Icoseptree};

    #[test]
    pub fn test_create() {
        let octant = Icoseptant::<u8>::new();

        assert!(octant.parent.is_null());
    }

    #[test]
    pub fn test_access_payloads() {
        #[derive(Copy, Clone, PartialEq)]
        struct PayloadData(usize);

        // let mut octree = Icoseptree::<PayloadData>::new();
        let mut octant = Icoseptant::<PayloadData>::new();
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
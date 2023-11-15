//! The file for functions that partition the level geometry

#[allow(unused_imports)]

use crate::{*, geometry::*};
use binary_tree::*;

/// Split a vector of segs into a node with itself as root
/// and the child nodes containing on the left, nodes behind,
/// and on the right, nodes in front.
fn front_back(segs: Vec<Seg>) -> NodeRef<Vec<Seg>> {
    // Give it children. We can unwrap here as we know that we 
    // will not get a panic because we are not trying to 
    // overwrite a preexisting child.
    let mut segs_node = Node::new(segs);

    segs_node.create_child(vec![], Direction::Left).unwrap();
    segs_node.create_child(vec![], Direction::Right).unwrap();

    // Iterate over the root node's item, adding to each child
    // for front and back.
    let root: Seg = segs_node.value[0];
    segs_node.value.iter_mut().for_each(|seg| {
        match root.determine_side(seg.clone()) {
            Side::Front => segs_node
                .right.as_mut()
                // This should never happen
                .ok_or_else(|| panic!())
                .unwrap()
                .borrow_mut()
                .value.push(*seg),
            Side::Back => segs_node
                .left.as_mut()
                .ok_or_else(|| panic!())
                .unwrap()
                .borrow_mut()
                .value.push(*seg),
            Side::Neither => {
                // Split the seg
                let split = seg.split_by(root);
                // Make sure it split
                match split {
                    // If it didn't, just send it to the back
                    None =>  segs_node
                        .left.as_mut()
                        .ok_or_else(|| panic!())
                        .unwrap()
                        .borrow_mut()
                        .value.push(*seg),
                    Some(segs) => {
                        let segvec = vec![segs.0, segs.1];
                        for seg in segvec {
                            match root.determine_side(seg.clone()) {
                                Side::Front => segs_node
                                    .right.as_mut()
                                    // This should never happen
                                    .ok_or_else(|| panic!())
                                    .unwrap()
                                    .borrow_mut()
                                    .value.push(seg),
                                Side::Back => segs_node
                                    .left.as_mut()
                                    .ok_or_else(|| panic!())
                                    .unwrap()
                                    .borrow_mut()
                                    .value.push(seg),
                                // It should never be neither again, so we panic.
                                Side::Neither => panic!(),
                            }
                        }
                    }
                }
            },
        }
    });

    segs_node.noderef()
}

// this code sucks
pub fn recursive_partition(seglists: Vec<Vec<Seg>>) -> Vec<Seg> {
    let mut new_seglists: Vec<Vec<Seg>> = Vec::new();
    // iterate over the segs
    for segs in seglists {
        // if the length is 1 do not touch it
        match segs.len() {
            1usize => new_seglists.push(segs.clone()),
            _ => {
            // If length is not 1, we must partition, get the front and back then recurse
                let partitioned = front_back(segs.to_owned()); 
                let front_segs = partitioned
                    .borrow()
                    .get_child_reference(Direction::Left)
                    .unwrap()
                    .borrow()
                    .value.clone();
                let back_segs = partitioned
                    .borrow()
                    .get_child_reference(Direction::Right)
                    .unwrap()
                    .borrow()
                    .value.clone();
                
                // Put the two halves into one thing to recurse with
                new_seglists.push(recursive_partition(vec![front_segs, back_segs]));
            }
        }
    }     

    new_seglists.iter().fold(vec![], |acc: Vec<Seg>, segs| {
        let mut new_segs = acc.clone();
        new_segs.push(segs[0]);
        new_segs
    })
}

#[allow(unused_imports)]
pub mod tests {
    use crate::partitioning::*;
    use crate::geometry::tests::init;

    #[test]
    fn no_panic_sanity_check() {
        let (seg0, seg1, seg2, seg3) = init();
        let segvec = vec![seg0, seg1, seg2, seg3];
        dbg!(front_back(segvec));
    }   
}
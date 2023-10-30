//! The file for functions that partition the level geometry

use crate::{*, geometry::*};
use binary_tree::*;

/// Split a vector of segs into a node with itself as root
/// and the child nodes containing on the left, nodes behind,
/// and on the right, nodes in front.
fn front_back(segs: Vec<Seg>) -> NodeRef<Vec<Seg>> {
    // Make the root node
    let mut segs_node = Node::new(segs);

    // Give it children. We can unwrap here as we know that we 
    // will not get a panic because we are not trying to 
    // overwrite a preexisting child.
    segs_node.create_child(vec![], Direction::Left).unwrap();
    segs_node.create_child(vec![], Direction::Right).unwrap();

    // Iterate over the root node's item, adding to each child
    // for front and back.
    let root: Seg = segs_node.value[0];
    segs_node.value.iter_mut().for_each(|seg| {
        let side = root.determine_side(seg.clone());
        
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

pub mod tests {
    use crate::partitioning::*;
    use crate::geometry::tests::init;

    #[test]
    fn no_panic_sanity_check() {
        let (mut seg0, mut seg1, mut seg2, mut seg3) = init();
        let segvec = vec![seg0, seg1, seg2, seg3];
        dbg!(front_back(segvec));
    }   
}
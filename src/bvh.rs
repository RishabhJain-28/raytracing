use crate::{aabb, hit, Hitable, AABB};
use rand::Rng;
use std::cmp::Ordering;

enum BVHNode {
    Branch {
        left: Box<dyn Hitable>,
        right: Box<dyn Hitable>,
    },
    Leaf(Box<dyn Hitable>),
}

struct BVH {
    tree: BVHNode,
    bbox: AABB,
}
fn box_compare(
    time0: f64,
    time1: f64,
    axis: usize,
) -> impl FnMut(&Box<dyn Hitable>, &Box<dyn Hitable>) -> Ordering {
    move |a, b| {
        let a_bbox = a.bounding_box(time0, time1);
        let b_bbox = b.bounding_box(time0, time1);
        if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
            let ac = a.min()[axis] + a.max()[axis];
            let bc = b.min()[axis] + b.max()[axis];
            ac.partial_cmp(&bc).unwrap()
        } else {
            panic!["no bounding box in bvh node"]
        }
    }
}
// axis ranges to get best BVH
impl BVH {
    fn new(mut hitables: Vec<Box<dyn Hitable>>, time0: f64, time1: f64) -> Self {
        match hitables.len() {
            0 => panic!("[BHV::new] No objects in the scene"),
            1 => {
                let leaf = hitables.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(time0, time1) {
                    BVH {
                        tree: BVHNode::Leaf(leaf),
                        bbox,
                    }
                } else {
                    panic!("no bounding box in bvh node")
                }
            }
            _ => {
                let mut rng = rand::thread_rng();
                let axis = rng.gen_range(0..2);

                hitables.sort_unstable_by(box_compare(time0, time1, axis));

                let mid = hitables.len() / 2;
                let left = BVH::new(hitables.drain(0..mid).collect(), time0, time1);
                let right = BVH::new(hitables.drain(mid..).collect(), time0, time1);

                let left_bbox = left
                    .bounding_box(time0, time1)
                    .expect("no bounding box in left bvh node");

                let right_bbox = right
                    .bounding_box(time0, time1)
                    .expect("no bounding box in right bvh node");

                let bbox = AABB::surrounding_box(&left_bbox, &right_bbox);

                BVH {
                    tree: BVHNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                }
            }
        }
    }
}

impl Hitable for BVH {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<crate::HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        };

        match &self.tree {
            BVHNode::Branch { left, right } => {
                let left_hit_rec = left.hit(r, t_min, t_max);
                let t_max = if left_hit_rec.is_some() {
                    left_hit_rec.as_ref().unwrap().t
                } else {
                    t_max
                };
                let right_hit_rec = right.hit(r, t_min, t_max);
                if left_hit_rec.is_some() {
                    right_hit_rec
                } else {
                    left_hit_rec
                }
            }
            BVHNode::Leaf(leaf) => leaf.hit(r, t_min, t_max),
        }
    }
    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}

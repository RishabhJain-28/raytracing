use crate::{Hitable, Point3};

pub struct aabb {
    min: Point3,
    max: Point3,
}

impl aabb {
    pub fn new(min: Point3, max: Point3) -> aabb {
        aabb { min, max }
    }
    pub fn surrounding_box(box0: &aabb, box1: &aabb) -> aabb {
        let small = Point3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );
        let big = Point3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );
        aabb {
            min: small,
            max: big,
        }
    }
    pub fn min(&self) -> Point3 {
        return self.min;
    }
    pub fn max(&self) -> Point3 {
        return self.max;
    }
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let t1 = (self.max()[a] - r.origin()[a]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            let t_min = t_min.max(t0);
            let t_max = t_max.min(t1);

            if t_max <= t_min {
                return false;
            }
        }

        return true;
    }
}

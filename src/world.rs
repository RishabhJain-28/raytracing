use crate::{HitRecord, Hitable, Ray, AABB};

pub type World = Vec<Box<dyn Hitable>>;
pub type HittableList = World;

impl Hitable for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let bbox = self.first()?.bounding_box(time0, time1)?;
        let output_box = self.into_iter().skip(1).try_fold(bbox, |acc, obj| {
            let curr_bound = obj.bounding_box(time0, time1)?;
            Some(AABB::surrounding_box(&acc, &curr_bound))
        });

        output_box
    }
}

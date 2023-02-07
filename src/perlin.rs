use rand::Rng;

use crate::{Point3, Vec3};

pub struct Perlin {
    rand_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    fn permute(p: &mut [usize], n: usize) {
        let mut rng = rand::thread_rng();
        for i in (0..(p.len() - 1)).rev() {
            let target = rng.gen_range(0..=i);
            p.swap(target, i);
        }
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p: Vec<usize> = (0..Self::POINT_COUNT).collect();
        Self::permute(&mut p, Self::POINT_COUNT);
        p
    }
    fn trilinear_intp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    acc += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        acc
    }
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * c[i][j][k].dot(weight);
                }
            }
        }
        accum
    }

    pub fn new() -> Self {
        let mut rand_float = Vec::with_capacity(Self::POINT_COUNT);
        let mut rng = rand::thread_rng();
        for _ in 0..Self::POINT_COUNT {
            rand_float.push(Vec3::random(-1.0..1.0).normalized());
        }

        return Self {
            rand_vec: rand_float,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        };
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as usize;
        let j = p.y().floor() as usize;
        let k = p.z().floor() as usize;
        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[self.perm_x[((i + di) & 255)]
                        ^ self.perm_y[((j + dj) & 255)]
                        ^ self.perm_z[((k + dk) & 255)]];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }
    pub fn turb(&self, point: Point3, depth: Option<i32>) -> f64 {
        let depth = depth.unwrap_or(7);
        let mut accum = 0.0;
        let mut temp_p = point;
        let mut weight = 1.0;
        for i in 0..depth {
            accum += weight * Self::noise(&self, &temp_p);
            weight *= 0.25;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}

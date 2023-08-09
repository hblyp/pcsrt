use las::Point;
use nalgebra::{vector, Vector3};

use super::point::PointAsNaVec;

#[derive(Copy, Clone, Debug)]
pub struct NormalVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl NormalVector {
    pub fn _new() -> NormalVector {
        NormalVector {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
    pub fn upright() -> NormalVector {
        NormalVector {
            x: 0.,
            y: 0.,
            z: 1.,
        }
    }
    pub fn as_na_vec(&self) -> Vector3<f64> {
        Vector3::from([self.x, self.y, self.z])
    }
    pub fn from_na_vec(na_vec: &Vector3<f64>) -> Self {
        if na_vec[2] < 0. {
            NormalVector {
                x: -na_vec[0],
                y: -na_vec[1],
                z: -na_vec[2],
            }
        } else {
            NormalVector {
                x: na_vec[0],
                y: na_vec[1],
                z: na_vec[2],
            }
        }
    }
    pub fn angle(&self, vec: &Vector3<f64>) -> f64 {
        self.as_na_vec().angle(vec)
    }

    // https://www.ilikebigbits.com/2015_03_04_plane_from_points.html
    pub fn from_points(points: &[Point]) -> Option<Self> {
        let points: &Vec<Vector3<f64>> = &points.iter().map(|point| point.as_na_vec()).collect();

        if points.len() < 3 {
            return None;
        }

        let mut sum = vector![0.0, 0.0, 0.0];

        for p in points {
            sum += p;
        }

        let centroid = sum * (1.0 / (points.len() as f64));

        // Calc full 3x3 covariance matrix, excluding symmetries:
        let mut xx = 0.0;
        let mut xy = 0.0;
        let mut xz = 0.0;
        let mut yy = 0.0;
        let mut yz = 0.0;
        let mut zz = 0.0;

        for p in points {
            let r = p - centroid;
            xx += r.x * r.x;
            xy += r.x * r.y;
            xz += r.x * r.z;
            yy += r.y * r.y;
            yz += r.y * r.z;
            zz += r.z * r.z;
        }

        let det_x = yy * zz - yz * yz;
        let det_y = xx * zz - xz * xz;
        let det_z = xx * yy - xy * xy;

        let det_max = det_x.max(det_y).max(det_z);

        if det_max <= 0.0 {
            return None; // The points don't span a plane
        }

        // Pick path with best conditioning:
        let dir = if det_max == det_x {
            let x = det_x;
            let y = xz * yz - xy * zz;
            let z = xy * yz - xz * yy;
            vector![x, y, z]
        } else if det_max == det_y {
            let x = xz * yz - xy * zz;
            let y = det_y;
            let z = xy * xz - yz * xx;
            vector![x, y, z]
        } else {
            let x = xy * yz - xz * yy;
            let y = xy * xz - yz * xx;
            let z = det_z;
            vector![x, y, z]
        };

        let normal_vector = normalize(dir);

        Some(NormalVector::from_na_vec(&normal_vector))
    }
}

fn normalize(v: Vector3<f64>) -> Vector3<f64> {
    let norm: f64 = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    vector![v.x / norm, v.y / norm, v.z / norm]
}

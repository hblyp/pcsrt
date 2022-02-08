use log::warn;
use nalgebra::{vector, Vector3};

use super::{NormalVector, Point};

/* The MIT License (MIT)

Copyright (c) 2017-2020 John Lindsay

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */
#[allow(unknown_lints)]
#[allow(clippy::all)]
pub fn plane_from_points(points: &Vec<Point>) -> NormalVector {
    let points: &Vec<Vector3<f64>> = &points.iter().map(|point| point.to_na_vec()).collect();
    let n = points.len();

    if n < 3 {
        return NormalVector::new();
    }

    let mut sum = vector![0.0, 0.0, 0.0];
    for p in points {
        sum += *p;
    }
    let centroid = sum * (1.0 / (n as f64));

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

    let det_max_abs = det_max.abs();

    let dir = if (det_max_abs - det_x.abs()).abs() < std::f64::EPSILON {
        let a = (xz * yz - xy * zz) / det_x;
        let b = (xy * yz - xz * yy) / det_x;
        if a.is_nan() || b.is_nan() {
            warn!("No plane from points:\n {:?}", points);
        }
        vector![1.0, a, b]
    } else if (det_max_abs - det_y.abs()).abs() < std::f64::EPSILON {
        let a = (yz * xz - xy * zz) / det_y;
        let b = (xy * xz - yz * xx) / det_y;
        vector![a, 1.0, b]
    } else {
        let a = (yz * xy - xz * yy) / det_z;
        let b = (xz * xy - yz * xx) / det_z;
        vector![a, b, 1.0]
    };

    let normal_vector = normalize(dir);

    NormalVector::from_na_vec(&normal_vector)
}

fn normalize(v: Vector3<f64>) -> Vector3<f64> {
    let norm: f64 = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    vector![v.x / norm, v.y / norm, v.z / norm]
}

// Copyright 2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {Rect, Point3D};

#[derive(Clone, Copy, RustcDecodable, RustcEncodable, PartialEq)]
#[cfg_attr(feature = "plugins", derive(HeapSizeOf))]
pub struct Ray3D {
    pub origin: Point3D<f32>,
    pub end: Point3D<f32>,
}

impl Ray3D {
    pub fn new(origin: Point3D<f32>, end: Point3D<f32>) -> Ray3D {
        Ray3D {
            origin: origin,
            end: end,
        }
    }

    /// A naive port of ["An Efficient and Robust Ray–Box Intersection
    /// Algorithm"][1]. Assumes `rect` is in the z=0 plane.
    ///
    /// [1]: https://www.cs.utah.edu/~awilliam/box/box.pdf
    #[inline]
    pub fn intersects_rect(&self, rect: &Rect<f32>) -> bool {
        let mut dir = self.end - self.origin;
        let len = ((dir.x*dir.x) + (dir.y*dir.y) + (dir.z*dir.z)).sqrt();
        dir.x = dir.x / len;
        dir.y = dir.y / len;
        dir.z = dir.z / len;
        let inv_direction = Point3D::new(1.0/dir.x, 1.0/dir.y, 1.0/dir.z);

        let sign = [
            if inv_direction.x < 0.0 {
                1
            } else {
                0
            },
            if inv_direction.y < 0.0 {
                1
            } else {
                0
            },
            if inv_direction.z < 0.0 {
                1
            } else {
                0
            },
        ];

        let parameters = [
            Point3D::new(rect.origin.x, rect.origin.y, 0.0),
            Point3D::new(rect.origin.x + rect.size.width,
                         rect.origin.y + rect.size.height,
                         0.0),
        ];

        let mut tmin = (parameters[sign[0]].x - self.origin.x) * inv_direction.x;
        let mut tmax = (parameters[1-sign[0]].x - self.origin.x) * inv_direction.x;
        let tymin = (parameters[sign[1]].y - self.origin.y) * inv_direction.y;
        let tymax = (parameters[1-sign[1]].y - self.origin.y) * inv_direction.y;
        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }
        let tzmin = (parameters[sign[2]].z - self.origin.z) * inv_direction.z;
        let tzmax = (parameters[1-sign[2]].z - self.origin.z) * inv_direction.z;
        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        // Don't care about where on the ray it hits...
        true

        /*
        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        let t0 = 0.0;
        let t1 = len;

        (tmin < t1) && (tmax > t0)
        */
    }
}

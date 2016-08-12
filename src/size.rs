// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use length::{Length, UnknownUnit};
use scale_factor::ScaleFactor;
use num::Zero;

use num_traits::NumCast;
use std::fmt;
use std::ops::{Mul, Div};
use std::marker::PhantomData;

define_matrix! {
    #[derive(RustcDecodable, RustcEncodable)]
    pub struct TypedSize2D<T, U> {
        pub width: T,
        pub height: T,
    }
}

pub type Size2D<T> = TypedSize2D<T, UnknownUnit>;

impl<T: fmt::Debug, U> fmt::Debug for TypedSize2D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}×{:?}", self.width, self.height)
    }
}

impl<T: fmt::Display, U> fmt::Display for TypedSize2D<T, U> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({}x{})", self.width, self.height)
    }
}

impl<T, U> TypedSize2D<T, U> {
    pub fn new(width: T, height: T) -> TypedSize2D<T, U> {
        TypedSize2D {
            width: width,
            height: height,
            _unit: PhantomData,
        }
    }
}

impl<T: Clone, U> TypedSize2D<T, U> {
    pub fn from_lengths(width: Length<T, U>, height: Length<T, U>) -> TypedSize2D<T, U> {
        TypedSize2D::new(width.get(), height.get())
    }
}

impl<T: Copy + Clone + Mul<T, Output=U>, U> TypedSize2D<T, U> {
    pub fn area(&self) -> U { self.width * self.height }
}

impl<T: Zero, U> TypedSize2D<T, U> {
    pub fn zero() -> TypedSize2D<T, U> {
        TypedSize2D::new(
            Zero::zero(),
            Zero::zero(),
        )
    }
}

impl<T: Zero, U> Zero for TypedSize2D<T, U> {
    fn zero() -> TypedSize2D<T, U> {
        TypedSize2D::new(
            Zero::zero(),
            Zero::zero(),
        )
    }
}

impl<T: Copy + Mul<T, Output=T>, U> Mul<T> for TypedSize2D<T, U> {
    type Output = TypedSize2D<T, U>;
    #[inline]
    fn mul(self, scale: T) -> TypedSize2D<T, U> {
        TypedSize2D::new(self.width * scale, self.height * scale)
    }
}

impl<T: Copy + Div<T, Output=T>, U> Div<T> for TypedSize2D<T, U> {
    type Output = TypedSize2D<T, U>;
    #[inline]
    fn div(self, scale: T) -> TypedSize2D<T, U> {
        TypedSize2D::new(self.width / scale, self.height / scale)
    }
}

impl<T: Copy + Mul<T, Output=T>, U1, U2> Mul<ScaleFactor<T, U1, U2>> for TypedSize2D<T, U1> {
    type Output = TypedSize2D<T, U2>;
    #[inline]
    fn mul(self, scale: ScaleFactor<T, U1, U2>) -> TypedSize2D<T, U2> {
        TypedSize2D::new(self.width * scale.get(), self.height * scale.get())
    }
}

impl<T: Copy + Div<T, Output=T>, U1, U2> Div<ScaleFactor<T, U1, U2>> for TypedSize2D<T, U2> {
    type Output = TypedSize2D<T, U1>;
    #[inline]
    fn div(self, scale: ScaleFactor<T, U1, U2>) -> TypedSize2D<T, U1> {
        TypedSize2D::new(self.width / scale.get(), self.height / scale.get())
    }
}

// Convenient aliases for TypedSize2D with typed units

impl<Unit, T: Clone> TypedSize2D<T, Unit> {
    /// Drop the units, preserving only the numeric value.
    pub fn to_untyped(&self) -> Size2D<T> {
        TypedSize2D::new(self.width.clone(), self.height.clone())
    }

    /// Tag a unitless value with units.
    pub fn from_untyped(p: &Size2D<T>) -> TypedSize2D<T, Unit> {
        TypedSize2D::new(p.width.clone(), p.height.clone())
    }
}

impl<Unit, T0: NumCast + Clone> TypedSize2D<T0, Unit> {
    /// Cast from one numeric representation to another, preserving the units.
    pub fn cast<T1: NumCast + Clone>(&self) -> Option<TypedSize2D<T1, Unit>> {
        match (NumCast::from(self.width.clone()), NumCast::from(self.height.clone())) {
            (Some(w), Some(h)) => Some(TypedSize2D::new(w, h)),
            _ => None
        }
    }
}

// Convenience functions for common casts
impl<Unit, T: NumCast + Clone> TypedSize2D<T, Unit> {
    pub fn as_f32(&self) -> TypedSize2D<f32, Unit> {
        self.cast().unwrap()
    }

    pub fn as_uint(&self) -> TypedSize2D<usize, Unit> {
        self.cast().unwrap()
    }
}

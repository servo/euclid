use {Rect, Box2D, Box3D, size2, point2, point3};
use approxord::{min, max};
use num::Zero;
use core::ops::Deref;
use core::ops::{Add, Sub};
use core::cmp::{PartialEq};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NonEmpty<T>(pub(crate) T);

impl<T> Deref for NonEmpty<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> NonEmpty<T> {
    #[inline]
    pub fn get(&self) -> &T {
        &self.0
    }
}

impl<T, U> NonEmpty<Rect<T, U>>
where
    T: Copy + Clone + Zero + PartialOrd + PartialEq + Add<T, Output = T> + Sub<T, Output = T>,
{
    pub fn union(&self, other: &NonEmpty<Rect<T, U>>) -> NonEmpty<Rect<T, U>> {
        let origin = point2(
            min(self.min_x(), other.min_x()),
            min(self.min_y(), other.min_y()),
        );

        let lower_right_x = max(self.max_x(), other.max_x());
        let lower_right_y = max(self.max_y(), other.max_y());

        NonEmpty(Rect {
            origin,
            size: size2(
                lower_right_x - origin.x,
                lower_right_y - origin.y,
            ),
        })
    }

    #[inline]
    pub fn contains_rect(&self, rect: &Self) -> bool {
        self.min_x() <= rect.min_x()
            && rect.max_x() <= self.max_x()
            && self.min_y() <= rect.min_y()
            && rect.max_y() <= self.max_y()
    }
}

impl<T, U> NonEmpty<Box2D<T, U>>
where
    T: Copy + PartialOrd,
{
    pub fn union(&self, other: &NonEmpty<Box2D<T, U>>) -> NonEmpty<Box2D<T, U>> {
        NonEmpty(Box2D {
            min: point2(
                min(self.min.x, other.min.x),
                min(self.min.y, other.min.y),
            ),
            max: point2(
                max(self.max.x, other.max.x),
                max(self.max.y, other.max.y),
            ),
        })
    }

    /// Returns true if this box contains the interior of the other box. Always
    #[inline]
    pub fn contains_box(&self, other: &Self) -> bool {
        self.min.x <= other.min.x
            && other.max.x <= self.max.x
            && self.min.y <= other.min.y
            && other.max.y <= self.max.y
    }
}

impl<T, U> NonEmpty<Box3D<T, U>>
where
    T: Copy + PartialOrd,
{
    pub fn union(&self, other: &NonEmpty<Box3D<T, U>>) -> NonEmpty<Box3D<T, U>> {
        NonEmpty(Box3D {
            min: point3(
                max(self.min.x, other.min.x),
                max(self.min.y, other.min.y),
                max(self.min.z, other.min.z),
            ),
            max: point3(
                min(self.max.x, other.max.x),
                min(self.max.y, other.max.y),
                min(self.max.z, other.max.z),
            ),
        })
    }

    /// Returns true if this box contains the interior of the other box. Always
    #[inline]
    pub fn contains_box(&self, other: &Self) -> bool {
        self.min.x <= other.min.x
            && other.max.x <= self.max.x
            && self.min.y <= other.min.y
            && other.max.y <= self.max.y
            && self.min.z <= other.min.z
            && other.max.z <= self.max.z
    }
}

/* This file exists just to make it easier to import things inside of
 ./geom/ without specifying the file they came out of imports.
 
Note that you still must define each of the files as a module in
geom.rc. This is not ideal and may be changed in the future. */

pub use matrix::Matrix4;
pub use matrix2d::Matrix2D;
pub use point::Point2D;
pub use rect::Rect;
pub use size::Size2D;

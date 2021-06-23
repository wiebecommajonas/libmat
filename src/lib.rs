//! This library provides tools for linear algebra.
//!
//! # Example
//!
//! ```rust
//! use libmat::mat::{Matrix, Vector};
//! use libmat::{matrix, vector}; // macros
//!
//! let vec_a = vector![1,0,0];
//! let vec_b = vector![0,1,0];
//! let vec_c = vector![0,0,1];
//!
//! // Are the vectors perpendicular to each other?
//! assert_eq!(&vec_a * &vec_b, 0);
//! assert_eq!(&vec_a * &vec_c, 0);
//! assert_eq!(&vec_c * &vec_b, 0);
//!
//! let mat_a = Matrix::<i32>::one(3);
//! let mat_b = matrix!{
//!     1, 2, 3;
//!     3, 2, 1;
//!     2, 1, 3;
//! };
//!
//! // Are the matrices invertible?
//! assert_ne!(mat_a.det(), 0_f64);
//! assert_ne!(mat_b.det(), 0_f64);
//! ```

mod macros;
pub mod mat;

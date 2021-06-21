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
//! ```

pub mod macros;
pub mod mat;

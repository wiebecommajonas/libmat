# libmat

This library provides tools for linear algebra. For a full documentation, please visit [docs.rs].

## Usage

To use this library, add this to your `Cargo.toml`:

```toml
[dependencies]
libmat = "0.1.0"
```

## Example

```rust
use libmat::mat::{Matrix, Vector};
use libmat::{matrix, vector}; // macros

let vec_a = vector![1,0,0];
let vec_b = vector![0,1,0];
let vec_c = vector![0,0,1];

// Are the vectors perpendicular to each other?
assert_eq!(&vec_a * &vec_b, 0);
assert_eq!(&vec_a * &vec_c, 0);
assert_eq!(&vec_c * &vec_b, 0);

let mat_a = Matrix::<u32>::one(3);
let mat_b = matrix!{
    1, 2, 3;
    3, 2, 1;
    2, 1, 3;
};

// Are the matrices invertible?
assert_ne!(mat_a.det(), 0);
assert_ne!(mat_b.det(), 0);
```

## Changes

A full changelog is available in [`CHANGELOG.md`]

## License

Licensed under either of

* [Apache License, Version 2.0][apache]
* [MIT License][mit]

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[docs.rs]: https://docs.rs/libmat/0.1.0
[apache]: http://www.apache.org/licenses/LICENSE-2.0
[mit]: http://opensource.org/licenses/MIT
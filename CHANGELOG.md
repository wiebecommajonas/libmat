# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## [Unreleased]

### Added

- Matrix
    - statically sized matrix `SMatrix`
    - invert matrices with `inv()`
- Vector
    - statically sized vectors `SColVector` and `SRowVector`
- Macros
    - `smatrix!` for creating `SMatrix`
    - matrix macros accept [Wolfram Alpha](https://www.wolframalpha.com/input/?i=matrix+multiplication) syntax

## [0.1.0] - 2021-06-21

### Added

- Vector
    - vector and matrix multiplication
    - scaling
- Macros
    - `vector!`
    - `matrix!`

## 0.0.1

- Matrix
    - Basic operations

[Unreleased]: https://github.com/wiebecommajonas/libmat/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/wiebecommajonas/libmat/releases/tag/v0.1.0

# matrices

`matrices` is a simple library for doing calculations with matrices and vectors. The full documentation can be found here: [rustdocs]

## Features
### Basic operations
This library implements basic arithmetic operations on matrices. The operations need to be done on references to matrices.

**List of currently implemented oprations**

- [X] Addition
- [X] Subtraction
- [X] Multiplication
- [X] Division
- [X] Negation
- [X] Index
- [X] IndexMut

### Matrix methods
`new(rows, cols, init)`: Creates new matrix.
`from_vec(rows, cols, vec)`: Creates new matrix.
`one(dim)`: Creates a unit matrix.
`zero(rows, cols)`: Creates a zero matrix.
`diag(dim, init)`: Creates a diagonal matrix.
`det()`: Calculates the determinant of a matrix.
`transpose()`: Transposes a matrix.
`is_square()`: Checks whether the matrix is a square or not.

[rustdocs]: 
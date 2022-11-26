# vecgrid

[![dependency status](https://deps.rs/repo/github/good-praxis/vecgrid/status.svg)](https://deps.rs/repo/github/good-praxis/vecgrid)
[![Build Status](https://github.com/good-praxis/vecgrid/workflows/CI/badge.svg)](https://github.com/good-praxis/vecgrid/actions?workflow=CI)

Vecgrid provides a dynamically sized two-dimensional vector. It is more efficient
and is easier to use than a vector of vectors, i.e. `Vec<Vec<T>>`.

This is beneficial when using a grid-like structure, which is common in
image processing, game development, and other situations. Vecgrid cannot be used
when rows or columns might have different lengths⁠—all rows and columns must
be the same length.

Vecgrid is a fork of the statically-sized two-dimensional array library [`Array2D`].

## Roadmap

This project attemps to extend the upstream project in an opinionated fashion, by adding mutable iterators and dynamic resizing of the inner collection. Here's how that is going:

- [x] `row_iter_mut`
- [x] `column_iter_mut`
- [x] `rows_iter_mut`
- [x] `columns_iter_mut`
- [x] `elements_row_major_iter_mut`
- [x] `elements_column_major_iter_mut`
- [x] `insert_row`
- [ ] `insert_column`
- [x] `insert_rows`
- [ ] `insert_columns`
- [x] `remove_row`
- [ ] `remove_column`
- [x] `remove_rows`
- [ ] `remove_columns`
- [x] `append_rows`
- [ ] `append_columns`
- [ ] `extend_rows`
- [ ] `extend_columns`

Upstream code might be refactored along the way to make use of optimizations or to align approaches across the crate. Code deprecated upstream from before the inital release of this crate is dropped, future deprecated upstream code may or may not be deprecated in this crate in kind. A release of a major version of this crate indicates maturity surpassing active tracking of the upstream repository, but until then changes will be synced as they happen.

## How to use [`Vecgrid`]

### Creating a [`Vecgrid`]

A [`Vecgrid`] can be created in many different ways. These include:

- Providing the rows or the columns, which must all be the same size (see
  [`from_rows`] and [`from_columns`]).
- Providing a "flat" vector of elements in either [row major or column
  major order] along with the dimensions, which must match the number of
  elements in the vector (see [`from_row_major`] and
  [`from_column_major`]).
- Providing a value to repeatedly put in every location (see
  [`filled_with`]).
- Providing a generator function that is repeatedly called to produce
  values to fill the vecgrid (see [`filled_by_row_major`] and
  [`filled_by_column_major`]).
- Providing an iterator that is used to produce values to fill the vecgrid
  (see [`from_iter_row_major`] and [`from_iter_column_major`]).

### Extending a [`Vecgrid`]

Since [`Vecgrid`]s are dynamically sized, it is possible to extend them:

- Providing singular rows of matching length alongside row indices to [`insert_row`],
  or providing a mutable slice of rows to [`insert_rows`].
- Append the grid, either with matching length rows via [`append_rows`]... or future additions!
- Remove singular or consecutive rows via [`remove_row`] and [`remove_rows`] respectively.

### Accessing data from a [`Vecgrid`]

[`Vecgrid`] supports several forms of indexing:

- Using the indexing syntax (square brackets) with a tuple of [`(usize,
  usize)`], which panics on out-of-bounds accesses.
- Using the [`get`], [`get_mut`], and [`set`] methods, which return an
  [`Option`] or a [`Result`] on out-of-bounds accesses.
- Using the row major or column major version of these methods,
  i.e. [`get_row_major`], [`get_mut_row_major`], [`set_row_major`],
  [`get_column_major`], [`get_mut_column_major`],
  [`set_column_major`]. These perform the same tasks as the non row/column
  major methods, but take one index instead of two.

[`Vecgrid`] also supports several forms of iteration. You can iterate
through:

- All of the elements, in either [row major or column major order] (see
  [`elements_row_major_iter`] and [`elements_column_major_iter`]).
- All of the elements as mutable references, in [row major or column major order] (see
  [`elements_row_major_iter_mut`] and [`elements_column_major_iter_mut`]).
- Individual rows or columns (see [`row_iter`] and [`column_iter`]).
- Individual rows and columns of mutable entries (see [`row_iter_mut`] and [`column_iter_mut`]).
- All rows or all columns (see [`rows_iter`] and [`columns_iter`]).
- All rows or all columns of mutable entries (see [`rows_iter_mut`] and [`columns_iter_mut`]).

### Extracting all data from a [`Vecgrid`]

A [`Vecgrid`] can be converted back into a [`Vec`] through several
methods. You can extract the data as:

- A [`Vec`] of rows or columns (see [`as_rows`] and [`as_columns`]).
- A "flat" [`Vec`] of elements in either [row major or column major order]
  (see [`as_row_major`] and [`as_column_major`]).

## Examples

```rust
use vecgrid::{Vecgrid, Error};

pub fn main() -> Result<(), Error> {
    // Create a vecgrid filled with the same element.
    let prefilled = Vecgrid::filled_with(42, 2, 3);
    assert_eq!(prefilled.num_rows(), 2);
    assert_eq!(prefilled.num_columns(), 3);
    assert_eq!(prefilled[(0, 0)], 42);

    // Create a vecgrid from the given rows. You can also use columns
    // with the `columns` function
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let from_rows = Vecgrid::from_rows(&rows)?;
    assert_eq!(from_rows.num_rows(), 2);
    assert_eq!(from_rows.num_columns(), 3);
    assert_eq!(from_rows[(1, 1)], 5);

    // Create  vecgrid from a flat Vec of elements in row major or
    // column major order.
    let column_major = vec![1, 4, 2, 5, 3, 6];
    let from_column_major =
        Vecgrid::from_column_major(&column_major, 2, 3)?;
    assert_eq!(from_column_major.num_rows(), 2);
    assert_eq!(from_column_major.num_columns(), 3);
    assert_eq!(from_column_major[(1, 1)], 5);

    // Implements `Eq` if the element type does.
    assert_eq!(from_rows, from_column_major);

    // Index into a vecgrid using a tuple of usize to access or alter
    // the vecgrid.
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(&rows)?;
    vecgrid[(1, 1)] = 100;

    // Convert the vecgrid back into a nested Vec using `as_rows` or
    // `as_columns`.
    let vecgrid_rows = vecgrid.as_rows();
    assert_eq!(vecgrid_rows, vec![vec![1, 2, 3], vec![4, 100, 6]]);

    // Convert the vecgrid back into a flat Vec using `as_row_major` or
    // `as_column_major`.
    let vecgrid_column_major = vecgrid.as_column_major();
    assert_eq!(vecgrid_column_major, vec![1, 4, 2, 100, 3, 6]);

    // Iterate over a single row or column
    println!("First column:");
    for element in vecgrid.column_iter(0)? {
        println!("{}", element);
    }

    // Iterate over all rows or columns.
    println!("All elements:");
    for row_iter in vecgrid.rows_iter() {
        for element in row_iter {
            print!("{} ", element);
        }
        println!();
    }

    Ok(())
}

```

## Acknowledgement

This library is made possible thanks to the excellent groundwork laid down in [`Array2D`] by author [HarrisonMc555](https://github.com/HarrisonMc555), as well as contributor to the upstream project [tylerjw](https://github.com/tylerjw). [`Array2D`] has been published under the MIT license.

[`vecgrid`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html
[`from_rows`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.from_rows
[`from_columns`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.from_columns
[`from_row_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.from_row_major
[`from_column_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.from_column_major
[`filled_with`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.filled_with
[`filled_by_row_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.filled_by_row_major
[`filled_by_column_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.filled_by_column_major
[`from_iter_row_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.from_iter_row_major
[`from_iter_column_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.from_iter_column_major
[`get`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.get
[`get_mut`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.get_mut
[`set`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.set
[`get_row_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.get_row_major
[`get_mut_row_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.get_mut_row_major
[`set_row_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.set_row_major
[`get_column_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.get_column_major
[`get_mut_column_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.get_mut_column_major
[`set_column_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.set_column_major
[`elements_row_major_iter`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.elements_row_major_iter
[`elements_column_major_iter`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.elements_column_major_iter
[`elements_row_major_iter_mut`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.elements_row_major_iter_mut
[`elements_column_major_iter_mut`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.elements_column_major_iter_mut
[`row_iter`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.row_iter
[`column_iter`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.column_iter
[`row_iter_mut`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.row_iter_mut
[`column_iter_mut`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.column_iter_mut
[`rows_iter`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.rows_iter
[`columns_iter`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.columns_iter
[`rows_iter_mut`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.rows_iter_mut
[`columns_iter_mut`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.columns_iter_mut
[`as_rows`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.as_rows
[`as_columns`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.as_columns
[`as_row_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.as_row_major
[`as_column_major`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.as_column_major
[`insert_row`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.insert_row
[`insert_rows`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.insert_rows
[`append_rows`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.append_rows
[`remove_row`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.remove_row
[`remove_rows`]: https://docs.rs/vecgrid/latest/vecgrid/struct.Vecgrid.html#method.remove_rows
[`vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[`option`]: https://doc.rust-lang.org/std/option/
[`result`]: https://doc.rust-lang.org/std/result/
[`(usize, usize)`]: https://doc.rust-lang.org/std/primitive.usize.html
[row major or column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
[`array2d`]: https://github.com/HarrisonMc555/array2d

License: MIT

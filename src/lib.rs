//! Vecgrid provides a dynamically sized two-dimensional vec. It is more efficient
//! and is easier to use than a vector of vectors, i.e. `Vec<Vec<T>>`.
//!
//! This is beneficial when using a grid-like structure, which is common in
//! image processing, game boards, and other situations. Vecgrid cannot be used
//! when rows or columns might have different lengths⁠—all rows and columns must
//! be the same length.
//!
//! # How to use [`Vecgrid`]
//!
//! ## Creating an [`Vecgrid`]
//!
//! An [`Vecgrid`] can be created in many different ways. These include:
//!   - Providing the rows or the columns, which must all be the same size (see
//!     [`from_rows`] and [`from_columns`]).
//!   - Providing a "flat" vector of elements in either [row major or column
//!     major order] along with the dimensions, which must match the number of
//!     elements in the vector (see [`from_row_major`] and
//!     [`from_column_major`]).
//!   - Providing a value to repeatedly put in every location (see
//!     [`filled_with`]).
//!   - Providing a generator function that is repeatedly called to produce
//!     values to fill the vecgrid (see [`filled_by_row_major`] and
//!     [`filled_by_column_major`]).
//!   - Providing an iterator that is used to produce values to fill the vecgrid
//!     (see [`from_iter_row_major`] and [`from_iter_column_major`]).
//!
//! ## Extending a [`Vecgrid`]
//!
//! Since [`Vecgrid`]s are dynamically sized, it is possible to extend them:
//!
//!   - Providing singular rows of matching length alongside row indices to [`insert_row`],
//!     or providing a mutable slice of rows to [`insert_rows`].
//!   - Append the grid, either with matching length rows via [`append_rows`]... or future additions!
//!   - Remove singular or consecutive rows via [`remove_row`] and [`remove_rows`] respectively.
//!
//! ## Accessing data from an [`Vecgrid`]
//!
//! [`Vecgrid`] supports several forms of indexing:
//!   - Using the indexing syntax (square brackets) with a tuple of [`(usize,
//!     usize)`], which panics on out-of-bounds accesses.
//!   - Using the [`get`], [`get_mut`], and [`set`] methods, which return an
//!     [`Option`] or a [`Result`] on out-of-bounds accesses.
//!   - Using the row major or column major version of these methods,
//!     i.e. [`get_row_major`], [`get_mut_row_major`], [`set_row_major`],
//!     [`get_column_major`], [`get_mut_column_major`],
//!     [`set_column_major`]. These perform the same tasks as the non row/column
//!     major methods, but take one index instead of two.
//!
//! [`Vecgrid`] also supports several forms of iteration. You can iterate
//! through:
//!   - All of the elements, in either [row major or column major order] (see
//!     [`elements_row_major_iter`] and [`elements_column_major_iter`]).
//!   - All of the elements as mutable references, in [row major or column major order] (see
//!     [`elements_row_major_iter_mut`] and [`elements_column_major_iter_mut`]).
//!   - Individual rows or columns (see [`row_iter`] and [`column_iter`]).
//!   - Individual rows and columns of mutable entries (see [`row_iter_mut`] and [`column_iter_mut`]).
//!   - All rows or all columns (see [`rows_iter`] and [`columns_iter`]).
//!   - All rows or all columns of mutable entries (see [`rows_iter_mut`] and [`columns_iter_mut`]).
//!
//!
//! ## Extracting all data from an [`Vecgrid`]
//!
//! An [`Vecgrid`] can be converted back into a [`Vec`] through several
//! methods. You can extract the data as:
//!   - A [`Vec`] of rows or columns (see [`as_rows`] and [`as_columns`]).
//!   - A "flat" [`Vec`] of elements in either [row major or column major order]
//!     (see [`as_row_major`] and [`as_column_major`]).
//!
//! # Examples
//!
//! ```rust
//! use vecgrid::{Vecgrid, Error};
//!
//! pub fn main() -> Result<(), Error> {
//!     // Create a vecgrid filled with the same element.
//!     let prefilled = Vecgrid::filled_with(42, 2, 3);
//!     assert_eq!(prefilled.num_rows(), 2);
//!     assert_eq!(prefilled.num_columns(), 3);
//!     assert_eq!(prefilled[(0, 0)], 42);
//!
//!     // Create a vecgrid from the given rows. You can also use columns
//!     // with the `columns` function
//!     let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
//!     let from_rows = Vecgrid::from_rows(rows)?;
//!     assert_eq!(from_rows.num_rows(), 2);
//!     assert_eq!(from_rows.num_columns(), 3);
//!     assert_eq!(from_rows[(1, 1)], 5);
//!
//!     // Create a vecgrid from a flat Vec of elements in row major or
//!     // column major order.
//!     let column_major = vec![1, 4, 2, 5, 3, 6];
//!     let from_column_major =
//!         Vecgrid::from_column_major(column_major, 2, 3)?;
//!     assert_eq!(from_column_major.num_rows(), 2);
//!     assert_eq!(from_column_major.num_columns(), 3);
//!     assert_eq!(from_column_major[(1, 1)], 5);
//!
//!     // Implements `Eq` if the element type does.
//!     assert_eq!(from_rows, from_column_major);
//!
//!     // Index into a vecgrid using a tuple of usize to access or alter
//!     // the vecgrid.
//!     let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
//!     let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
//!     vecgrid[(1, 1)] = 100;
//!
//!     // Convert the vecgrid back into a nested Vec using `as_rows` or
//!     // `as_columns`.
//!     let vecgrid_rows = vecgrid.as_rows();
//!     assert_eq!(vecgrid_rows, vec![vec![1, 2, 3], vec![4, 100, 6]]);
//!
//!     // Convert the vecgrid back into a flat Vec using `as_row_major` or
//!     // `as_column_major`.
//!     let vecgrid_column_major = vecgrid.as_column_major();
//!     assert_eq!(vecgrid_column_major, vec![1, 4, 2, 100, 3, 6]);
//!
//!     // Iterate over a single row or column
//!     println!("First column:");
//!     for element in vecgrid.column_iter(0)? {
//!         println!("{}", element);
//!     }
//!
//!     // Iterate over all rows or columns.
//!     println!("All elements:");
//!     for row_iter in vecgrid.rows_iter() {
//!         for element in row_iter {
//!             print!("{} ", element);
//!         }
//!         println!();
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! [`Vecgrid`]: struct.Vecgrid.html
//! [`from_rows`]: struct.Vecgrid.html#method.from_rows
//! [`from_columns`]: struct.Vecgrid.html#method.from_columns
//! [`from_row_major`]: struct.Vecgrid.html#method.from_row_major
//! [`from_column_major`]: struct.Vecgrid.html#method.from_column_major
//! [`filled_with`]: struct.Vecgrid.html#method.filled_with
//! [`filled_by_row_major`]: struct.Vecgrid.html#method.filled_by_row_major
//! [`filled_by_column_major`]: struct.Vecgrid.html#method.filled_by_column_major
//! [`from_iter_row_major`]: struct.Vecgrid.html#method.from_iter_row_major
//! [`from_iter_column_major`]: struct.Vecgrid.html#method.from_iter_column_major
//! [`get`]: struct.Vecgrid.html#method.get
//! [`get_mut`]: struct.Vecgrid.html#method.get_mut
//! [`set`]: struct.Vecgrid.html#method.set
//! [`get_row_major`]: struct.Vecgrid.html#method.get_row_major
//! [`get_mut_row_major`]: struct.Vecgrid.html#method.get_mut_row_major
//! [`set_row_major`]: struct.Vecgrid.html#method.set_row_major
//! [`get_column_major`]: struct.Vecgrid.html#method.get_column_major
//! [`get_mut_column_major`]: struct.Vecgrid.html#method.get_mut_column_major
//! [`set_column_major`]: struct.Vecgrid.html#method.set_column_major
//! [`elements_row_major_iter`]: struct.Vecgrid.html#method.elements_row_major_iter
//! [`elements_column_major_iter`]: struct.Vecgrid.html#method.elements_column_major_iter
//! [`elements_row_major_iter_mut`]: struct.Vecgrid.html#method.elements_row_major_iter_mut
//! [`elements_column_major_iter_mut`]: struct.Vecgrid.html#method.elements_column_major_iter_mut
//! [`row_iter`]: struct.Vecgrid.html#method.row_iter
//! [`column_iter`]: struct.Vecgrid.html#method.column_iter
//! [`row_iter_mut`]: struct.Vecgrid.html#method.row_iter_mut
//! [`column_iter_mut`]: struct.Vecgrid.html#method.column_iter_mut
//! [`rows_iter`]: struct.Vecgrid.html#method.rows_iter
//! [`columns_iter`]: struct.Vecgrid.html#method.columns_iter
//! [`rows_iter_mut`]: struct.Vecgrid.html#method.rows_iter_mut
//! [`columns_iter_mut`]: struct.Vecgrid.html#method.columns_iter_mut
//! [`as_rows`]: struct.Vecgrid.html#method.as_rows
//! [`as_columns`]: struct.Vecgrid.html#method.as_columns
//! [`as_row_major`]: struct.Vecgrid.html#method.as_row_major
//! [`as_column_major`]: struct.Vecgrid.html#method.as_column_major
//! [`insert_row`]: struct.Vecgrid.html#method.insert_row
//! [`insert_rows`]: struct.Vecgrid.html#method.insert_rows
//! [`append_rows`]: struct.Vecgrid.html#method.append_rows
//! [`remove_row`]: struct.Vecgrid.html#method.remove_row
//! [`remove_rows`]: struct.Vecgrid.html#method.remove_rows
//! [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
//! [`Option`]: https://doc.rust-lang.org/std/option/
//! [`Result`]: https://doc.rust-lang.org/std/result/
//! [`(usize, usize)`]: https://doc.rust-lang.org/std/primitive.usize.html
//! [row major or column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order

#![deny(missing_docs)]

use std::ops::{Index, IndexMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A dynamically sized two-dimensional vec.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vecgrid<T> {
    vecgrid: Vec<T>,
    num_rows: usize,
    num_columns: usize,
}

/// An error that can arise during the use of an [`Vecgrid`].
///
/// [`Vecgrid`]: struct.Vecgrid.html
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// The given indices were out of bounds.
    IndicesOutOfBounds(usize, usize),
    /// The given index in row or column major order was out of bounds.
    IndexOutOfBounds(usize),
    /// The dimensions given did not match the elements provided
    DimensionMismatch,
    /// There were not enough elements to fill the vecgrid.
    NotEnoughElements,
}

impl<T> Vecgrid<T> {
    /// Creates a new [`Vecgrid`] from a [`Vec`] of rows, each of which is a
    /// [`Vec`] of elements.
    ///
    /// Returns an error if the rows are not all the same size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// assert_eq!(vecgrid[(1, 2)], 6);
    /// assert_eq!(vecgrid.as_rows(), rows);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn from_rows(rows: Vec<Vec<T>>) -> Result<Self, Error> {
        let row_len = rows.get(0).map(Vec::len).unwrap_or(0);
        let mut vecgrid = Vecgrid {
            vecgrid: Vec::new(),
            num_rows: 0,
            num_columns: row_len,
        };
        vecgrid.append_rows(rows)?;
        Ok(vecgrid)
    }

    /// Creates a new [`Vecgrid`] from a [`Vec`] of columns, each of which
    /// contains a [`Vec`] of elements.
    ///
    /// Returns an error if the columns are not all the same size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let columns = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    /// let vecgrid = Vecgrid::from_columns(columns.clone())?;
    /// assert_eq!(vecgrid[(1, 2)], 6);
    /// assert_eq!(vecgrid.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn from_columns(columns: Vec<Vec<T>>) -> Result<Self, Error>
    where
        T: Clone, //TODO: Remove type guard
    {
        let column_len = columns.get(0).map(Vec::len).unwrap_or(0);
        if !columns.iter().all(|column| column.len() == column_len) {
            return Err(Error::DimensionMismatch);
        }
        let num_rows = column_len;
        let num_columns = columns.len();
        let vecgrid = indices_row_major(num_rows, num_columns)
            .map(|(row, column)| columns[column][row].clone())
            .collect();
        Ok(Vecgrid {
            vecgrid,
            num_rows,
            num_columns,
        })
    }

    /// Creates a new [`Vecgrid`] from the given flat [`Vec`] in [row major
    /// order].
    ///
    /// Returns an error if the number of elements in `elements` is not the
    /// product of `num_rows` and `num_columns`, i.e. the dimensions do not
    /// match.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let row_major = vec![1, 2, 3, 4, 5, 6];
    /// let vecgrid = Vecgrid::from_row_major(row_major, 2, 3)?;
    /// assert_eq!(vecgrid[(1, 2)], 6);
    /// assert_eq!(vecgrid.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    /// [row major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_row_major(
        elements: Vec<T>,
        num_rows: usize,
        num_columns: usize,
    ) -> Result<Self, Error> {
        let total_len = num_rows * num_columns;
        if total_len != elements.len() {
            return Err(Error::DimensionMismatch);
        }
        Ok(Vecgrid {
            vecgrid: elements,
            num_rows,
            num_columns,
        })
    }

    /// Creates a new [`Vecgrid`] from the given flat [`Vec`] in [column major
    /// order].
    ///
    /// Return an error if the number of elements in `elements` is not the
    /// product of `num_rows` and `num_columns`, i.e. the dimensions do not
    /// match.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let column_major = vec![1, 4, 2, 5, 3, 6];
    /// let vecgrid = Vecgrid::from_column_major(column_major, 2, 3)?;
    /// assert_eq!(vecgrid[(1, 2)], 6);
    /// assert_eq!(vecgrid.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    /// [column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_column_major(
        elements: Vec<T>,
        num_rows: usize,
        num_columns: usize,
    ) -> Result<Self, Error>
    where
        T: Clone, // TODO: remove type guard
    {
        let total_len = num_rows * num_columns;
        if total_len != elements.len() {
            return Err(Error::DimensionMismatch);
        }
        let indices_row_major =
            (0..num_rows).flat_map(move |row| (0..num_columns).map(move |column| (row, column)));
        let vecgrid = indices_row_major
            .map(|(row, column)| {
                let index = column * num_rows + row;
                elements[index].clone()
            })
            .collect();
        Ok(Vecgrid {
            vecgrid,
            num_rows,
            num_columns,
        })
    }

    /// Creates a new [`Vecgrid`] with the specified number of rows and columns
    /// that contains `element` in every location.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// let vecgrid = Vecgrid::filled_with(42, 2, 3);
    /// assert_eq!(vecgrid.as_rows(), vec![vec![42, 42, 42], vec![42, 42, 42]]);
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    pub fn filled_with(element: T, num_rows: usize, num_columns: usize) -> Self
    where
        T: Clone,
    {
        let total_len = num_rows * num_columns;
        let vecgrid = vec![element; total_len];
        Vecgrid {
            vecgrid,
            num_rows,
            num_columns,
        }
    }

    /// Creates a new [`Vecgrid`] with the specified number of rows and columns
    /// and fills each element with the result of calling the given
    /// function. The function is called once for every location going in
    /// row major order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// let mut counter = 1;
    /// let increment = || {
    ///     let tmp = counter;
    ///     counter += 1;
    ///     tmp
    /// };
    /// let vecgrid = Vecgrid::filled_by_row_major(increment, 2, 3);
    /// assert_eq!(vecgrid.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    pub fn filled_by_row_major<F>(mut generator: F, num_rows: usize, num_columns: usize) -> Self
    where
        F: FnMut() -> T,
    {
        let total_len = num_rows * num_columns;
        let vecgrid = (0..total_len).map(|_| generator()).collect();
        Vecgrid {
            vecgrid,
            num_rows,
            num_columns,
        }
    }

    /// Creates a new [`Vecgrid`] with the specified number of rows and columns
    /// and fills each element with the result of calling the given
    /// function. The function is called once for every location going in
    /// column major order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// let mut counter = 1;
    /// let increment = || {
    ///     let tmp = counter;
    ///     counter += 1;
    ///     tmp
    /// };
    /// let vecgrid = Vecgrid::filled_by_column_major(increment, 2, 3);
    /// assert_eq!(vecgrid.as_columns(), vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    pub fn filled_by_column_major<F>(mut generator: F, num_rows: usize, num_columns: usize) -> Self
    where
        F: FnMut() -> T,
        T: Clone,
    {
        let total_len = num_rows * num_columns;
        let vecgrid_column_major = (0..total_len).map(|_| generator()).collect::<Vec<_>>();
        Vecgrid::from_column_major(vecgrid_column_major, num_rows, num_columns)
            .expect("Filled by should never fail")
    }

    /// Creates a new [`Vecgrid`] with the specified number of rows and columns
    /// and fills each element with the elements produced from the provided
    /// iterator. If the iterator produces more than enough elements, the
    /// remaining are unused. Returns an error if the iterator does not produce
    /// enough elements.
    ///
    /// The elements are inserted into the vecgrid in [row major order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let iterator = (1..);
    /// let vecgrid = Vecgrid::from_iter_row_major(iterator, 2, 3)?;
    /// assert_eq!(vecgrid.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    /// [row major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_iter_row_major<I>(
        iterator: I,
        num_rows: usize,
        num_columns: usize,
    ) -> Result<Self, Error>
    where
        I: Iterator<Item = T>,
    {
        let total_len = num_rows * num_columns;
        let vecgrid = iterator.take(total_len).collect::<Vec<_>>();
        if vecgrid.len() != total_len {
            return Err(Error::NotEnoughElements);
        }
        Ok(Vecgrid {
            vecgrid,
            num_rows,
            num_columns,
        })
    }

    /// Creates a new [`Vecgrid`] with the specified number of rows and columns
    /// and fills each element with the elements produced from the provided
    /// iterator. If the iterator produces more than enough elements, the
    /// remaining are unused. Returns an error if the iterator does not produce
    /// enough elements.
    ///
    /// The elements are inserted into the vecgrid in [column major order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let iterator = (1..);
    /// let vecgrid = Vecgrid::from_iter_column_major(iterator, 2, 3)?;
    /// assert_eq!(vecgrid.as_rows(), vec![vec![1, 3, 5], vec![2, 4, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    /// [column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_iter_column_major<I>(
        iterator: I,
        num_rows: usize,
        num_columns: usize,
    ) -> Result<Self, Error>
    where
        I: Iterator<Item = T>,
        T: Clone,
    {
        let total_len = num_rows * num_columns;
        let vecgrid_column_major = iterator.take(total_len).collect::<Vec<_>>();
        Vecgrid::from_column_major(vecgrid_column_major, num_rows, num_columns)
            .map_err(|_| Error::NotEnoughElements)
    }

    /// The number of rows.
    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    /// The number of columns.
    pub fn num_columns(&self) -> usize {
        self.num_columns
    }

    /// The total number of elements, i.e. the product of `num_rows` and
    /// `num_columns`.
    pub fn num_elements(&self) -> usize {
        self.num_rows * self.num_columns
    }

    /// The number of elements in each row, i.e. the number of columns.
    pub fn row_len(&self) -> usize {
        self.num_columns
    }

    /// The number of elements in each column, i.e. the number of rows.
    pub fn column_len(&self) -> usize {
        self.num_rows
    }

    /// Returns a reference to the element at the given `row` and `column` if the
    /// index is in bounds (wrapped in [`Some`]). Returns [`None`] if the index
    /// is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// let vecgrid = Vecgrid::filled_with(42, 2, 3);
    /// assert_eq!(vecgrid.get(0, 0), Some(&42));
    /// assert_eq!(vecgrid.get(10, 10), None);
    /// ```
    ///
    /// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.get_index(row, column)
            .map(|index| &self.vecgrid[index])
    }

    /// Returns a reference to the element at the given index in row major
    /// order. Returns [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// assert_eq!(vecgrid.get_row_major(2), Some(&3));
    /// assert_eq!(vecgrid.get_row_major(4), Some(&5));
    /// assert_eq!(vecgrid.get_row_major(10), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get_row_major(&self, index: usize) -> Option<&T> {
        self.vecgrid.get(index)
    }

    /// Returns a reference to the element at the given index in column major
    /// order. Returns [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// assert_eq!(vecgrid.get_column_major(2), Some(&2));
    /// assert_eq!(vecgrid.get_column_major(4), Some(&3));
    /// assert_eq!(vecgrid.get_column_major(10), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get_column_major(&self, index: usize) -> Option<&T> {
        let column = dbg!(dbg!(index) / self.num_rows);
        let row = dbg!(index % self.num_rows);
        self.get(row, column)
    }

    /// Returns a mutable reference to the element at the given `row` and
    /// `column` if the index is in bounds (wrapped in [`Some`]). Returns
    /// [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// let mut vecgrid = Vecgrid::filled_with(42, 2, 3);
    ///
    /// assert_eq!(vecgrid.get_mut(0, 0), Some(&mut 42));
    /// assert_eq!(vecgrid.get_mut(10, 10), None);
    ///
    /// vecgrid.get_mut(0, 0).map(|x| *x = 100);
    /// assert_eq!(vecgrid.get(0, 0), Some(&100));
    ///
    /// vecgrid.get_mut(10, 10).map(|x| *x = 200);
    /// assert_eq!(vecgrid.get(10, 10), None);
    /// ```
    ///
    /// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.get_index(row, column)
            .map(move |index| &mut self.vecgrid[index])
    }

    /// Returns a mutable reference to the element at the given index in row
    /// major order. Returns [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    ///
    /// assert_eq!(vecgrid.get_mut_row_major(1), Some(&mut 2));
    /// assert_eq!(vecgrid.get_mut_row_major(10), None);
    ///
    /// vecgrid.get_mut_row_major(3).map(|x| *x = 100);
    /// assert_eq!(vecgrid.get(1, 0), Some(&100));
    ///
    /// vecgrid.get_mut_row_major(10).map(|x| *x = 200);
    /// assert_eq!(vecgrid.get(10, 10), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get_mut_row_major(&mut self, index: usize) -> Option<&mut T> {
        self.vecgrid.get_mut(index)
    }

    /// Returns a mutable reference to the element at the given index in row
    /// major order. Returns [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    ///
    /// assert_eq!(vecgrid.get_mut_column_major(1), Some(&mut 4));
    /// assert_eq!(vecgrid.get_mut_column_major(10), None);
    ///
    /// vecgrid.get_mut_column_major(4).map(|x| *x = 100);
    /// assert_eq!(vecgrid.get(0, 2), Some(&100));
    ///
    /// vecgrid.get_mut_column_major(10).map(|x| *x = 200);
    /// assert_eq!(vecgrid.get(10, 10), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get_mut_column_major(&mut self, index: usize) -> Option<&mut T> {
        let column = index / self.num_rows;
        let row = index % self.num_rows;
        self.get_mut(row, column)
    }

    /// Changes the element at given `row` and `column` to `element`. Returns
    /// [`Ok(())`] if the indices were in bounds and returns an [`Err`]
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// let mut vecgrid = Vecgrid::filled_with(42, 2, 3);
    ///
    /// let result = vecgrid.set(0, 0, 100);
    /// assert_eq!(result, Ok(()));
    /// assert_eq!(vecgrid.get(0, 0), Some(&100));
    ///
    /// let result = vecgrid.set(10, 20, 200);
    /// assert_eq!(result, Err(Error::IndicesOutOfBounds(10, 20)));
    /// ```
    ///
    /// [`Ok(())`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [vecgrid::Error]: enum.Error.html
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`vecgrid::Error`]: enum.Error.html
    pub fn set(&mut self, row: usize, column: usize, element: T) -> Result<(), Error> {
        self.get_mut(row, column)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndicesOutOfBounds(row, column))
    }

    /// Changes the element at the given `index` to `element`, in row major
    /// order. Returns [`Ok(())`] if the index is in bounds and returns an
    /// [`Err`] otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// let mut vecgrid = Vecgrid::filled_with(42, 2, 3);
    ///
    /// let result = vecgrid.set_row_major(4, 100);
    /// assert_eq!(result, Ok(()));
    /// assert_eq!(vecgrid.get(1, 1), Some(&100));
    ///
    /// let result = vecgrid.set_row_major(10, 200);
    /// assert_eq!(result, Err(Error::IndexOutOfBounds(10)));
    /// ```
    ///
    /// [`Ok(())`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [vecgrid::Error]: enum.Error.html
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`vecgrid::Error`]: enum.Error.html
    pub fn set_row_major(&mut self, index: usize, element: T) -> Result<(), Error> {
        self.get_mut_row_major(index)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndexOutOfBounds(index))
    }

    /// Changes the element at the given `index` to `element`, in column major
    /// order. Returns [`Ok(())`] if the index is in bounds and returns an
    /// [`Err`] otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// let mut vecgrid = Vecgrid::filled_with(42, 2, 3);
    ///
    /// let result = vecgrid.set_column_major(4, 100);
    /// assert_eq!(result, Ok(()));
    /// assert_eq!(vecgrid.get(0, 2), Some(&100));
    ///
    /// let result = vecgrid.set_column_major(10, 200);
    /// assert_eq!(result, Err(Error::IndexOutOfBounds(10)));
    /// ```
    ///
    /// [`Ok(())`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [vecgrid::Error]: enum.Error.html
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`vecgrid::Error`]: enum.Error.html
    pub fn set_column_major(&mut self, index: usize, element: T) -> Result<(), Error> {
        self.get_mut_column_major(index)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndexOutOfBounds(index))
    }

    /// Returns an [`Iterator`] over references to all elements in [row major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let elements = vec![1, 2, 3, 4, 5, 6];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// let row_major = vecgrid.elements_row_major_iter();
    /// assert_eq!(row_major.cloned().collect::<Vec<_>>(), elements);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [row major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn elements_row_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone {
        self.vecgrid.iter()
    }

    /// Returns an [`Iterator`] over mutable references to all elements in [row major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    ///    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    ///    let elements = vec![1, 2, 3, 4, 5, 6];
    ///    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    ///    let row_major = vecgrid.elements_row_major_iter_mut();
    ///    for (i, val) in row_major
    ///        .map(|val| {
    ///            *val += 1;
    ///            val
    ///        })
    ///        .enumerate()
    ///    {
    ///        assert_eq!(*val, elements[i] + 1);
    ///    }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [row major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn elements_row_major_iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> {
        self.vecgrid.iter_mut()
    }

    /// Returns an [`Iterator`] over references to all elements in [column major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let elements = vec![1, 4, 2, 5, 3, 6];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// let column_major = vecgrid.elements_column_major_iter();
    /// assert_eq!(column_major.cloned().collect::<Vec<_>>(), elements);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn elements_column_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone {
        self.indices_column_major().map(move |i| &self[i])
    }

    /// Returns an [`Iterator`] over mutable references to all elements in [column major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    /// let elements = vec![1, 4, 7, 2, 5, 8, 3, 6, 9];
    /// let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// let column_major = vecgrid.elements_column_major_iter_mut();
    /// for (i, val) in column_major
    ///     .map(|val| {
    ///         *val += 1;
    ///         val
    ///     })
    ///     .enumerate()
    /// {
    ///     assert_eq!(*val, elements[i] + 1);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn elements_column_major_iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> {
        self.columns_iter_mut().flatten()
    }

    /// Returns an [`Iterator`] over references to all elements in the given
    /// row. Returns an error if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// let mut row_iter = vecgrid.row_iter(1)?;
    /// assert_eq!(row_iter.next(), Some(&4));
    /// assert_eq!(row_iter.next(), Some(&5));
    /// assert_eq!(row_iter.next(), Some(&6));
    /// assert_eq!(row_iter.next(), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    pub fn row_iter(&self, row_index: usize) -> Result<impl DoubleEndedIterator<Item = &T>, Error> {
        let start = self
            .get_index(row_index, 0)
            .ok_or(Error::IndicesOutOfBounds(row_index, 0))?;
        let end = start + self.row_len();
        Ok(self.vecgrid[start..end].iter())
    }

    /// Returns an [`Iterator`] over mutable references to all elements in the given
    /// row. Returns an error if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// let mut row_iter = vecgrid.row_iter_mut(1)?;
    /// assert_eq!(row_iter.next(), Some(&mut 4));
    /// assert_eq!(row_iter.next(), Some(&mut 5));
    /// assert_eq!(row_iter.next(), Some(&mut 6));
    /// assert_eq!(row_iter.next(), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    pub fn row_iter_mut(
        &mut self,
        row_index: usize,
    ) -> Result<impl DoubleEndedIterator<Item = &mut T>, Error> {
        let start = self
            .get_index(row_index, 0)
            .ok_or(Error::IndicesOutOfBounds(row_index, 0))?;
        let end = start + self.row_len();
        Ok(self.vecgrid[start..end].iter_mut())
    }

    /// Returns an [`Iterator`] over references to all elements in the given
    /// column. Returns an error if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// let mut column_iter = vecgrid.column_iter(1)?;
    /// assert_eq!(column_iter.next(), Some(&2));
    /// assert_eq!(column_iter.next(), Some(&5));
    /// assert_eq!(column_iter.next(), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    pub fn column_iter(
        &self,
        column_index: usize,
    ) -> Result<impl DoubleEndedIterator<Item = &T>, Error> {
        if column_index >= self.num_columns {
            return Err(Error::IndicesOutOfBounds(0, column_index));
        }
        Ok((0..self.column_len()).map(move |row_index| &self[(row_index, column_index)]))
    }

    /// Returns an [`Iterator`] over mutable references to all elements in the given
    /// column. Returns an error if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// let mut column_iter = vecgrid.column_iter_mut(1)?;
    /// assert_eq!(column_iter.next(), Some(&mut 2));
    /// assert_eq!(column_iter.next(), Some(&mut 5));
    /// assert_eq!(column_iter.next(), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    pub fn column_iter_mut(
        &mut self,
        column_index: usize,
    ) -> Result<impl DoubleEndedIterator<Item = &mut T>, Error> {
        if column_index >= self.num_columns {
            return Err(Error::IndicesOutOfBounds(0, column_index));
        }
        Ok(self
            .vecgrid
            .iter_mut()
            .skip(column_index)
            .step_by(self.num_columns))
    }

    /// Returns an [`Iterator`] over all rows. Each [`Item`] is itself another
    /// [`Iterator`] over references to the elements in that row.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// for row_iter in vecgrid.rows_iter() {
    ///     for element in row_iter {
    ///         print!("{} ", element);
    ///     }
    ///     println!();
    /// }
    ///
    /// let mut rows_iter = vecgrid.rows_iter();
    ///
    /// let mut first_row_iter = rows_iter.next().unwrap();
    /// assert_eq!(first_row_iter.next(), Some(&1));
    /// assert_eq!(first_row_iter.next(), Some(&2));
    /// assert_eq!(first_row_iter.next(), Some(&3));
    /// assert_eq!(first_row_iter.next(), None);
    ///
    /// let mut second_row_iter = rows_iter.next().unwrap();
    /// assert_eq!(second_row_iter.next(), Some(&4));
    /// assert_eq!(second_row_iter.next(), Some(&5));
    /// assert_eq!(second_row_iter.next(), Some(&6));
    /// assert_eq!(second_row_iter.next(), None);
    ///
    /// assert!(rows_iter.next().is_none());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [`Item`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item
    pub fn rows_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> + Clone {
        (0..self.num_rows()).map(move |row_index| {
            self.row_iter(row_index)
                .expect("rows_iter should never fail")
        })
    }

    /// Returns an [`Iterator`] over all rows. Each [`Item`] is itself another
    /// [`Iterator`] over mutable references to the elements in that row.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// for row_iter in vecgrid.rows_iter() {
    ///     for element in row_iter {
    ///         print!("{} ", element);
    ///     }
    ///     println!();
    /// }
    ///
    /// let mut rows_iter = vecgrid.rows_iter_mut();
    ///
    /// let mut first_row_iter = rows_iter.next().unwrap();
    /// assert_eq!(first_row_iter.next(), Some(&mut 1));
    /// assert_eq!(first_row_iter.next(), Some(&mut 2));
    /// assert_eq!(first_row_iter.next(), Some(&mut 3));
    /// assert_eq!(first_row_iter.next(), None);
    ///
    /// let mut second_row_iter = rows_iter.next().unwrap();
    /// assert_eq!(second_row_iter.next(), Some(&mut 4));
    /// assert_eq!(second_row_iter.next(), Some(&mut 5));
    /// assert_eq!(second_row_iter.next(), Some(&mut 6));
    /// assert_eq!(second_row_iter.next(), None);
    ///
    /// assert!(rows_iter.next().is_none());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [`Item`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item
    pub fn rows_iter_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &mut T>> {
        let row_len = self.row_len();
        self.vecgrid.chunks_mut(row_len).map(|r| r.iter_mut())
    }

    /// Returns an [`Iterator`] over all columns. Each [`Item`] is itself
    /// another [`Iterator`] over references to the elements in that column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// for column_iter in vecgrid.columns_iter() {
    ///     for element in column_iter {
    ///         print!("{} ", element);
    ///     }
    ///     println!();
    /// }
    ///
    /// let mut columns_iter = vecgrid.columns_iter();
    ///
    /// let mut first_column_iter = columns_iter.next().unwrap();
    /// assert_eq!(first_column_iter.next(), Some(&1));
    /// assert_eq!(first_column_iter.next(), Some(&4));
    /// assert_eq!(first_column_iter.next(), None);
    ///
    /// let mut second_column_iter = columns_iter.next().unwrap();
    /// assert_eq!(second_column_iter.next(), Some(&2));
    /// assert_eq!(second_column_iter.next(), Some(&5));
    /// assert_eq!(second_column_iter.next(), None);
    ///
    /// let mut third_column_iter = columns_iter.next().unwrap();
    /// assert_eq!(third_column_iter.next(), Some(&3));
    /// assert_eq!(third_column_iter.next(), Some(&6));
    /// assert_eq!(third_column_iter.next(), None);
    ///
    /// assert!(columns_iter.next().is_none());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [`Item`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item
    pub fn columns_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> + Clone {
        (0..self.num_columns).map(move |column_index| {
            self.column_iter(column_index)
                .expect("columns_iter should never fail")
        })
    }

    /// Returns an [`Iterator`] over all columns. Each [`Item`] is itself
    /// another [`Iterator`] over mutable references to the elements in that column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// for column_iter in vecgrid.columns_iter_mut() {
    ///     for element in column_iter {
    ///         print!("{} ", element);
    ///     }
    ///     println!();
    /// }
    ///
    /// let mut columns_iter = vecgrid.columns_iter_mut();
    ///
    /// let mut first_column_iter = columns_iter.next().unwrap();
    /// assert_eq!(first_column_iter.next(), Some(&mut 1));
    /// assert_eq!(first_column_iter.next(), Some(&mut 4));
    /// assert_eq!(first_column_iter.next(), None);
    ///
    /// let mut second_column_iter = columns_iter.next().unwrap();
    /// assert_eq!(second_column_iter.next(), Some(&mut 2));
    /// assert_eq!(second_column_iter.next(), Some(&mut 5));
    /// assert_eq!(second_column_iter.next(), None);
    ///
    /// let mut third_column_iter = columns_iter.next().unwrap();
    /// assert_eq!(third_column_iter.next(), Some(&mut 3));
    /// assert_eq!(third_column_iter.next(), Some(&mut 6));
    /// assert_eq!(third_column_iter.next(), None);
    ///
    /// assert!(columns_iter.next().is_none());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [`Item`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item
    pub fn columns_iter_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &mut T>> {
        let (num_columns, num_rows) = (self.num_columns(), self.num_rows());
        let pointer = self.vecgrid.as_mut_ptr();
        (0..num_columns).map(move |ci| {
            (0..num_rows).map(move |i| {
                let offset = (i * num_columns) + ci;
                unsafe { &mut *pointer.add(offset) }
            })
        })
    }

    /// Collects the [`Vecgrid`] into a [`Vec`] of rows, each of which contains
    /// a [`Vec`] of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// assert_eq!(vecgrid.as_rows(), rows);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn as_rows(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        self.rows_iter()
            .map(|row_iter| row_iter.cloned().collect())
            .collect()
    }

    /// Collects the [`Vecgrid`] into a [`Vec`] of columns, each of which
    /// contains a [`Vec`] of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let columns = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    /// let vecgrid = Vecgrid::from_columns(columns.clone())?;
    /// assert_eq!(vecgrid.as_columns(), columns);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn as_columns(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        self.columns_iter()
            .map(|column_iter| column_iter.cloned().collect())
            .collect()
    }

    /// Collects the [`Vecgrid`] into a [`Vec`] of elements in [row major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// assert_eq!(vecgrid.as_row_major(), vec![1, 2, 3, 4, 5, 6]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [row major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn as_row_major(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.elements_row_major_iter().cloned().collect()
    }

    /// Collects the [`Vecgrid`] into a [`Vec`] of elements in [column major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// assert_eq!(vecgrid.as_column_major(), vec![1, 4, 2, 5, 3, 6]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Vecgrid`]: struct.Vecgrid.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn as_column_major(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.elements_column_major_iter().cloned().collect()
    }

    /// Returns the indices of the vecgrid in row major order. Each index is a tuple of [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// let indices_row_major = vecgrid.indices_row_major().collect::<Vec<_>>();
    /// assert_eq!(
    ///     indices_row_major,
    ///     vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
    pub fn indices_row_major(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
        indices_row_major(self.num_rows, self.num_columns)
    }

    /// Returns the indices of the vecgrid in column major order. Each index is a tuple of [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// let indices_column_major = vecgrid.indices_column_major().collect::<Vec<_>>();
    /// assert_eq!(
    ///     indices_column_major,
    ///     vec![(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)]
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
    pub fn indices_column_major(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
        indices_column_major(self.num_rows, self.num_columns)
    }

    /// Iterate through the vecgrid in row major order along with the corresponding indices. Each
    /// index is a tuple of [`usize`].
    ///
    /// # Examples
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// let enumerate_row_major = vecgrid.enumerate_row_major().collect::<Vec<_>>();
    /// assert_eq!(
    ///     enumerate_row_major,
    ///     vec![
    ///         ((0, 0), &1),
    ///         ((0, 1), &2),
    ///         ((0, 2), &3),
    ///         ((1, 0), &4),
    ///         ((1, 1), &5),
    ///         ((1, 2), &6)
    ///     ]
    /// );
    /// # Ok(())
    /// # }
    ///
    /// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
    pub fn enumerate_row_major(
        &self,
    ) -> impl DoubleEndedIterator<Item = ((usize, usize), &T)> + Clone {
        self.indices_row_major().map(move |i| (i, &self[i]))
    }

    /// Iterate through the vecgrid in column major order along with the corresponding indices. Each
    /// index is a tuple of [`usize`].
    ///
    /// # Examples
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// let enumerate_column_major = vecgrid.enumerate_column_major().collect::<Vec<_>>();
    /// assert_eq!(
    ///     enumerate_column_major,
    ///     vec![
    ///         ((0, 0), &1),
    ///         ((1, 0), &4),
    ///         ((0, 1), &2),
    ///         ((1, 1), &5),
    ///         ((0, 2), &3),
    ///         ((1, 2), &6)
    ///     ]
    /// );
    /// # Ok(())
    /// # }
    ///
    /// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
    pub fn enumerate_column_major(
        &self,
    ) -> impl DoubleEndedIterator<Item = ((usize, usize), &T)> + Clone {
        self.indices_column_major().map(move |i| (i, &self[i]))
    }

    fn get_index(&self, row: usize, column: usize) -> Option<usize> {
        if row < self.num_rows && column < self.num_columns {
            Some(row * self.row_len() + column)
        } else {
            None
        }
    }

    /// Inserts a new row into the vecgrid at the provided index of the row.
    /// Guards ensure that the supplied row matches the expected dimensions and that
    /// the index is in bound.
    ///
    /// # Examples
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![7, 8, 9]];
    /// let new_row = vec![4, 5, 6];
    /// let result = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    /// let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// vecgrid.insert_row(new_row, 1)?;
    /// assert_eq!(vecgrid.as_rows(), result);
    /// # Ok(())
    /// # }
    ///
    pub fn insert_row(&mut self, row: Vec<T>, at: usize) -> Result<(), Error> {
        match (row.len() == self.num_columns, at < self.num_rows) {
            (false, _) => Err(Error::DimensionMismatch),
            (_, false) => Err(Error::IndexOutOfBounds(at)),
            (true, true) => {
                let i = at * self.row_len();
                self.vecgrid.splice(i..i, row);
                self.num_rows += 1;
                Ok(())
            }
        }
    }

    /// Inserts a slice of rows into the vecgrid at the provided index.
    /// Guards ensure that the supplied rows matches the expected dimensions and that
    /// the index is in bound.
    ///
    /// # Examples
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2], vec![7, 8]];
    /// let new_rows = vec![vec![3, 4], vec![5, 6]];
    /// let result = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
    /// let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// vecgrid.insert_rows(new_row, 1)?;
    /// assert_eq!(vecgrid.as_rows(), result);
    /// # Ok(())
    /// # }
    ///
    pub fn insert_rows(&mut self, mut rows: Vec<Vec<T>>, at: usize) -> Result<(), Error> {
        match (
            rows.iter_mut().all(|r| r.len() == self.num_columns),
            at < self.num_rows + 1,
        ) {
            (false, _) => Err(Error::DimensionMismatch),
            (_, false) => Err(Error::IndexOutOfBounds(at)),
            (true, true) => {
                let i = at * self.row_len();
                let capacity = self.num_columns * rows.len();
                let num_new_rows = rows.len();

                self.vecgrid
                    .splice(i..i, with_size_hint(rows.into_iter().flatten(), capacity));
                self.num_rows += num_new_rows;
                Ok(())
            }
        }
    }

    /// Appends a vec of rows at the end of the vecgrid.
    /// Guards ensure that the supplied rows matches the expected dimensions.
    ///
    /// # Examples
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2], vec![3, 4]];
    /// let new_rows = vec![vec![5, 6], vec![7, 8]];
    /// let result = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
    /// let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    /// vecgrid.append_rows(new_row)?;
    /// assert_eq!(vecgrid.as_rows(), result);
    /// # Ok(())
    /// # }
    ///
    pub fn append_rows(&mut self, rows: Vec<Vec<T>>) -> Result<(), Error> {
        self.insert_rows(rows, self.num_rows)
    }

    /// Removes a row at the provided row index from the vecgrid.
    /// Guards ensure that the index is in bound.
    ///
    /// # Examples
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    /// let result = vec![vec![1, 2, 3], vec![7, 8, 9]];
    /// let mut vecgrid = Vecgrid::from_rows(rows)?;
    /// vecgrid.remove_row(1)?;
    /// assert_eq!(vecgrid.as_rows(), result);
    /// # Ok(())
    /// # }
    ///
    pub fn remove_row(&mut self, at: usize) -> Result<(), Error> {
        self.remove_rows(at, 1)
    }

    /// Removes `n` consecutive rows at the provided row index from the vecgrid.
    /// Guards ensure that the index is in bound.
    ///
    /// # Examples
    /// # use vecgrid::{Vecgrid, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
    /// let result = vec![vec![1, 2], vec![7, 8]];
    /// let mut vecgrid = Vecgrid::from_rows(rows)?;
    /// vecgrid.remove_rows(1, 2)?;
    /// assert_eq!(vecgrid.as_rows(), result);
    /// # Ok(())
    /// # }
    ///
    pub fn remove_rows(&mut self, at: usize, n: usize) -> Result<(), Error> {
        if at > self.num_rows && at + n > self.num_rows + 1 {
            return Err(Error::IndicesOutOfBounds(at, at + n));
        }
        let start = self.row_len() * at;
        let end = start + n * self.row_len();
        self.vecgrid.drain(start..end);
        self.num_rows -= n;
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Vecgrid<T> {
    type Output = T;

    /// Returns the element at the given indices, given as `(row, column)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// let vecgrid = Vecgrid::filled_with(42, 2, 3);
    /// assert_eq!(vecgrid[(0, 0)], 42);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the indices are out of bounds.
    ///
    /// ```rust,should_panic
    /// # use vecgrid::Vecgrid;
    /// let vecgrid = Vecgrid::filled_with(42, 2, 3);
    /// let element = vecgrid[(10, 10)];
    /// ```
    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        self.get(row, column)
            .unwrap_or_else(|| panic!("Index indices {}, {} out of bounds", row, column))
    }
}

impl<T> IndexMut<(usize, usize)> for Vecgrid<T> {
    /// Returns a mutable version of the element at the given indices, given as
    /// `(row, column)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vecgrid::{Vecgrid, Error};
    /// let mut vecgrid = Vecgrid::filled_with(42, 2, 3);
    /// vecgrid[(0, 0)] = 100;
    /// assert_eq!(vecgrid[(0, 0)], 100);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the indices are out of bounds.
    ///
    /// ```rust,should_panic
    /// # use vecgrid::Vecgrid;
    /// let mut vecgrid = Vecgrid::filled_with(42, 2, 3);
    /// vecgrid[(10, 10)] = 7;
    /// ```
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, column)
            .unwrap_or_else(|| panic!("Index mut indices {}, {} out of bounds", row, column))
    }
}

struct SizeHint<I: Iterator> {
    inner: I,
    size_hint: usize,
}
impl<I: Iterator> Iterator for SizeHint<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.size_hint = self.size_hint.saturating_sub(1);
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size_hint, Some(self.size_hint))
    }
}
fn with_size_hint<I: Iterator>(inner: I, size_hint: usize) -> SizeHint<I> {
    SizeHint { inner, size_hint }
}

fn indices_row_major(
    num_rows: usize,
    num_columns: usize,
) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
    (0..num_rows).flat_map(move |row| (0..num_columns).map(move |column| (row, column)))
}

fn indices_column_major(
    num_rows: usize,
    num_columns: usize,
) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
    (0..num_columns).flat_map(move |column| (0..num_rows).map(move |row| (row, column)))
}

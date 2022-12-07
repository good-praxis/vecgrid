use vecgrid::{Error, Vecgrid};

////////////////////////////////////////////////////////////////////////////////
// Normal Operation ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_from_rows() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    assert_eq!(vecgrid.as_rows(), rows);
    Ok(())
}

#[test]
fn test_from_columns() -> Result<(), Error> {
    let columns = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    let vecgrid = Vecgrid::from_columns(columns.clone())?;
    assert_eq!(vecgrid.as_columns(), columns);
    Ok(())
}

#[test]
fn test_from_row_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let row_major = vec![1, 2, 3, 4, 5, 6];
    let num_rows = 2;
    let num_columns = 3;
    let vecgrid = Vecgrid::from_row_major(row_major, num_rows, num_columns)?;
    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, element) in row.iter().enumerate() {
            assert_eq!(vecgrid.get(row_index, column_index), Some(element));
        }
    }
    Ok(())
}

#[test]
fn test_from_column_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let column_major = vec![1, 4, 2, 5, 3, 6];
    let num_rows = 2;
    let num_columns = 3;
    let vecgrid = Vecgrid::from_column_major(column_major, num_rows, num_columns)?;
    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, element) in row.iter().enumerate() {
            assert_eq!(vecgrid.get(row_index, column_index), Some(element));
        }
    }
    Ok(())
}

#[test]
fn test_filled_with() -> Result<(), Error> {
    let element = 7;
    let vecgrid = Vecgrid::filled_with(element, 4, 5);
    assert_eq!(vecgrid.num_rows(), 4);
    assert_eq!(vecgrid.num_columns(), 5);
    assert_eq!(vecgrid.num_elements(), 20);
    for element in vecgrid.elements_row_major_iter() {
        assert_eq!(element, &7);
    }
    for element in vecgrid.elements_column_major_iter() {
        assert_eq!(element, &7);
    }
    Ok(())
}

#[test]
fn test_filled_by_row_major() -> Result<(), Error> {
    let mut counter = 1;
    let increment = || {
        let tmp = counter;
        counter += 1;
        tmp
    };
    let vecgrid = Vecgrid::filled_by_row_major(increment, 2, 3);
    assert_eq!(vecgrid.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    Ok(())
}

#[test]
fn test_filled_by_column_major() -> Result<(), Error> {
    let mut counter = 1;
    let increment = || {
        let tmp = counter;
        counter += 1;
        tmp
    };
    let vecgrid = Vecgrid::filled_by_column_major(increment, 2, 3);
    assert_eq!(
        vecgrid.as_columns(),
        vec![vec![1, 2], vec![3, 4], vec![5, 6]]
    );
    Ok(())
}

#[test]
fn test_from_iter_row_major() -> Result<(), Error> {
    let vecgrid = Vecgrid::from_iter_row_major(1.., 2, 3)?;
    assert_eq!(vecgrid.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    Ok(())
}

#[test]
fn test_from_iter_column_major() -> Result<(), Error> {
    let vecgrid = Vecgrid::from_iter_column_major(1.., 2, 3)?;
    assert_eq!(
        vecgrid.as_columns(),
        vec![vec![1, 2], vec![3, 4], vec![5, 6]]
    );
    Ok(())
}

#[test]
fn test_dimensions() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    assert_eq!(vecgrid.num_rows(), 2);
    assert_eq!(vecgrid.num_columns(), 3);
    assert_eq!(vecgrid.row_len(), 3);
    assert_eq!(vecgrid.column_len(), 2);
    Ok(())
}

#[test]
fn test_get() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            assert_eq!(vecgrid.get(row, column), Some(&rows[row][column]));
        }
    }
    Ok(())
}

#[test]
fn test_get_row_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    assert_eq!(vecgrid.get_row_major(0), Some(&1));
    assert_eq!(vecgrid.get_row_major(1), Some(&2));
    assert_eq!(vecgrid.get_row_major(2), Some(&3));
    assert_eq!(vecgrid.get_row_major(3), Some(&4));
    assert_eq!(vecgrid.get_row_major(4), Some(&5));
    assert_eq!(vecgrid.get_row_major(5), Some(&6));
    assert_eq!(vecgrid.get_row_major(6), None);
    Ok(())
}

#[test]
fn test_get_column_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    assert_eq!(vecgrid.get_column_major(0), Some(&1));
    assert_eq!(vecgrid.get_column_major(1), Some(&4));
    assert_eq!(vecgrid.get_column_major(2), Some(&2));
    assert_eq!(vecgrid.get_column_major(3), Some(&5));
    assert_eq!(vecgrid.get_column_major(4), Some(&3));
    assert_eq!(vecgrid.get_column_major(5), Some(&6));
    assert_eq!(vecgrid.get_column_major(6), None);
    Ok(())
}

#[test]
fn test_get_mut() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    let (set_row, set_column) = (0, 2);
    let element = 53;
    let element_ref_option = vecgrid.get_mut(set_row, set_column);
    assert!(element_ref_option.is_some());
    let element_ref = element_ref_option.unwrap();
    assert_eq!(element_ref, &rows[set_row][set_column]);
    *element_ref = element;
    assert_eq!(element_ref, &element);
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            let actual = vecgrid.get(row, column);
            if (row, column) == (set_row, set_column) {
                assert_eq!(actual, Some(&element));
            } else {
                assert_eq!(actual, Some(&rows[row][column]));
            }
        }
    }
    Ok(())
}

#[test]
fn test_get_mut_row_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    assert_eq!(vecgrid.get_mut_row_major(0), Some(&mut 1));
    assert_eq!(vecgrid.get_mut_row_major(1), Some(&mut 2));
    assert_eq!(vecgrid.get_mut_row_major(2), Some(&mut 3));
    assert_eq!(vecgrid.get_mut_row_major(3), Some(&mut 4));
    assert_eq!(vecgrid.get_mut_row_major(4), Some(&mut 5));
    assert_eq!(vecgrid.get_mut_row_major(5), Some(&mut 6));
    assert_eq!(vecgrid.get_mut_row_major(6), None);
    Ok(())
}

#[test]
fn test_get_mut_column_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    assert_eq!(vecgrid.get_mut_column_major(0), Some(&mut 1));
    assert_eq!(vecgrid.get_mut_column_major(1), Some(&mut 4));
    assert_eq!(vecgrid.get_mut_column_major(2), Some(&mut 2));
    assert_eq!(vecgrid.get_mut_column_major(3), Some(&mut 5));
    assert_eq!(vecgrid.get_mut_column_major(4), Some(&mut 3));
    assert_eq!(vecgrid.get_mut_column_major(5), Some(&mut 6));
    assert_eq!(vecgrid.get_mut_column_major(6), None);
    Ok(())
}

#[test]
fn test_set() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    let (set_row, set_column) = (1, 0);
    let element = 42;
    vecgrid.set(set_row, set_column, element).unwrap();
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            let actual = vecgrid.get(row, column);
            if (row, column) == (set_row, set_column) {
                assert_eq!(actual, Some(&element));
            } else {
                assert_eq!(actual, Some(&rows[row][column]));
            }
        }
    }
    Ok(())
}

#[test]
fn test_set_row_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    let set_index = 4;
    let set_row = 1;
    let set_column = 1;
    let element = 42;
    vecgrid.set_row_major(set_index, element).unwrap();
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            let actual = vecgrid.get(row, column);
            if (row, column) == (set_row, set_column) {
                assert_eq!(actual, Some(&element));
            } else {
                assert_eq!(actual, Some(&rows[row][column]));
            }
        }
    }
    Ok(())
}

#[test]
fn test_set_column_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    let set_index = 4;
    let set_row = 0;
    let set_column = 2;
    let element = 42;
    vecgrid.set_column_major(set_index, element).unwrap();
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            let actual = vecgrid.get(row, column);
            if (row, column) == (set_row, set_column) {
                assert_eq!(actual, Some(&element));
            } else {
                assert_eq!(actual, Some(&rows[row][column]));
            }
        }
    }
    Ok(())
}

#[test]
fn test_elements_row_major_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let row_major = vec![1, 2, 3, 4, 5, 6];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let row_len = rows[0].len();
    for (index, element) in vecgrid.elements_row_major_iter().enumerate() {
        let row_index = index / row_len;
        let column_index = index % row_len;
        // Do it both ways to make sure we're doing this right
        assert_eq!(element, &rows[row_index][column_index]);
        assert_eq!(element, &row_major[index]);
    }
    Ok(())
}

#[test]
fn test_elements_row_major_iter_mut() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let row_major = vec![1, 2, 3, 4, 5, 6];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    let row_len = rows[0].len();
    for (index, element) in vecgrid.elements_row_major_iter_mut().enumerate() {
        let row_index = index / row_len;
        let column_index = index % row_len;
        // Do it both ways to make sure we're doing this right
        *element += 1;
        assert_eq!(*element, &rows[row_index][column_index] + 1);
        assert_eq!(*element, &row_major[index] + 1);
    }
    Ok(())
}

#[test]
fn test_elements_column_major_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let column_major = vec![1, 4, 2, 5, 3, 6];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let column_len = rows.len();
    for (index, element) in vecgrid.elements_column_major_iter().enumerate() {
        let column_index = index / column_len;
        let row_index = index % column_len;
        // Do it both ways to make sure we're doing this right
        assert_eq!(element, &rows[row_index][column_index]);
        assert_eq!(element, &column_major[index]);
    }
    Ok(())
}

#[test]
fn test_elements_column_major_iter_mut() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let column_major = vec![1, 4, 2, 5, 3, 6];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    let column_len = rows.len();
    for (index, element) in vecgrid.elements_column_major_iter_mut().enumerate() {
        let column_index = index / column_len;
        let row_index = index % column_len;
        // Do it both ways to make sure we're doing this right
        *element += 1;
        assert_eq!(*element, &rows[row_index][column_index] + 1);
        assert_eq!(*element, &column_major[index] + 1);
    }
    Ok(())
}

#[test]
fn test_row_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let first_row_iter = vecgrid.row_iter(0)?;
    for (index, element) in first_row_iter.enumerate() {
        assert_eq!(element, &rows[0][index]);
    }
    let second_row_iter = vecgrid.row_iter(1)?;
    for (index, element) in second_row_iter.enumerate() {
        assert_eq!(element, &rows[1][index]);
    }
    Ok(())
}

#[test]
fn test_row_iter_mut() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    let first_row_iter = vecgrid.row_iter_mut(0)?;
    for (index, element) in first_row_iter.enumerate() {
        *element += 1;
        assert_eq!(*element, &rows[0][index] + 1);
    }
    let second_row_iter = vecgrid.row_iter_mut(1)?;
    for (index, element) in second_row_iter.enumerate() {
        *element += 1;
        assert_eq!(*element, rows[1][index] + 1);
    }
    Ok(())
}

#[test]
fn test_column_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let first_column_iter = vecgrid.column_iter(0)?;
    for (index, element) in first_column_iter.enumerate() {
        assert_eq!(element, &rows[index][0]);
    }
    let second_column_iter = vecgrid.column_iter(1)?;
    for (index, element) in second_column_iter.enumerate() {
        assert_eq!(element, &rows[index][1]);
    }
    Ok(())
}

#[test]
fn test_column_iter_mut() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    let first_column_iter = vecgrid.column_iter_mut(0)?;
    for (index, element) in first_column_iter.enumerate() {
        *element += 1;
        assert_eq!(*element, &rows[index][0] + 1);
    }
    let second_column_iter = vecgrid.column_iter_mut(1)?;
    for (index, element) in second_column_iter.enumerate() {
        *element += 1;
        assert_eq!(*element, &rows[index][1] + 1);
    }
    Ok(())
}

#[test]
fn test_rows_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    for (row_index, row_iter) in vecgrid.rows_iter().enumerate() {
        for (column_index, element) in row_iter.enumerate() {
            assert_eq!(element, &rows[row_index][column_index]);
        }
    }
    Ok(())
}

#[test]
fn test_rows_iter_mut() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    for (row_index, row_iter) in vecgrid.rows_iter_mut().enumerate() {
        for (column_index, element) in row_iter.enumerate() {
            *element += 1;
            assert_eq!(*element, &rows[row_index][column_index] + 1);
        }
    }
    Ok(())
}

#[test]
fn test_columns_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    for (column_index, column_iter) in vecgrid.columns_iter().enumerate() {
        for (row_index, element) in column_iter.enumerate() {
            assert_eq!(element, &rows[row_index][column_index]);
        }
    }
    Ok(())
}

#[test]
fn test_columns_iter_mut() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    for (column_index, column_iter) in vecgrid.columns_iter_mut().enumerate() {
        for (row_index, element) in column_iter.enumerate() {
            *element += 1;
            assert_eq!(*element, &rows[row_index][column_index] + 1);
        }
    }
    Ok(())
}

#[test]
fn test_op_index() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            assert_eq!(vecgrid[(row, column)], rows[row][column]);
        }
    }
    Ok(())
}

#[test]
fn test_op_index_mut() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            vecgrid[(row, column)] += 1;
            assert_eq!(vecgrid[(row, column)], rows[row][column] + 1);
        }
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
// Error Handling //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_from_rows_not_all_same_size() {
    let rows = vec![vec![1, 2, 3], vec![4, 5]];
    let result = Vecgrid::from_rows(rows.clone());
    assert_eq!(result, Err(Error::DimensionMismatch));
}

#[test]
fn test_from_columns_not_all_same_size() {
    let columns = vec![vec![1, 4], vec![2, 3], vec![4]];
    let result = Vecgrid::from_columns(columns.clone());
    assert_eq!(result, Err(Error::DimensionMismatch));
}

#[test]
fn test_from_row_major_dimensions_do_not_match_size() {
    let row_major = vec![1, 2, 3, 4, 5, 6, 7];
    let num_rows = 2;
    let num_columns = 3;
    let result = Vecgrid::from_row_major(row_major, num_rows, num_columns);
    assert_eq!(result, Err(Error::DimensionMismatch));
}

#[test]
fn test_from_column_major_dimensions_do_not_match_size() {
    let column_major = vec![1, 4, 2, 5, 3];
    let num_rows = 2;
    let num_columns = 3;
    let result = Vecgrid::from_column_major(column_major, num_rows, num_columns);
    assert_eq!(result, Err(Error::DimensionMismatch));
}

#[test]
fn test_from_iter_row_major_not_enough() {
    let iter = 1..5;
    let num_rows = 2;
    let num_columns = 3;
    let result = Vecgrid::from_iter_row_major(iter, num_rows, num_columns);
    assert_eq!(result, Err(Error::NotEnoughElements));
}

#[test]
fn test_from_iter_column_major_not_enough() {
    let iter = 1..5;
    let num_rows = 2;
    let num_columns = 3;
    let result = Vecgrid::from_iter_column_major(iter, num_rows, num_columns);
    assert_eq!(result, Err(Error::NotEnoughElements));
}

#[test]
fn test_row_iter_out_of_bounds() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let vecgrid = Vecgrid::filled_with(element, num_rows, num_columns);
    let result = vecgrid.row_iter(num_rows);
    assert!(result.is_err());
}

#[test]
fn test_column_iter_out_of_bounds() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let vecgrid = Vecgrid::filled_with(element, num_rows, num_columns);
    let result = vecgrid.column_iter(num_columns);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_row() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let vecgrid = Vecgrid::filled_with(element, num_rows, num_columns);
    let _ = vecgrid[(num_rows, 0)];
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_column() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let vecgrid = Vecgrid::filled_with(element, num_rows, num_columns);
    let _ = vecgrid[(0, num_columns)];
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_row_and_column() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let vecgrid = Vecgrid::filled_with(element, num_rows, num_columns);
    let _ = vecgrid[(num_rows, num_columns)];
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds_row() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let mut vecgrid = Vecgrid::filled_with(element, num_rows, num_columns);
    vecgrid[(num_rows, 0)] += 1;
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds_column() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let mut vecgrid = Vecgrid::filled_with(element, num_rows, num_columns);
    vecgrid[(0, num_columns)] += 1;
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds_row_and_column() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let mut vecgrid = Vecgrid::filled_with(element, num_rows, num_columns);
    vecgrid[(num_rows, num_columns)] += 1;
}

////////////////////////////////////////////////////////////////////////////////
// Empty Vecgrids ////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_empty_vecgrid_from_rows() -> Result<(), Error> {
    let rows: Vec<Vec<i32>> = vec![];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    assert_eq!(vecgrid.num_rows(), 0);
    assert_eq!(vecgrid.num_columns(), 0);
    assert_eq!(vecgrid.row_len(), 0);
    assert_eq!(vecgrid.column_len(), 0);
    Ok(())
}

#[test]
fn test_empty_vecgrid_from_row_major() -> Result<(), Error> {
    let row_major: Vec<i32> = vec![];
    let vecgrid = Vecgrid::from_row_major(row_major, 0, 0)?;
    assert_eq!(vecgrid.num_rows(), 0);
    assert_eq!(vecgrid.num_columns(), 0);
    assert_eq!(vecgrid.row_len(), 0);
    assert_eq!(vecgrid.column_len(), 0);
    Ok(())
}

#[test]
fn test_empty_vecgrid_from_rows_many_empty_rows() -> Result<(), Error> {
    let rows: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    assert_eq!(vecgrid.num_rows(), 3);
    assert_eq!(vecgrid.num_columns(), 0);
    assert_eq!(vecgrid.row_len(), 0);
    assert_eq!(vecgrid.column_len(), 3);
    Ok(())
}

#[test]
fn test_empty_vecgrid_from_row_major_non_zero_columns() -> Result<(), Error> {
    let row_major: Vec<i32> = vec![];
    let vecgrid = Vecgrid::from_row_major(row_major, 0, 4)?;
    assert_eq!(vecgrid.num_rows(), 0);
    assert_eq!(vecgrid.num_columns(), 4);
    assert_eq!(vecgrid.row_len(), 4);
    assert_eq!(vecgrid.column_len(), 0);
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
// Double-Ended Iterators //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_double_ended_iterator_elements_row_major_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let reversed_columns = vecgrid
        .elements_row_major_iter()
        .cloned()
        .rev()
        .collect::<Vec<_>>();
    assert_eq!(reversed_columns, vec![6, 5, 4, 3, 2, 1]);
    Ok(())
}

#[test]
fn test_double_ended_iterator_elements_column_major_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let reversed_columns = vecgrid
        .elements_column_major_iter()
        .cloned()
        .rev()
        .collect::<Vec<_>>();
    assert_eq!(reversed_columns, vec![6, 3, 5, 2, 4, 1]);
    Ok(())
}

#[test]
fn test_double_ended_iterator_row_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let reversed_columns = vecgrid.row_iter(0)?.cloned().rev().collect::<Vec<_>>();
    assert_eq!(reversed_columns, vec![3, 2, 1]);
    Ok(())
}

#[test]
fn test_double_ended_iterator_column_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let reversed_columns = vecgrid.column_iter(1)?.cloned().rev().collect::<Vec<_>>();
    assert_eq!(reversed_columns, vec![5, 2]);
    Ok(())
}

#[test]
fn test_double_ended_iterator_rows_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let reversed_rows = vecgrid
        .rows_iter()
        .rev()
        .map(|row| row.cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert_eq!(reversed_rows, vec![vec![4, 5, 6], vec![1, 2, 3]]);
    Ok(())
}

#[test]
fn test_double_ended_iterator_columns_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let reversed_columns = vecgrid
        .columns_iter()
        .rev()
        .map(|row| row.cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert_eq!(reversed_columns, vec![vec![3, 6], vec![2, 5], vec![1, 4]]);
    Ok(())
}

#[test]
fn test_indices_row_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let indices_row_major = vecgrid.indices_row_major().collect::<Vec<_>>();
    assert_eq!(
        indices_row_major,
        vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
    );
    Ok(())
}

#[test]
fn test_indices_column_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let indices_column_major = vecgrid.indices_column_major().collect::<Vec<_>>();
    assert_eq!(
        indices_column_major,
        vec![(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)]
    );
    Ok(())
}

#[test]
fn test_enumerate_row_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let enumerate_row_major = vecgrid.enumerate_row_major().collect::<Vec<_>>();
    assert_eq!(
        enumerate_row_major,
        vec![
            ((0, 0), &1),
            ((0, 1), &2),
            ((0, 2), &3),
            ((1, 0), &4),
            ((1, 1), &5),
            ((1, 2), &6)
        ]
    );
    Ok(())
}

#[test]
fn test_insert_row() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![7, 8, 9]];
    let new_row = vec![4, 5, 6];
    let result = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut vecgrid = Vecgrid::from_rows(rows.clone())?;
    assert_eq!(vecgrid.num_rows(), 2);
    vecgrid.insert_row(new_row.clone(), 1)?;
    assert_eq!(vecgrid.as_rows(), result);
    assert_eq!(vecgrid.num_rows(), 3);

    let invalid_row = vec![10, 11];
    assert!(vecgrid.insert_row(invalid_row, 1).is_err());
    assert!(vecgrid.insert_row(new_row, 10).is_err());
    Ok(())
}

#[test]
fn test_insert_rows() -> Result<(), Error> {
    let rows = vec![vec![1, 2], vec![7, 8]];
    let new_rows = vec![vec![3, 4], vec![5, 6]];
    let result = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
    let mut vecgrid = Vecgrid::from_rows(rows)?;
    assert_eq!(vecgrid.num_rows(), 2);
    vecgrid.insert_rows(new_rows.clone(), 1)?;
    assert_eq!(vecgrid.as_rows(), result);
    assert_eq!(vecgrid.num_rows(), 4);

    let invalid_row = vec![9, 10, 11];
    let mut invalid_rows = new_rows.clone();
    invalid_rows.insert(2, invalid_row);
    assert!(vecgrid.insert_rows(invalid_rows, 1).is_err());
    assert!(vecgrid.insert_rows(new_rows, 10).is_err());
    Ok(())
}

#[test]
fn test_insert_column() -> Result<(), Error> {
    let columns = vec![vec![1, 2, 3], vec![7, 8, 9]];
    let new_column = vec![4, 5, 6];
    let result = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut vecgrid = Vecgrid::from_columns(columns.clone())?;
    assert_eq!(vecgrid.num_columns(), 2);
    vecgrid.insert_column(new_column.clone(), 1)?;
    assert_eq!(vecgrid.as_columns(), result);
    assert_eq!(vecgrid.num_columns(), 3);

    let invalid_column = vec![10, 11];
    assert!(vecgrid.insert_column(invalid_column, 1).is_err());
    assert!(vecgrid.insert_column(new_column, 10).is_err());
    Ok(())
}

#[test]
fn test_append_rows() -> Result<(), Error> {
    let rows = vec![vec![1, 2], vec![3, 4]];
    let new_rows = vec![vec![5, 6], vec![7, 8]];
    let result = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
    let mut vecgrid = Vecgrid::from_rows(rows)?;
    assert_eq!(vecgrid.num_rows(), 2);
    vecgrid.append_rows(new_rows.clone())?;
    assert_eq!(vecgrid.as_rows(), result);
    assert_eq!(vecgrid.num_rows(), 4);

    let invalid_row = vec![9, 10, 11];
    let mut invalid_rows = new_rows.clone();
    invalid_rows.insert(2, invalid_row);
    assert!(vecgrid.append_rows(invalid_rows).is_err());
    Ok(())
}

#[test]
fn test_remove_row() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let result = vec![vec![1, 2, 3], vec![7, 8, 9]];
    let mut vecgrid = Vecgrid::from_rows(rows)?;
    assert_eq!(vecgrid.num_rows(), 3);
    vecgrid.remove_row(1)?;
    assert_eq!(vecgrid.num_rows(), 2);
    assert_eq!(vecgrid.as_rows(), result);

    assert!(vecgrid.remove_row(3).is_err());
    Ok(())
}

#[test]
fn test_remove_rows() -> Result<(), Error> {
    let rows = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
    let result = vec![vec![1, 2], vec![7, 8]];
    let mut vecgrid = Vecgrid::from_rows(rows)?;
    assert_eq!(vecgrid.num_rows(), 4);
    vecgrid.remove_rows(1, 2)?;
    assert_eq!(vecgrid.num_rows(), 2);
    assert_eq!(vecgrid.as_rows(), result);

    assert!(vecgrid.remove_row(3).is_err());
    Ok(())
}

fn main() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let vecgrid = Vecgrid::from_rows(rows.clone())?;
    let enumerate_column_major = vecgrid.enumerate_column_major().collect::<Vec<_>>();
    assert_eq!(
        enumerate_column_major,
        vec![
            ((0, 0), &1),
            ((1, 0), &4),
            ((0, 1), &2),
            ((1, 1), &5),
            ((0, 2), &3),
            ((1, 2), &6)
        ]
    );
    Ok(())
}

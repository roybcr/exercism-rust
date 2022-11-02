// use std::{collections::HashSet, hash::Hash};

pub fn count(lines: &[&str]) -> u32 {
    // Instructions:

    // Count the rectangles in an ASCII diagram like the one below.

    //    +--+
    //   ++  |
    // +-++--+
    // |  |  |
    // +--+--+

    // The given lines are a slice of strings, each string is a line of the diagram.

    // let mut visited_cols: HashSet<usize> = HashSet::new();
    // let mut visited_rows: HashSet<usize> = HashSet::new();

    let mut rows: Vec<Vec<char>> = lines.iter()
                                        .map(|l| l.chars().collect())
                                        .collect();

    for (i, row) in rows.iter().enumerate() {
        let iter_col = row.iter().enumerate().into_iter();
        let range = vec![0..lines[i].len()];
        let iter_row = range.iter().enumerate().into_iter();

        let zip = iter_col.zip(iter_row);
        println!("ZIP{:#?}", zip);
    }

    let mut cols: Vec<Vec<char>> = Vec::new();

    let mut expanded: Vec<char> = lines.iter()
                                       .fold(vec![], |mut acc, curr| {
                                           acc.extend(curr.chars().into_iter());
                                           acc
                                       });

    println!("{:#?}", expanded);
    for r in 0..rows.len() {
        for c in 0..rows[r].len() {
            if cols.len() <= c {
                cols.push(Vec::new());
            }
            cols[c].push(rows[r][c]);
        }
    }

    println!("{:#?}", cols);
    println!("{:#?}", rows);

    // let vertical_iter = char_squares.iter().enumerate();

    // let get_char_at = |row: usize, col: usize| -> char { char_squares[row][col]
    // }; for (r, row) in vertical_iter {}

    0u32
    // at the begining of every iteration, check whether the row or column has
    // already been visited

    // Hints:
    // - You can use the `lines` iterator to iterate over the lines of the
    //   diagram.
    // - You can use the `chars` iterator to iterate over the characters of a
    //   line.
    // - You can use the `enumerate` iterator to iterate over the characters of
    //   a line and their index.
    // - You can use the `filter` iterator to filter out characters that are not
    //   part of a rectangle.
    // - You can use the `fold` iterator to count the number of rectangles.
    // - You can use the `any` iterator to check if a line contains a rectangle.
    // - You can use the `all` iterator to check if a line contains a rectangle.
    // - You can use the `zip` iterator to iterate over two iterators in
    //   parallel.
    // - You can use the `map` iterator to transform the characters of a line
    //   into a boolean value.
    // - You can use the `collect` iterator to collect the results of an
    //   iterator into a vector.
    // - You can use the `split` iterator to split a string into substrings
    //   based on a delimiter.
    // - You can use the `skip` iterator to skip the first `n` elements of an
    //   iterator.
    // - You can use the `take` iterator to take the first `n` elements of an
    //   iterator.
    // - You can use the `rev` iterator to iterate over the elements of an
    //   iterator in reverse order.
    // - You can use the `chain` iterator to chain two iterators together.
    // - You can use the `flat_map` iterator to flatten an iterator of
    //   iterators.
    // - You can use the `peekable` iterator to peek at the next element of an
    //   iterator.
    // - You can use the `skip_while` iterator to skip elements of an iterator
    //   while a predicate holds.
    // - You can use the `take_while` iterator to take elements of an iterator
    //   while a predicate holds.
    // - You can use the `max` iterator to find the maximum element of an
    //   iterator.
    // - You can use the `min` iterator to find the minimum element of an
    //   iterator.
    // - You can use the `sum` iterator to sum the elements of an iterator.
    // - You can use the `product` iterator to multiply the elements of an
    //   iterator.
    // - You can use the `min_by` iterator to find the minimum element of an
    //   iterator based on a comparison function.

    // Bonus points:
    // - You can use the `scan` iterator to count the number of rectangles.
    // - You can use the `partition` iterator to split an iterator into two
    //   iterators based on a predicate.
    // - You can use the `unzip` iterator to split an iterator of tuples into
    //   two iterators.
    // - You can use the `inspect` iterator to inspect the elements of an
    //   iterator.
    // - You can use the `by_ref` iterator to create a reference to an iterator.
    // - You can use the `cloned` iterator to clone the elements of an iterator.
    // - You can use the `cycle` iterator to repeat an iterator indefinitely.
    // - You can use the `fuse` iterator to create a fused iterator.
    // - You can use the `skip_last` iterator to skip the last `n` elements of
    //   an iterator.
    // - You can use the `take_last` iterator to take the last `n` elements of
    //   an iterator.
    // - You can use the `step_by` iterator to iterate over every `n`th element
    //   of an iterator.
    // - You can use the `stride` iterator to iterate over every `n`th element
    //   of an iterator.
    // - You can use the `windows` iterator to iterate over the elements of an
    //   iterator in windows of size `n`.
    // - You can use the `zip_longest` iterator to iterate over two iterators in
    //   parallel and return the longest iterator.
    // - You can use the `try_fold` iterator to count the number of rectangles.
    // - You can use the `try_for_each` iterator to count the number of
    //   rectangles.
    // - You can use the `find_map` iterator to find the first rectangle.
    // - You can use the `try_find` iterator to find the first rectangle.
    // - You can use the `try_find_map` iterator to find the first rectangle.
    // - You can use the `try_rfold` iterator to count the number of rectangles.
    // - You can use the `try_unzip` iterator to split an iterator of tuples
    //   into two iterators.
    // - You can use the `try_zip` iterator to iterate over two iterators in
    //   parallel and return the first error.
    // - You can use the `try_zip_longest` iterator to iterate over two
    //   iterators in parallel and return the longest iterator and the first
    //   error.
    // - You can use the `try_for_each_concurrent` iterator to count the number
    //   of rectangles.
    // - You can use the `try_fold_concurrent` iterator to count the number of
    //   rectangles.
    // - You can use the `try_rfold_concurrent` iterator to count the number of
    //   rectangles.
    // - You can use the `try_unzip_concurrent` iterator to split an iterator of
    //   tuples into two iterators.
}

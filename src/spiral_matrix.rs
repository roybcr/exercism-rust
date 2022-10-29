#[allow(dead_code)]
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut num = 1;
    let mut col = 0;
    let mut row = 0;
    let mut direction = 0;
    let mut row_start = 0;
    let mut col_start = 0;
    let mut row_end = size - 1;
    let mut col_end = size - 1;
    let mut matrix = vec![vec![0; size as usize]; size as usize];

    while num <= size.pow(2) {
        matrix[row][col] = num;
        num += 1;

        match direction {
            0 =>
                if col == col_end as usize {
                    row_start += 1;
                    row += 1;
                    direction = 1;
                } else {
                    col += 1;
                },
            1 =>
                if row == row_end as usize {
                    col_end -= 1;
                    col -= 1;
                    direction = 2;
                } else {
                    row += 1;
                },
            2 =>
                if col == col_start as usize {
                    row_end -= 1;
                    row -= 1;
                    direction = 3;
                } else {
                    col -= 1;
                },
            3 =>
                if row == row_start as usize {
                    col_start += 1;
                    col += 1;
                    direction = 0;
                } else {
                    row -= 1;
                },
            _ => unreachable!(),
        }
    }
    matrix
}

fn main() {
    let sudoku = [
        [1, 2, 0, 0, 3, 0, 0, 0, 0],
        [4, 0, 0, 5, 0, 0, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 0, 3],
        [5, 0, 0, 0, 6, 0, 0, 0, 4],
        [0, 0, 0, 8, 0, 3, 0, 0, 5],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 0, 0, 0, 0, 0, 2, 0, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 8],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    let valid = valid_sudoku(sudoku);
    println!("valid sudoku: {valid}");
}

fn valid_sudoku(sudoku: [[u8; 9]; 9]) -> bool {
    let mut row_bit_map = [0; 9];
    let mut col_bit_map = [0; 9];
    let mut cell_bit_map = [0; 9];
    for row in 0..9 {
        for col in 0..9 {
            let val = sudoku[row][col] as usize;
            if val == 0 {
                continue;
            }

            let cell = get_cell(row, col);
            // check if the value is already existing on the row
            // or the column or the cell
            if is_bit_set(row_bit_map[row], val)
                || is_bit_set(col_bit_map[col], val)
                || is_bit_set(cell_bit_map[cell], val)
            {
                return false;
            }
            row_bit_map[row] = set_bit(row_bit_map[row], val);
            col_bit_map[col] = set_bit(col_bit_map[col], val);
            cell_bit_map[cell] = set_bit(cell_bit_map[cell], val);
        }
    }
    true
}

#[inline]
fn get_cell(row: usize, col: usize) -> usize {
    (row / 3) * 3 + (col / 3)
}

#[inline]
fn set_bit(bit_map: u16, pos: usize) -> u16 {
    bit_map | (1 << pos)
}

#[inline]
fn is_bit_set(bit_map: u16, pos: usize) -> bool {
    bit_map & (1 << pos) != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0b0000_0000_0000_0000, 0, 0b0000_0000_0000_0001)]
    #[case(0b0000_0000_0000_0001, 1, 0b0000_0000_0000_0011)]
    #[case(0b0000_0000_0000_0011, 2, 0b0000_0000_0000_0111)]
    #[case(0b0000_0000_0000_0111, 3, 0b0000_0000_0000_1111)]
    #[case(0b0000_0000_0000_1111, 4, 0b0000_0000_0001_1111)]
    #[case(0b0000_0000_0001_1111, 5, 0b0000_0000_0011_1111)]
    #[case(0b0000_0000_0011_1111, 6, 0b0000_0000_0111_1111)]
    #[case(0b0000_0000_0111_1111, 7, 0b0000_0000_1111_1111)]
    #[case(0b0000_0000_1111_1111, 8, 0b0000_0001_1111_1111)]
    fn test_set_bit(#[case] bit_map: u16, #[case] pos: usize, #[case] expected: u16) {
        let got = set_bit(bit_map, pos);
        assert_eq!(
            expected, got,
            "expected: {expected:09b}, but got: {got:09b}"
        );
    }

    #[rstest]
    #[case(0b0000_0000_0000_0001, 0, true)]
    #[case(0b0000_0000_0000_0010, 1, true)]
    #[case(0b0000_0000_0000_0100, 2, true)]
    #[case(0b0000_0000_0000_1000, 3, true)]
    #[case(0b0000_0000_0001_0000, 4, true)]
    #[case(0b0000_0000_0010_0000, 5, true)]
    #[case(0b0000_0000_0100_0000, 6, true)]
    #[case(0b0000_0000_1000_0000, 7, true)]
    #[case(0b0000_0001_0000_0000, 8, true)]
    #[case(0b0000_0000_0000_0000, 0, false)]
    #[case(0b0000_0000_0000_0000, 1, false)]
    #[case(0b0000_0000_0000_0000, 2, false)]
    #[case(0b0000_0000_0000_0000, 3, false)]
    #[case(0b0000_0000_0000_0000, 4, false)]
    #[case(0b0000_0000_0000_0000, 5, false)]
    #[case(0b0000_0000_0000_0000, 6, false)]
    #[case(0b0000_0000_0000_0000, 7, false)]
    #[case(0b0000_0000_0000_0000, 8, false)]
    #[case(0b0000_0000_0000_0001, 1, false)]
    #[case(0b0000_0000_0000_0010, 0, false)]
    #[case(0b0000_0000_0000_0010, 2, false)]
    #[case(0b0000_0001_0000_0000, 0, false)]
    #[case(0b0000_0001_0000_0000, 7, false)]
    #[case(0b0000_0001_1111_1111, 0, true)]
    #[case(0b0000_0001_1111_1111, 4, true)]
    #[case(0b0000_0001_1111_1111, 8, true)]
    #[case(0b0000_0001_0101_0101, 0, true)]
    #[case(0b0000_0001_0101_0101, 1, false)]
    #[case(0b0000_0001_0101_0101, 2, true)]
    #[case(0b0000_0001_0101_0101, 3, false)]
    #[case(0b0000_0001_0101_0101, 4, true)]
    #[case(0b0000_0001_0101_0101, 5, false)]
    #[case(0b0000_0001_0101_0101, 6, true)]
    #[case(0b0000_0001_0101_0101, 7, false)]
    #[case(0b0000_0001_0101_0101, 8, true)]
    fn test_is_bit_set(#[case] bit_map: u16, #[case] pos: usize, #[case] expected: bool) {
        let got = is_bit_set(bit_map, pos);
        assert_eq!(expected, got);
    }

    #[rstest]
    // row 0
    #[case(0, 0, 0)]
    #[case(0, 1, 0)]
    #[case(0, 2, 0)]
    #[case(0, 3, 1)]
    #[case(0, 4, 1)]
    #[case(0, 5, 1)]
    #[case(0, 6, 2)]
    #[case(0, 7, 2)]
    #[case(0, 8, 2)]
    // row 1
    #[case(1, 0, 0)]
    #[case(1, 1, 0)]
    #[case(1, 2, 0)]
    #[case(1, 3, 1)]
    #[case(1, 4, 1)]
    #[case(1, 5, 1)]
    #[case(1, 6, 2)]
    #[case(1, 7, 2)]
    #[case(1, 8, 2)]
    // row 2
    #[case(2, 0, 0)]
    #[case(2, 1, 0)]
    #[case(2, 2, 0)]
    #[case(2, 3, 1)]
    #[case(2, 4, 1)]
    #[case(2, 5, 1)]
    #[case(2, 6, 2)]
    #[case(2, 7, 2)]
    #[case(2, 8, 2)]
    // row 3
    #[case(3, 0, 3)]
    #[case(3, 1, 3)]
    #[case(3, 2, 3)]
    #[case(3, 3, 4)]
    #[case(3, 4, 4)]
    #[case(3, 5, 4)]
    #[case(3, 6, 5)]
    #[case(3, 7, 5)]
    #[case(3, 8, 5)]
    // row 4
    #[case(4, 0, 3)]
    #[case(4, 1, 3)]
    #[case(4, 2, 3)]
    #[case(4, 3, 4)]
    #[case(4, 4, 4)]
    #[case(4, 5, 4)]
    #[case(4, 6, 5)]
    #[case(4, 7, 5)]
    #[case(4, 8, 5)]
    // row 5
    #[case(5, 0, 3)]
    #[case(5, 1, 3)]
    #[case(5, 2, 3)]
    #[case(5, 3, 4)]
    #[case(5, 4, 4)]
    #[case(5, 5, 4)]
    #[case(5, 6, 5)]
    #[case(5, 7, 5)]
    #[case(5, 8, 5)]
    // row 6
    #[case(6, 0, 6)]
    #[case(6, 1, 6)]
    #[case(6, 2, 6)]
    #[case(6, 3, 7)]
    #[case(6, 4, 7)]
    #[case(6, 5, 7)]
    #[case(6, 6, 8)]
    #[case(6, 7, 8)]
    #[case(6, 8, 8)]
    // row 7
    #[case(7, 0, 6)]
    #[case(7, 1, 6)]
    #[case(7, 2, 6)]
    #[case(7, 3, 7)]
    #[case(7, 4, 7)]
    #[case(7, 5, 7)]
    #[case(7, 6, 8)]
    #[case(7, 7, 8)]
    #[case(7, 8, 8)]
    // row 8
    #[case(8, 0, 6)]
    #[case(8, 1, 6)]
    #[case(8, 2, 6)]
    #[case(8, 3, 7)]
    #[case(8, 4, 7)]
    #[case(8, 5, 7)]
    #[case(8, 6, 8)]
    #[case(8, 7, 8)]
    #[case(8, 8, 8)]
    fn test_get_cell(#[case] row: usize, #[case] col: usize, #[case] expected: usize) {
        let got = get_cell(row, col);
        assert_eq!(expected, got);
    }

    #[rstest]
    #[case(
        [[1,2,0,0,3,0,0,0,0],
         [4,0,0,5,0,0,0,0,0],
         [0,9,8,0,0,0,0,0,3],
         [5,0,0,0,6,0,0,0,4],
         [0,0,0,8,0,3,0,0,5],
         [7,0,0,0,2,0,0,0,6],
         [0,0,0,0,0,0,2,0,0],
         [0,0,0,4,1,9,0,0,8],
         [0,0,0,0,8,0,0,7,9]],
        true
    )]
    #[case(
        [[1,2,0,0,3,0,0,0,0],
         [4,0,0,5,0,0,0,0,0],
         [0,9,1,0,0,0,0,0,3],
         [5,0,0,0,6,0,0,0,4],
         [0,0,0,8,0,3,0,0,5],
         [7,0,0,0,2,0,0,0,6],
         [0,0,0,0,0,0,2,0,0],
         [0,0,0,4,1,9,0,0,8],
         [0,0,0,0,8,0,0,7,9]],
        false
    )]
    fn test_valid_sudoku(#[case] sudoku: [[u8; 9]; 9], #[case] expected: bool) {
        let got = valid_sudoku(sudoku);
        assert_eq!(expected, got);
    }
}

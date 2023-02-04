//! Conversion functions between a digit 1..=9 and a (row, col) pair.
//!
//! This mapping emulates the layout of a keyboard "numpad":
//! ```
//! 789
//! 456
//! 123
//! ```

pub fn to_row_col(digit: u8) -> Option<(usize, usize)> {
    if !(1..=9).contains(&digit) {
        return None;
    }

    let zero_indexed = digit - 1;

    let row_bottom_up = zero_indexed / 3;
    let col = zero_indexed % 3;

    let row_top_down = 2 - row_bottom_up;

    Some((row_top_down as usize, col as usize))
}

pub fn from_row_col(row: usize, col: usize) -> Option<u8> {
    if row >= 3 || col >= 3 {
        return None;
    }

    let row_top_down = row;
    let row_bottom_up = 2 - row_top_down;

    let zero_indexed = row_bottom_up * 3 + col;
    debug_assert!(zero_indexed < 9);

    Some(zero_indexed as u8 + 1)
}

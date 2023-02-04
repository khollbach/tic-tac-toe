use super::{Board, DisplayPlayableDigits, Player};
use crate::digit;
use std::fmt::{self, Display, Formatter};

impl Player {
    fn to_char(self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Board {
    /// Helper function to implement Display for Board and DisplayPlayableDigits.
    fn display(
        &self,
        f: &mut Formatter<'_>,
        mut cell_to_char: impl FnMut(usize, usize, Option<Player>) -> char,
    ) -> fmt::Result {
        for row in 0..3 {
            if row != 0 {
                // Note that the final row of output won't have a line-terminator.
                write!(f, "\n-+-+-\n")?;
            }

            for col in 0..3 {
                if col != 0 {
                    write!(f, "|")?;
                }

                let char_ = cell_to_char(row, col, self.cells[row][col]);
                write!(f, "{char_}")?;
            }
        }

        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.display(f, |_, _, cell| {
            if let Some(player) = cell {
                player.to_char()
            } else {
                ' '
            }
        })
    }
}

impl Display for DisplayPlayableDigits {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.board.display(f, |row, col, cell| {
            if cell.is_some() {
                ' '
            } else {
                let offset = digit::from_row_col(row, col).unwrap();
                (b'0' + offset) as char
            }
        })
    }
}

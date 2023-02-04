mod display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

pub const PLAYERS: [Player; 2] = [Player::X, Player::O];

#[derive(Debug, Clone)]
pub struct Board {
    cells: [[Option<Player>; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[None; 3]; 3],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Player> {
        assert!(row < 3 && col < 3);

        self.cells[row][col]
    }

    /// Return the previous contents of that cell, if any.
    pub fn set(&mut self, row: usize, col: usize, player: Player) -> Option<Player> {
        assert!(row < 3 && col < 3);

        self.cells[row][col].replace(player)
    }

    pub fn game_outcome(&self) -> Option<GameOver> {
        for player in PLAYERS {
            let row_win = (0..3).any(|row| (0..3).all(|col| self.get(row, col) == Some(player)));
            let col_win = (0..3).any(|col| (0..3).all(|row| self.get(row, col) == Some(player)));
            let main_diag = (0..3).all(|i| self.get(i, i) == Some(player));
            let cross_diag = (0..3).all(|i| self.get(i, 2 - i) == Some(player));

            if row_win || col_win || main_diag || cross_diag {
                return Some(GameOver::Winner(player));
            }
        }

        if (0..3).all(|row| (0..3).all(|col| self.get(row, col).is_some())) {
            return Some(GameOver::Tie);
        }

        None
    }

    /// Returns an object that implements `Display`. When printed, it will show
    /// digit "hints" in the empty squares, to make it easier to see which legal
    /// moves are left.
    ///
    /// ```
    /// let mut  board = Board::new();
    /// board.set(1, 1, Player::X);
    ///
    /// println!("{}", board.display_playable_digits());
    ///
    /// let expected = "\
    /// 7|8|9
    /// -+-+-
    /// 4| |6
    /// -+-+-
    /// 1|2|3";
    /// assert_eq!(expected, board.display_playable_digits().to_string());
    /// ```
    pub fn display_playable_digits(&self) -> DisplayPlayableDigits {
        DisplayPlayableDigits {
            board: self.clone(),
        }
    }
}

pub enum GameOver {
    Winner(Player),
    Tie,
}

/// Wrapper around a `Board` that implements `Display` differently.
///
/// See [`Board::display_playable_digits`].
pub struct DisplayPlayableDigits {
    board: Board,
}

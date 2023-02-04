mod board;
pub mod digit;

use crate::board::{Board, GameOver, PLAYERS};
use anyhow::Result;
use std::{
    io::{self, Write},
    iter,
};

fn main() -> Result<()> {
    let mut board = Board::new();

    println!("Tic Tac Toe");
    println!("===========");
    println!();
    println!("Squares are numbered 1 through 9.");
    println!("To make a move, type one of these digits, then press enter.");
    println!();
    println!("{}", board.display_playable_digits());
    println!();

    for player in iter::repeat(PLAYERS).flatten() {
        print!("{player}'s turn. Make your move: ");
        io::stdout().flush()?;

        let (row, col) = read_player_move(&board)?;

        let prev = board.set(row, col, player);
        assert!(prev.is_none());

        println!();
        println!("{board}");
        println!();

        match board.game_outcome() {
            Some(GameOver::Winner(player)) => {
                println!("{player} wins!");
                break;
            }
            Some(GameOver::Tie) => {
                println!("It's a tie.");
                break;
            }
            None => continue,
        }
    }

    Ok(())
}

/// Retry until the player enters a valid, legal move.
fn read_player_move(board: &Board) -> Result<(usize, usize)> {
    assert!(board.game_outcome().is_none());

    let mut verbose_help_printed = false;

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            // No need to print a new prompt here.
            continue;
        }

        let is_digit = input.len() == 1 && "0123456789".contains(input);
        if !is_digit {
            print!("That's not a digit! Try again: ");
            io::stdout().flush()?;
            continue;
        }

        let digit = input.chars().next().expect("len was 1");
        let digit = digit as u8 - b'0';

        if digit == 0 {
            print!("Squares are numbered 1 through 9. Try again: ");
            io::stdout().flush()?;
            continue;
        }

        let (row, col) = digit::to_row_col(digit).unwrap();

        if let Some(player) = board.get(row, col) {
            if !verbose_help_printed {
                println!("Square {digit} already has an {player}.");
                println!("Try one of these:");
                println!();
                println!("{}", board.display_playable_digits());
                println!();
                verbose_help_printed = true;
            }
            print!("Enter one of the above digits: ");
            io::stdout().flush()?;
            continue;
        }

        return Ok((row, col));
    }
}

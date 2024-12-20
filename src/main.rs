#![allow(non_snake_case)]
use std::io;

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
type Board = [[u8; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
enum Player {
    One = 1,
    Two = 2,
    None = 0,
}

impl Player {
    fn FromInt(value: u8) -> Player {
        match value {
            1 => Player::One,
            2 => Player::Two,
            _ => Player::None,
        }
    }
}

#[derive(Debug)]
enum MoveError {
    GameFinished,
    InvalidColumn,
    ColumnFull,
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveError::GameFinished => write!(f, "Game is already finished"),
            MoveError::InvalidColumn => write!(f, "Invalid column"),
            MoveError::ColumnFull => write!(f, "Column is full"),
        }
    }
}

struct Game {
    CurrentMove: u8,
    CurrentPlayer: Player,
    Board: Board,
    IsFinished: bool,
    Winner: Player,
}

impl Game {
    fn new() -> Game {
        Game {
            CurrentMove: 0,
            CurrentPlayer: Player::One,
            Board: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
            IsFinished: false,
            Winner: Player::None,
        }
    }

    // fn ClearScreen() {
    //     print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // }
    fn DisplayBoard(&self) {
        //Self::ClearScreen();
        println!("{}--------------------{}", YELLOW, RESET);
        println!("{}Connect 4  (Move: {}){}", YELLOW, self.CurrentMove, RESET);
        println!("{}--------------------{}", YELLOW, RESET);
        for row in self.Board {
            let RowStr: String = row
                .iter()
                .map(|cell| match Player::FromInt(*cell) {
                    Player::One => "🔴".to_string(),
                    Player::Two => "🟡".to_string(),
                    Player::None => "🔵".to_string(),
                })
                .collect::<Vec<String>>()
                .join(" ");
            println!("{}", RowStr);
        }
        println!("{}--------------------{}", YELLOW, RESET);
        if self.IsFinished {
            match self.Winner {
                Player::One => println!("{}🔴 Player One Wins!{}", YELLOW, RESET),
                Player::Two => println!("{}🟡 Player Two Wins!{}", YELLOW, RESET),
                Player::None => println!("{}It's a Draw!{}", YELLOW, RESET),
            }
        }
        println!("{}--------------------{}", YELLOW, RESET);
    }

    fn DisplayError(&self, error: String) {
        self.DisplayBoard();
        println!("{}Error: {}{}", RED, error, RESET);
    }

    fn FindWinner(&mut self) -> Player {
        if self.CurrentMove < 7 {
            return Player::None;
        }

        let directions = [(0, 1), (1, 0), (1, 1), (-1, 1)];
        for row in 0..BOARD_HEIGHT {
            for column in 0..BOARD_WIDTH {
                let cell = self.Board[row][column];
                if cell != 0 {
                    for &(RowStep, ColStep) in &directions {
                        let mut count = 1;
                        let mut r = row as isize + RowStep;
                        let mut c = column as isize + ColStep;

                        while r >= 0
                            && r < BOARD_HEIGHT as isize
                            && c >= 0
                            && c < BOARD_WIDTH as isize
                            && self.Board[r as usize][c as usize] == cell
                        {
                            count += 1;
                            if count == 4 {
                                return Player::FromInt(cell);
                            }
                            r += RowStep;
                            c += ColStep;
                        }
                    }
                }
            }
        }

        if self.CurrentMove as usize == BOARD_WIDTH * BOARD_HEIGHT {
            self.IsFinished = true;
        }

        Player::None
    }

    fn MakeMove(&mut self, column: usize) -> Result<(), MoveError> {
        if self.IsFinished {
            return Err(MoveError::GameFinished);
        }

        if column >= BOARD_WIDTH {
            return Err(MoveError::InvalidColumn);
        }
        if let Some(row) = (0..BOARD_HEIGHT).rev().find(|&row| self.Board[row][column] == 0) {
            self.Board[row][column] = self.CurrentPlayer as u8;
            self.CurrentMove += 1;
        } else {
            return Err(MoveError::ColumnFull);
        };

        let FoundWinner = self.FindWinner();
        if FoundWinner != Player::None {
            self.Winner = FoundWinner;
            self.IsFinished = true;
        } else {
            self.CurrentPlayer = match self.CurrentPlayer {
                Player::One => Player::Two,
                _ => Player::One,
            };
        }
        Ok(())
    }
}

fn main() {
    let mut game = Game::new();
    game.DisplayBoard();
  loop {
    while !game.IsFinished {
        println!("\n");
        match game.CurrentPlayer {
            Player::One => println!("Player 1"),
            Player::Two => println!("Player 2"),
            _ => (),
        }
        println!("Enter a column number (1-7): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: usize = match input.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 7 {
                    game.DisplayError("Invalid column number".to_string());
                    continue;
                } else {
                    num
                }
            }
            Err(err) => {
                game.DisplayError(err.to_string());
                continue;
            }
        };
        match game.MakeMove(input - 1) {
            Ok(_) => game.DisplayBoard(),
            Err(err) => game.DisplayError(err.to_string()),
        }
    }
    println!("Do you want to play again? (y/n)");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().to_lowercase().as_str() {
        "y" => {
            game = Game::new();
            game.DisplayBoard();
        }
        "n" => break,
        _ => println!("Invalid input"),
    }
  }
}

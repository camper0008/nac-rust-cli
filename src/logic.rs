use crate::io::{clear_screen, display_board, display_winner, parse_stdin, Input};
use crate::utils::blank_board;

#[derive(Debug, PartialEq, Clone)]
pub enum Piece {
    Blank,
    O,
    X,
}

pub type Board = [[Piece; 3]; 3];

impl Piece {
    pub fn display(&self) -> String {
        match self {
            Piece::Blank => ".".to_string(),
            Piece::O => "o".to_string(),
            Piece::X => "x".to_string(),
        }
    }
    pub fn opposite(&self) -> Self {
        match self {
            Piece::Blank => Self::Blank,
            Piece::O => Self::X,
            Piece::X => Self::O,
        }
    }
}

fn rows_winner(board: &Board) -> Option<Piece> {
    let mut winner = None;
    (0..=2).into_iter().for_each(|row| {
        if board[row][0] == board[row][1]
            && board[row][0] == board[row][2]
            && board[row][0] != Piece::Blank
        {
            winner = Some(board[row][0].clone());
        }
    });
    winner
}

fn columns_winner(board: &Board) -> Option<Piece> {
    let mut winner = None;
    (0..=2).into_iter().for_each(|col| {
        if board[0][col] == board[1][col]
            && board[0][col] == board[2][col]
            && board[0][col] != Piece::Blank
        {
            winner = Some(board[0][col].clone());
        }
    });
    winner
}

fn diagonals_winner(board: &Board) -> Option<Piece> {
    let mut winner = None;
    if board[0][0] == board[1][1] && board[0][0] == board[2][2] && board[1][1] != Piece::Blank {
        winner = Some(board[1][1].clone())
    }
    if board[2][0] == board[1][1] && board[2][0] == board[0][2] && board[1][1] != Piece::Blank {
        winner = Some(board[1][1].clone())
    }
    winner
}

pub fn game_winner(board: &Board) -> Option<Piece> {
    let rows_winner_option = rows_winner(board);
    if rows_winner_option.is_some() {
        return rows_winner_option;
    }

    let columns_winner_option = columns_winner(board);
    if columns_winner_option.is_some() {
        return columns_winner_option;
    }

    let diagonals_winner_option = diagonals_winner(board);
    if diagonals_winner_option.is_some() {
        return diagonals_winner_option;
    }

    None
}

fn place_if_blank(board: &mut Board, input: Input, taker: Piece) -> Result<(), String> {
    let row = input.row;
    let column = input.column;

    if board[row][column] != Piece::Blank {
        return Err("Column {column}, row {row} is already taken.".to_string());
    }

    board[row][column] = taker;

    Ok(())
}

pub fn game_loop() {
    let mut board: Board = blank_board();
    let mut turn = Piece::O;
    loop {
        println!("Current turn: {:?}", turn);
        display_board(&board);

        let input_res = parse_stdin();
        if let Err(err) = input_res.as_ref() {
            clear_screen();
            println!("{:?}", err);
            continue;
        }
        let input = input_res.unwrap();

        if let Err(err) = place_if_blank(&mut board, input, turn.clone()) {
            clear_screen();
            println!("{}", err);
            continue;
        }

        let winner_option = game_winner(&board);
        if let Some(winner) = winner_option {
            display_winner(&board, winner);
            break;
        }

        turn = turn.opposite();
        clear_screen();
    }
}

use crate::logic::Piece;
use std::io;

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

pub fn parse_stdin() -> Result<Input, InputError> {
    const ZERO_BYTE: u8 = b'0';
    const TWO_BYTE: u8 = b'2';

    let read = read_stdin().unwrap();
    let mut read_bytes = read.bytes();
    let column_byte = read_bytes.next().ok_or(InputError::InvalidColumn)?;
    let row_byte = read_bytes.next().ok_or(InputError::InvalidRow)?;

    if !(ZERO_BYTE..=TWO_BYTE).contains(&column_byte) {
        return Err(InputError::InvalidColumn);
    }

    if !(ZERO_BYTE..=TWO_BYTE).contains(&row_byte) {
        return Err(InputError::InvalidRow);
    }

    let row = (row_byte - ZERO_BYTE) as usize;
    let column = (column_byte - ZERO_BYTE) as usize;

    Ok(Input { row, column })
}

pub struct Input {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum InputError {
    InvalidColumn,
    InvalidRow,
}

pub fn display_board(board: &[[Piece; 3]; 3]) {
    println!(" 0 1 2");
    board.iter().enumerate().for_each(|(idx, row)| {
        row.iter().for_each(|col| print!("|{}", col.display()));
        println!("| {}", idx);
    })
}

pub fn display_winner(board: &[[Piece; 3]; 3], winner: Piece) {
    clear_screen();
    println!("Winner: {:?}", winner);
    display_board(board);
}

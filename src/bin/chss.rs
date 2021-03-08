use chsslib::*;

fn main() {
    let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let board = Board::parse(&start);
    match board {
        Ok(board) => print!("{:?}", board),
        Err(error) => println!("{}", error),
    }
}

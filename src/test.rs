#[cfg(test)]
use crate::{PlayerOrComputer::*, *};
mod tests {
    #[cfg(test)]
    use super::*;

    mod take_one {
        #[cfg(test)]
        use super::*;
        #[test]
        fn take_one_white_stone_row() {
            let mut board = Board::new();
            board.make_move(3, 2, Tile::Black);
            println!("{board}");
            let mut control_board = Board::new();
            control_board.board[pair_to_index(3, 2) as usize] = Tile::Black;
            control_board.board[pair_to_index(3, 3) as usize] = Tile::Black;
            control_board.black_count += 2;
            control_board.white_count -= 1;
            println!("{control_board}");

            assert_eq!(board, control_board);
        }
        #[test]
        fn take_one_black_stone_row() {
            let mut board = Board::new();
            board.make_move(4, 2, Tile::White);
            println!("{board}");
            let mut control_board = Board::new();
            control_board.board[pair_to_index(4, 2) as usize] = Tile::White;
            control_board.board[pair_to_index(4, 3) as usize] = Tile::White;
            control_board.black_count -= 1;
            control_board.white_count += 2;
            println!("{control_board}");

            assert_eq!(board, control_board);
        }
    }
    #[test]
    fn take_two_black_stones_in_a_row() {
        let mut board = Board::new();
        board.board[pair_to_index(4, 2) as usize] = Tile::Black;
        board.black_count += 1;
        board.make_move(4, 1, Tile::White);
        println!("{board}");
        let mut control_board = Board::new();
        control_board.board[pair_to_index(4, 1) as usize] = Tile::White;
        control_board.board[pair_to_index(4, 2) as usize] = Tile::White;
        control_board.board[pair_to_index(4, 3) as usize] = Tile::White;
        control_board.black_count -= 1;
        control_board.white_count += 3;
        println!("{control_board}");

        assert_eq!(board, control_board);
    }

    #[test]
    fn skip_turn_when_no_black_moves_possible() {
        let mut board = Board::new();
        board.board[pair_to_index(3, 3) as usize] =
            Tile::Empty(DrawOrEmpty::Empty);
        board.board[pair_to_index(3, 4) as usize] =
            Tile::Empty(DrawOrEmpty::Empty);
        board.board[pair_to_index(4, 3) as usize] =
            Tile::Empty(DrawOrEmpty::Empty);
        board.board[pair_to_index(4, 4) as usize] =
            Tile::Empty(DrawOrEmpty::Empty);
        board.black_count = 1;
        board.white_count = 1;
        board.board[pair_to_index(0, 0) as usize] = Tile::White;
        board.board[pair_to_index(1, 0) as usize] = Tile::Black;
        board.turn = Tile::Black;

        let mut control_board = board.clone();
        board.colored_move(
            Message::EmptyPressed(2, 0),
            Player,
            Player,
            Tile::Black,
        );
        println!("{board}");
        control_board.black_count = 0;
        control_board.white_count = 3;
        control_board.turn = Tile::White;
        control_board.board[pair_to_index(1, 0) as usize] = Tile::White;
        control_board.board[pair_to_index(2, 0) as usize] = Tile::White;
        control_board.win = Tile::White;
        println!("{control_board}");
        assert_eq!(board, control_board)
    }
}

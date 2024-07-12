#[cfg(test)]
use crate::{PlayerOrComputer::*, *};
mod tests {
    #[cfg(test)]
    use super::*;

    mod take_one {
        #[cfg(test)]
        use super::*;
        #[test]
        fn white_stone_row() {
            let mut board = Board::new();
            board.make_move(3, 2, StoneColor::Black);
            println!("{board}");

            let mut control_board = Board::new();
            control_board.board[pair_to_index(3, 2)].0 = Some(StoneColor::Black);
            control_board.board[pair_to_index(3, 3)].0 = Some(StoneColor::Black);
            
            control_board.black_count += 2;
            control_board.white_count -= 1;

            control_board.next_to_taken = [false; WIDTH * HEIGHT];
            [
                2 * WIDTH + 1,
                2 * WIDTH + 2,
                2 * WIDTH + 3,
                2 * WIDTH + 4,
                2 * WIDTH + 5,
                3 * WIDTH + 1,
                3 * WIDTH + 5,
                4 * WIDTH + 1,
                4 * WIDTH + 2,
                4 * WIDTH + 5,
                5 * WIDTH + 2,
                5 * WIDTH + 3,
                5 * WIDTH + 4,
                5 * WIDTH + 5,
            ]
            .iter()
            .for_each(|&id| control_board.next_to_taken[id] = true);

            println!("{control_board}");

            assert_eq!(board.board, control_board.board);
            assert_eq!(board.white_count, control_board.white_count);
            assert_eq!(board.black_count, control_board.black_count);
            assert_eq!(board.turn, control_board.turn);
            assert_eq!(board.win, control_board.win);
            assert_eq!(board.next_to_taken, control_board.next_to_taken);
        }
        #[test]
        fn black_stone_row() {
            let mut board = Board::new();
            board.make_move(4, 2, StoneColor::White);
            println!("{board}");
            let mut control_board = Board::new();
            control_board.board[pair_to_index(4, 2) as usize].0 = Some(StoneColor::White);
            control_board.board[pair_to_index(4, 3) as usize].0 = Some(StoneColor::White);
            control_board.black_count -= 1;
            control_board.white_count += 2;

            control_board.next_to_taken = [false; WIDTH * HEIGHT];
            [
                2 * WIDTH + 2,
                2 * WIDTH + 3,
                2 * WIDTH + 4,
                2 * WIDTH + 5,
                3 * WIDTH + 1,
                3 * WIDTH + 2,
                3 * WIDTH + 5,
                4 * WIDTH + 1,
                4 * WIDTH + 5,
                5 * WIDTH + 1,
                5 * WIDTH + 2,
                5 * WIDTH + 3,
                5 * WIDTH + 4,
                5 * WIDTH + 5,
            ]
            .iter()
            .for_each(|&id| control_board.next_to_taken[id] = true);

            println!("{control_board}");

            assert_eq!(board.board, control_board.board);
            assert_eq!(board.white_count, control_board.white_count);
            assert_eq!(board.black_count, control_board.black_count);
            assert_eq!(board.turn, control_board.turn);
            assert_eq!(board.win, control_board.win);
            assert_eq!(board.next_to_taken, control_board.next_to_taken);
        }
    }
    #[test]
    fn take_two_black_stones_in_a_row() {
        let mut board = Board::new();
        board.board[pair_to_index(4, 2) as usize].0 = Some(StoneColor::Black);
        board.black_count += 1;

        board.next_to_taken[4 * WIDTH + 2] = false;
        [3 * WIDTH + 1, 4 * WIDTH + 1, 5 * WIDTH + 1]
            .iter()
            .for_each(|&id| board.next_to_taken[id] = true);

        board.make_move(4, 1, StoneColor::White);
        println!("{board}");

        let mut control_board = Board::new();
        control_board.board[pair_to_index(4, 1) as usize].0 = Some(StoneColor::White);
        control_board.board[pair_to_index(4, 2) as usize].0 = Some(StoneColor::White);
        control_board.board[pair_to_index(4, 3) as usize].0 = Some(StoneColor::White);

        control_board.black_count -= 1;
        control_board.white_count += 3;

        control_board.next_to_taken = [false; WIDTH * HEIGHT];
        [
            2 * WIDTH + 2,
            2 * WIDTH + 3,
            2 * WIDTH + 4,
            2 * WIDTH + 5,
            3 * WIDTH,
            3 * WIDTH + 1,
            3 * WIDTH + 2,
            3 * WIDTH + 5,
            4 * WIDTH,
            4 * WIDTH + 5,
            5 * WIDTH,
            5 * WIDTH + 1,
            5 * WIDTH + 2,
            5 * WIDTH + 3,
            5 * WIDTH + 4,
            5 * WIDTH + 5,
        ]
        .iter()
        .for_each(|&id| control_board.next_to_taken[id] = true);

        println!("{control_board}");

        assert_eq!(board.board, control_board.board);
        assert_eq!(board.white_count, control_board.white_count);
        assert_eq!(board.black_count, control_board.black_count);
        assert_eq!(board.turn, control_board.turn);
        assert_eq!(board.win, control_board.win);
        assert_eq!(board.next_to_taken, control_board.next_to_taken);
    }

    #[test]
    fn skip_turn_when_no_black_moves_possible() {
        let mut board = Board::new();
        board.board[pair_to_index(3, 3) as usize].0 = None;
        board.board[pair_to_index(3, 4) as usize].0 = None;
        board.board[pair_to_index(4, 3) as usize].0 = None;
        board.board[pair_to_index(4, 4) as usize].0 = None;

        board.board[pair_to_index(0, 0) as usize].0 = Some(StoneColor::White);
        board.board[pair_to_index(1, 0) as usize].0 = Some(StoneColor::Black);

        board.white_count = 1;
        board.black_count = 1;

        board.next_to_taken = [false; WIDTH * HEIGHT];
        [1, WIDTH + 1, 2 * WIDTH, 2 * WIDTH + 1]
            .iter()
            .for_each(|&id| board.next_to_taken[id] = true);
        board.turn = StoneColor::Black;

        let mut control_board = board.clone();
        board.colored_move(
            Message::EmptyPressed(2, 0),
            Player,
            Player,
            StoneColor::Black,
        );
        println!("{board}");
        control_board.black_count = 0;
        control_board.white_count = 3;
        control_board.turn = StoneColor::White;
        control_board.board[pair_to_index(1, 0)].0 = Some(StoneColor::White);
        control_board.board[pair_to_index(2, 0)].0 = Some(StoneColor::White);
        control_board.win = GameOutcome::Win(StoneColor::White);

        control_board.next_to_taken = [false; WIDTH * HEIGHT];
        [1, WIDTH + 1, 2 * WIDTH + 1, 3 * WIDTH, 3 * WIDTH + 1]
            .iter()
            .for_each(|&id| control_board.next_to_taken[id] = true);

        println!("{control_board}");
        assert_eq!(board.board, control_board.board);
        assert_eq!(board.white_count, control_board.white_count);
        assert_eq!(board.black_count, control_board.black_count);
        assert_eq!(board.turn, control_board.turn);
        assert_eq!(board.win, control_board.win);
        assert_eq!(board.next_to_taken, control_board.next_to_taken);
    }
}

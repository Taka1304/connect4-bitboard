use std::io::{self, Write};

const ROWS: usize = 6;
const COLS: usize = 7;

// ボードの状態を保持するための構造体
struct BitBoard {
    player1: u64, // プレイヤー1のビットボード
    player2: u64, // プレイヤー2のビットボード
}

struct Game {
    board: BitBoard,
}

impl Game {
    // 列が埋まっているかを確認する関数
    fn is_column_full(&self, col: usize) -> bool {
        let combined_board = self.board.player1 | self.board.player2;
        combined_board & (1 << (col * ROWS + (ROWS - 1))) != 0
    }

    // ビットボードの値を更新する関数
    fn drop_disc(&mut self, col: usize, is_player1: bool) -> Result<(), &'static str> {
        if self.is_column_full(col) {
            return Err("Column is full");
        }

        let combined_board = self.board.player1 | self.board.player2;
        let mut bit_position = col * ROWS;
        while combined_board & (1 << bit_position) != 0 {
            bit_position += 1;
        }

        if is_player1 {
            self.board.player1 |= 1 << bit_position;
        } else {
            self.board.player2 |= 1 << bit_position;
        }

        Ok(())
    }

    // ゲームが終了したかを判定する関数
    fn judge(&self) -> Option<i32> {
        let directions = [
            1, // 水平方向
            ROWS as isize, // 垂直方向
            (ROWS + 1) as isize, // 斜め右下
            (ROWS - 1) as isize, // 斜め左下
        ];

        for &player_board in [self.board.player1, self.board.player2].iter() {
            for &dir in &directions {
                let mut bb = player_board;
                for s in 0..=3 {
                    bb &= player_board >> (dir * s);
                }
                if bb != 0 {
                    return Some(if player_board == self.board.player1 { 1 } else { 2 });
                }
            }
        }

        let combined_board = self.board.player1 | self.board.player2;
        if combined_board.count_ones() as usize == ROWS * COLS {
            return Some(0); // 引き分け
        }

        None
    }

    // ボードの状態を表示する関数
    fn display_board(&self) {
        for row in (0..ROWS).rev() {
            for col in 0..COLS {
                let bit_position = col * ROWS + row;
                if self.board.player1 & (1 << bit_position) != 0 {
                    print!(" X ");
                } else if self.board.player2 & (1 << bit_position) != 0 {
                    print!(" O ");
                } else {
                    print!(" . ");
                }
            }
            println!();
        }
        println!(" 1  2  3  4  5  6  7");
    }
}

fn main() {
    let mut game = Game {
        board: BitBoard {
            player1: 0,
            player2: 0,
        },
    };

    let mut current_player = true; // true -> Player 1, false -> Player 2

    loop {
        game.display_board();
        let player = if current_player { 1 } else { 2 };
        println!("Player {}'s turn. Enter a column (1-7): ", player);

        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let col: usize = match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= 7 => num - 1,
            _ => {
                println!("Invalid input. Please enter a column number between 1 and 7.");
                continue;
            }
        };

        match game.drop_disc(col, current_player) {
            Ok(_) => {
                if let Some(winner) = game.judge() {
                    game.display_board();
                    match winner {
                        0 => println!("The game is a draw!"),
                        1 => println!("Player 1 wins!"),
                        2 => println!("Player 2 wins!"),
                        _ => unreachable!(),
                    }
                    break;
                }
                current_player = !current_player; // プレイヤーを切り替える
            }
            Err(e) => println!("{}", e),
        }
    }
}

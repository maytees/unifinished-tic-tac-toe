use std::io;
use std::io::Write;

#[derive(Clone, Copy)]
enum TicTacBoxValue {
    X,
    O,
    Empty,
}

#[derive(PartialEq, Eq)]
enum GameResult {
    XPlayer,
    OPlayer,
    Tie,
    GameNotFinished,
}

#[derive(Clone, Copy)]
struct TicTacBox {
    pub value: TicTacBoxValue,
}

fn start_board() -> [TicTacBox; 9] {
    [TicTacBox {
        value: TicTacBoxValue::Empty,
    }; 9]
}

fn draw_board(board: &[TicTacBox; 9]) {
    let mut index = 1;

    for t_box in board {
        match t_box.value {
            TicTacBoxValue::X => {
                if index != 4 {
                    print!("| X |");
                } else {
                    index = 1;
                    print!("\n| X ");
                }
            }
            TicTacBoxValue::O => {
                if index != 4 {
                    print!("| O  ")
                } else {
                    print!("\n| O |");
                }
            }
            TicTacBoxValue::Empty => {
                if index != 4 {
                    print!("|   ")
                } else {
                    print!("\n|   |");
                }
            }
        }
        index = index + 1;
    }
}

// This will return the count of X and O's
// I dont know why I wrote this function
fn count_board_xo(board: &[TicTacBox; 9]) -> (u8, u8) {
    let mut xcount = 0;
    let mut ocount = 0;

    for tt_box in board {
        match tt_box.value {
            TicTacBoxValue::X => xcount = xcount + 1,
            TicTacBoxValue::O => ocount = ocount + 1,
            TicTacBoxValue::Empty => continue,
        }
    }

    (xcount, ocount)
}

fn count_board_empty(board: &[TicTacBox; 9]) -> u8 {
    let mut empty_boxes = 0;

    for tt_box in board {
        match tt_box.value {
            TicTacBoxValue::Empty => empty_boxes = empty_boxes + 1,
            _ => (),
        }
    }
    empty_boxes
}

fn check_board(board: &[TicTacBox; 9]) -> GameResult {
    let mut game_result = GameResult::GameNotFinished;
    let mut found: bool = false;

    // Check Column
    let mut index_column = 0;
    for tt_box in board {
        index_column = index_column + 1;

        if (index_column == 1) | (index_column == 4) | (index_column == 7) {
            match tt_box.value {
                TicTacBoxValue::X => match board[index_column + 1].value {
                    TicTacBoxValue::X => match board[index_column + 2].value {
                        TicTacBoxValue::X => {
                            game_result = GameResult::XPlayer;
                            found = true;
                            break;
                        }
                        _ => (),
                    },
                    _ => (),
                },
                TicTacBoxValue::O => match board[index_column + 1].value {
                    TicTacBoxValue::O => match board[index_column + 2].value {
                        TicTacBoxValue::O => {
                            game_result = GameResult::OPlayer;
                            found = true;
                            break;
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        }
    }

    // Check Row
    if !found {
        let mut index_row = 0;
        for tt_box in board {
            index_row = index_row + 1;

            if (index_row == 1) | (index_row == 2) | (index_row == 3) {
                match tt_box.value {
                    TicTacBoxValue::X => match board[index_row + 3].value {
                        TicTacBoxValue::X => match board[index_row + 6].value {
                            TicTacBoxValue::X => {
                                game_result = GameResult::XPlayer;
                                found = true;
                                break;
                            }
                            _ => (),
                        },
                        _ => (),
                    },
                    TicTacBoxValue::O => match board[index_row + 3].value {
                        TicTacBoxValue::O => match board[index_row + 6].value {
                            TicTacBoxValue::O => {
                                game_result = GameResult::OPlayer;
                                found = true;
                                break;
                            }
                            _ => (),
                        },
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }

    if (!found) || (count_board_empty(&board) == 0) {
        game_result = GameResult::Tie;
    } else {
        game_result = GameResult::GameNotFinished;
    }

    game_result
}

fn mark_board(player: TicTacBoxValue, board: [TicTacBox; 9]) -> bool {
    let sucess: bool = true;
}

fn main() {
    let mut board: [TicTacBox; 9] = start_board();
    let mut game_result = GameResult::GameNotFinished;

    draw_board(&board);

    while game_result != GameResult::GameNotFinished {
        let mut line = String::new();
        print!("Player X, where would you like to put your mark");
        std::io::stdin().read_line(&mut line).unwrap();
    }

    io::stdout().flush().unwrap();
}

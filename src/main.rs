use std::process::exit;

use rand::Rng;

#[derive(Clone, PartialEq)]
enum Type {
    Flag,
    Mine,
    Hidden,
    None
}

type BoardType = Vec<Vec<Type>>;

struct Board {
    board: BoardType,
    shown_board: BoardType,
    length: u8,
    height: u8
}

impl Board {
    fn new(length: u8, height: u8) -> Board {
        let board: BoardType = vec![vec![Type::None; length.into()]; height.into()];
        let shown_board: BoardType = vec![vec![Type::Hidden; length.into()]; height.into()];

        Board { board, shown_board, length, height }
    }

    fn print(&self, board: &BoardType) {
        for i in board {
            for v in i {
                let char = match v {
                    Type::Mine => "M",
                    Type::Flag => "#",
                    Type::Hidden => "-",
                    Type::None => " "
                };

                print!("{} ", char);
            }
            println!();
        }
    }

    fn place_mines(&mut self, amount: u16) {
        let mut rng = rand::thread_rng();

        for _ in 1..=amount {
            let x = rng.gen_range(0..self.height);
            let y = rng.gen_range(0..self.length);

            self.board[x as usize][y as usize] = Type::Mine;
        }
    }

    fn click(&mut self, x: u8, y: u8) {
        let space = &mut self.shown_board[x as usize][y as usize];
        let actual = &self.board[x as usize][y as usize];

        if space == &Type::Flag {
            return;
        }

        match actual {
            // The actual board shouldn't include a flag or hidden
            Type::Flag => unreachable!(),
            Type::Hidden => unreachable!(),
            Type::Mine => {
                *space = Type::None;
            },
            _ => {
                *space = actual.clone();
            }
        }

        if actual == &Type::Mine {
            // Lose the game
            println!("You lose.");
            self.print(&self.shown_board);

            exit(0);
        }
    }

    fn flag(&mut self, x: u8, y: u8) {
        self.shown_board[x as usize][y as usize] = Type::Flag;
    }
}

struct Console {}

impl Console {
    fn new() -> Console {
        Console {}
    }

    fn clear(&self) {
        print!("\x1bc");
        println!("Minesweeper\n");
    }
}

fn main() {
    let console = Console::new();
    console.clear();

    let mut board = Board::new(10, 10);
    board.place_mines(10);
    
    board.flag(5, 5);
    board.click(4, 4);

    board.print(&board.shown_board);
}

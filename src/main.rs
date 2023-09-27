use std::error::Error;
use std::io::{self, Write};
use std::process::exit;

use rand::Rng;

#[derive(Clone, PartialEq)]
enum Type {
    Flag,
    Mine,
    Hidden,
    None,
}

type BoardType = Vec<Vec<Type>>;

struct Board {
    board: BoardType,
    shown_board: BoardType,
    length: u8,
    height: u8,
}

impl Board {
    fn new(length: u8, height: u8) -> Board {
        let board: BoardType = vec![vec![Type::None; length.into()]; height.into()];
        let shown_board: BoardType = vec![vec![Type::Hidden; length.into()]; height.into()];

        Board {
            board,
            shown_board,
            length,
            height,
        }
    }

    fn print(&self, board: &BoardType) {
        for i in board {
            for v in i {
                let char = match v {
                    Type::Mine => "M",
                    Type::Flag => "#",
                    Type::Hidden => "-",
                    Type::None => " ",
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
        if x >= self.length {
            return;
        }
        if y >= self.height {
            return;
        }

        let space = &mut self.shown_board[x as usize][y as usize];
        let actual = &self.board[x as usize][y as usize];

        if space == &Type::Flag {
            return;
        }

        match actual {
            // The actual board shouldn't include a flag or hidden
            Type::Flag => unreachable!(),
            Type::Hidden => unreachable!(),
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

    fn input(&self, query: &str) -> String {
        print!("{}", query);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input.trim().into()
    }

    fn get_x_and_y(&self) -> Result<(u8, u8), Box<dyn Error>> {
        let x = self.input("X: ").parse::<u8>()?;
        let y = self.input("Y: ").parse::<u8>()?;

        if x < 1 || y < 1 {
            return Err("Integer underflow".into());
        }

        Ok((x - 1, y - 1))
    }
}

fn main() {
    let console = Console::new();
    console.clear();

    let mut board = Board::new(10, 10);
    board.place_mines(10);

    loop {
        console.clear();
        board.print(&board.shown_board);

        let what = console
            .input("\nWhat do you want to do? ([C]lick, [F]lag, [E]xit): ")
            .chars()
            .next()
            .unwrap()
            .to_ascii_lowercase();

        match what {
            'c' => {
                let (x, y) = match console.get_x_and_y() {
                    Ok(t) => t,
                    Err(_) => continue,
                };

                board.click(x, y);
            }
            'f' => {
                let (x, y) = match console.get_x_and_y() {
                    Ok(t) => t,
                    Err(_) => continue,
                };

                board.flag(x, y);
            }
            'e' => {
                exit(0);
            }
            _ => {
                console.input("Unknown value.\n");
            }
        }
    }
}

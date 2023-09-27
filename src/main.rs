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
        for (x, i) in board.iter().enumerate() {
            for (y, v) in i.iter().enumerate() {
                let number = self.find_close_mines(x as isize, y as isize) as u32;
                let number = match number.to_string().chars().next().unwrap() {
                    '0' => ' ',
                    x => x,
                };

                let char = match v {
                    Type::Mine => 'M',
                    Type::Flag => '#',
                    Type::Hidden => '-',
                    Type::None => number.to_string().chars().next().unwrap(),
                };

                print!("{} ", char);
            }
            println!();
        }
    }

    fn index_board(&self, x: isize, y: isize) -> &Type {
        if x < 0 || y < 0 {
            return &Type::None;
        }
        if (x as u8) >= self.height || (y as u8) >= self.length {
            return &Type::None;
        }

        &self.board[x as usize][y as usize]
    }
    fn find_close_mines(&self, x: isize, y: isize) -> usize {
        let board = [
            self.index_board(x, y - 1),
            self.index_board(x, y + 1),
            self.index_board(x - 1, y),
            self.index_board(x + 1, y),
            self.index_board(x - 1, y - 1),
            self.index_board(x - 1, y + 1),
            self.index_board(x + 1, y - 1),
            self.index_board(x + 1, y + 1),
        ];

        board.iter().filter(|x| **x == &Type::Mine).count()
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

        let space = &mut self.shown_board[y as usize][x as usize];
        let actual = &self.board[y as usize][x as usize];

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
        let space = &mut self.shown_board[y as usize][x as usize];

        match space {
            Type::Hidden => {
                *space = Type::Flag;
            }
            Type::Flag => {
                *space = Type::Hidden;
            }
            _ => {}
        }
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
        // For debugging
        //board.print(&board.board);

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

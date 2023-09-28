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

#[derive(Clone)]
struct Board {
    board: BoardType,
    shown_board: BoardType,
    length: u8,
    height: u8,
    mines: u32,
    shown_mines: i32,
    lost: bool,
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
            mines: 0,
            shown_mines: 0,
            lost: false,
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
                    Type::None => number,
                };

                print!("{} ", char);
            }
            println!("{} ", x + 1);
        }

        for i in 1..=self.length {
            print!("{} ", i.to_string().chars().last().unwrap());
        }

        println!();
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

    fn find_neighbors(&self, x: isize, y: isize) -> [(isize, isize, &Type); 8] {
        [
            (x, y - 1, self.index_board(x, y - 1)),
            (x, y + 1, self.index_board(x, y + 1)),
            (x - 1, y, self.index_board(x - 1, y)),
            (x + 1, y, self.index_board(x + 1, y)),
            (x - 1, y - 1, self.index_board(x - 1, y - 1)),
            (x - 1, y + 1, self.index_board(x - 1, y + 1)),
            (x + 1, y - 1, self.index_board(x + 1, y - 1)),
            (x + 1, y + 1, self.index_board(x + 1, y + 1)),
        ]
    }
    fn find_neighbors_no_diagonal(&self, x: isize, y: isize) -> [(isize, isize, &Type); 4] {
        [
            (x, y - 1, self.index_board(x, y - 1)),
            (x, y + 1, self.index_board(x, y + 1)),
            (x - 1, y, self.index_board(x - 1, y)),
            (x + 1, y, self.index_board(x + 1, y)),
        ]
    }

    fn find_close_mines(&self, x: isize, y: isize) -> usize {
        self.find_neighbors(x, y)
            .iter()
            .filter(|x| x.2 == &Type::Mine)
            .count()
    }

    fn place_mines(&mut self, amount: u32) {
        let mut rng = rand::thread_rng();

        for _ in 1..=amount {
            let x = rng.gen_range(0..self.height);
            let y = rng.gen_range(0..self.length);

            self.board[x as usize][y as usize] = Type::Mine;
        }

        self.mines = amount;
        self.shown_mines = amount as i32;
    }

    fn move_all_mines_neighboring(&mut self, x: u8, y: u8) {
        // Move all neighbor mines somewhere else.
        let binding = self.clone();
        let board = binding.find_neighbors(x as isize, y as isize);

        let mut rng = rand::thread_rng();
        while self.index_board(x as isize, y as isize) == &Type::Mine
            || self.find_close_mines(x as isize, y as isize) > 0
        {
            for (oldx, oldy, t) in board {
                match t {
                    Type::Mine => {}
                    _ => continue,
                }

                let newx = rng.gen_range(0..self.height);
                let newy = rng.gen_range(0..self.length);

                self.board[newx as usize][newy as usize] = Type::Mine;
                self.board[oldx as usize][oldy as usize] = Type::None;
            }

            if self.index_board(x as isize, y as isize) == &Type::Mine {
                // Move the mine somewhere else.
                let newx = rand::thread_rng().gen_range(0..self.height);
                let newy = rand::thread_rng().gen_range(0..self.length);

                self.board[newx as usize][newy as usize] = Type::Mine;
                self.board[x as usize][y as usize] = Type::None;
            }
        }
    }

    fn first_click(&mut self, x: u8, y: u8) {
        self.move_all_mines_neighboring(x, y);
        self.click(x, y, true);
    }

    fn click(&mut self, x: u8, y: u8, first: bool) {
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
            self.lost = true;
        }
        if actual == &Type::None {
            if self.find_close_mines(x as isize, y as isize) > 0 && !first {
                return;
            }

            self.reveal_empties(x as isize, y as isize, &mut vec![]);
        }
    }

    fn flag(&mut self, x: u8, y: u8) {
        let space = &mut self.shown_board[y as usize][x as usize];
        let actual = &mut self.board[y as usize][x as usize];

        match space {
            Type::Hidden => {
                *space = Type::Flag;

                self.shown_mines -= 1;
                if actual == &Type::Mine {
                    self.mines -= 1;
                }
            }
            Type::Flag => {
                *space = Type::Hidden;

                self.shown_mines += 1;
                if actual == &Type::Mine {
                    self.mines += 1;
                }
            }
            _ => {}
        }
    }

    fn reveal_empties(&mut self, x: isize, y: isize, traversed: &mut Vec<(isize, isize)>) {
        if x < 0 || y < 0 {
            return;
        }
        if (x as u8) >= self.length || (y as u8) >= self.height {
            return;
        }

        if traversed.contains(&(x, y)) {
            return;
        }

        traversed.push((x, y));
        for (x, y, c) in self.clone().find_neighbors_no_diagonal(x, y) {
            if x < 0 || y < 0 {
                continue;
            }

            if c == &Type::None {
                // It is a number. Reveal the number but dont reveal past it.
                if self.find_close_mines(x, y) > 0 {
                    self.update_cell(x, y, Type::None);
                    continue;
                }

                self.reveal_empties(x, y, traversed);
                self.update_cell(x, y, Type::None);
            }

            traversed.push((x, y));
        }
    }

    fn update_cell(&mut self, x: isize, y: isize, t: Type) {
        if x < 0 || y < 0 {
            return;
        }
        if (x as u8) >= self.length || (y as u8) >= self.height {
            return;
        }

        self.shown_board[x as usize][y as usize] = t;
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

    let mut board = Board::new(9, 9);
    board.place_mines(10);

    let mut first = true;
    loop {
        console.clear();
        println!("Mines left {}", board.shown_mines);
        board.print(&board.shown_board);
        // For debugging
        // board.print(&board.board);

        let what = match console
            .input("\nWhat do you want to do? ([C]lick, [F]lag, [E]xit): ")
            .chars()
            .next()
        {
            Some(what) => what.to_ascii_lowercase(),
            None => continue,
        };

        match what {
            'c' => {
                let (x, y) = match console.get_x_and_y() {
                    Ok(t) => t,
                    Err(_) => continue,
                };

                if first {
                    board.first_click(x, y);
                } else {
                    board.click(x, y, false);
                }
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

        first = false;

        // The player shouldn't be able to just flag every square to win.
        // Prevent that using the second condition
        if board.mines == 0 && board.shown_mines == 0 {
            console.clear();

            println!("You win!");
            board.print(&board.board);

            exit(0);
        }

        if board.lost {
            console.clear();

            println!("You lose.");
            board.print(&board.board);

            exit(0);
        }
    }
}

use rand::Rng;

struct Board {
    board: Vec<Vec<char>>,
    length: u8,
    height: u8
}

impl Board {
    fn new(length: u8, height: u8) -> Board {
        let board = vec![vec!['-'; length.into()]; height.into()];
        Board { board, length, height }
    }

    fn print(&self) {
        for i in &self.board {
            for v in i {
                print!("{} ", v);
            }
            println!();
        }
    }

    fn place_mines(&mut self, amount: u16) {
        let mut rng = rand::thread_rng();

        for _ in 1..=amount {
            let x = rng.gen_range(0..self.height);
            let y = rng.gen_range(0..self.length);

            self.board[x as usize][y as usize] = 'x';
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
}

fn main() {
    let console = Console::new();
    console.clear();

    let mut board = Board::new(10, 10);
    board.place_mines(10);
    board.print();
}

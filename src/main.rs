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
}

struct Console {}

impl Console {
    fn new() -> Console {
        Console {}
    }

    fn clear(&self) {
        print!("\x1bc");
    }
}

fn main() {
    let console = Console::new();
    console.clear();

    let board = Board::new(5, 5);
    board.print();
}

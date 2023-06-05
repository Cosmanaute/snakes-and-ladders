struct Board {
    grid: [[usize; 10]; 10],
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [[1; 10]; 10],
        }
    }

    fn set_numbers(&mut self) {
        let mut count: usize = 1;
        for i in 0..self.grid.len() {
            for j in 0..self.grid.len() {
                self.grid[i][j] = count;
                count += 1;
            }
        }
    }

    fn display(&self) {
        for row in self.grid.iter() {
            for col in row.iter() {
                if col <= &9 {
                    let col = format!("0{}", col);
                    print!(" {} ", col);
                } else {
                    print!(" {} ", col);
                }
            }
            println!("");
        }
    }
}

fn main() {
    let mut board = Board::new();
    board.set_numbers();
    board.display();
}

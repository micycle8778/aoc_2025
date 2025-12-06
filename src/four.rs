use itertools::Itertools;
use std::{fs::File, io::Read};

#[derive(Clone)]
struct Board {
    rows: usize,
    cols: usize,
    storage: Box<[u8]>
}

impl Board {
    fn new(input_board: &str) -> Self {
        let rows = input_board.lines().count();
        let cols = input_board.lines().next().unwrap().len();
        let storage = input_board.bytes()
            .filter(|&b| b != b'\n')
            .collect::<Box<[u8]>>();

        Self {
            rows,
            cols,
            storage
        }
    }

    fn enumerated_iter(&self) -> impl Iterator<Item = (usize, usize, u8)> {
        (0..self.rows)
            .cartesian_product(0..self.cols)
            .map(|(row, col)| (row, col, self.get_cell(row, col).unwrap()))
    }

    fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = u8> {
        let row_start = if row == 0 { 0 } else { row - 1 };
        let col_start = if col == 0 { 0 } else { col - 1 };

        (row_start..=row+1)
            .cartesian_product(col_start..=col+1)
            .filter(move |&(r, c)| (r, c) != (row, col))
            .filter_map(|(row, col)| self.get_cell(row, col))
    }

    fn paper_roll_neighbors_count(&self, row: usize, col: usize) -> usize {
        self.neighbors(row, col)
            .filter(|&cell| cell == b'@')
            .count()
    }

    fn get_cell(&self, row: usize, col: usize) -> Option<u8> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(self.storage[row * self.cols + col])
        }
    }

    fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut u8> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(&mut self.storage[row * self.cols + col])
        }
    }

    fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{}", self.get_cell(row, col).unwrap() as char);
            }
            println!();
        }
    }
}

/// # Panics
/// This function panics on file read error.
pub fn solution(f: &mut File, part_two: bool) {
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let mut board = Board::new(&buf);

    let mut ret = 0;
    if part_two {
        let mut flag = true;
        while flag {
            flag = false;

            let mut new_board = board.clone();
            for (row, col, cell) in board.enumerated_iter() {
                if cell == b'@' && board.paper_roll_neighbors_count(row, col) < 4 {
                    *new_board.get_cell_mut(row, col).unwrap() = b'.';
                    ret += 1;
                    flag = true;
                }
            }

            board = new_board;
        }

        println!("{ret}");
    } else {
        println!("{}", 
            board.enumerated_iter()
                .filter_map(|(row, col, cell)| (cell == b'@').then_some((row, col)))
                .filter(|&(row, col)| board.paper_roll_neighbors_count(row, col) < 4)
                .count()
        );
    }
}

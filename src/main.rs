use std::fmt;
use std::io::BufRead;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Number {
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::N1 => write!(f, "1"),
            Self::N2 => write!(f, "2"),
            Self::N3 => write!(f, "3"),
            Self::N4 => write!(f, "4"),
            Self::N5 => write!(f, "5"),
            Self::N6 => write!(f, "6"),
            Self::N7 => write!(f, "7"),
            Self::N8 => write!(f, "8"),
            Self::N9 => write!(f, "9"),
        }
    }
}

const NUMBERS: [Number; 9] = [
    Number::N1,
    Number::N2,
    Number::N3,
    Number::N4,
    Number::N5,
    Number::N6,
    Number::N7,
    Number::N8,
    Number::N9,
];

type Field = [[Option<Number>; 9]; 9];

#[derive(Clone, Copy, Default)]
struct Board {
    field: Field,
}

impl Board {
    fn new(field: [[Option<Number>; 9]; 9]) -> Self {
        Self { field }
    }

    fn find_empty(&self) -> Option<(usize, usize, Vec<Number>)> {
        let mut max = 10;
        let mut candidate = None;
        for (i, row) in self.field.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                if num.is_some() {
                    continue;
                }
                let mut numbers = Vec::new();
                for number in &NUMBERS {
                    if self.can_set(i, j, *number) {
                        numbers.push(*number);
                    }
                }
                if numbers.len() < max {
                    max = numbers.len();
                    candidate = Some((i, j, numbers));
                }
            }
        }
        candidate
    }

    fn can_set(&self, i: usize, j: usize, number: Number) -> bool {
        for j in 0..9 {
            if self.field[i][j] == Some(number) {
                return false;
            }
        }
        for i in 0..9 {
            if self.field[i][j] == Some(number) {
                return false;
            }
        }
        let ci = i / 3 * 3 + 1;
        let cj = j / 3 * 3 + 1;
        for i in (ci - 1)..=(ci + 1) {
            for j in (cj - 1)..=(cj + 1) {
                if self.field[i][j] == Some(number) {
                    return false;
                }
            }
        }
        true
    }

    fn set(&mut self, i: usize, j: usize, number: Number) {
        self.field[i][j] = Some(number);
    }

    fn reset(&mut self, i: usize, j: usize) {
        self.field[i][j] = None;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.field {
            let row: Vec<_> = row
                .iter()
                .map(|num| {
                    if let Some(num) = num {
                        num.to_string()
                    } else {
                        "*".to_string()
                    }
                })
                .collect();
            writeln!(f, "{}", row.join(" "))?;
        }
        Ok(())
    }
}

fn solve(board: &mut Board) -> Vec<Board> {
    let mut solution = Vec::new();
    solve_helper(board, &mut solution);
    solution
}

fn solve_helper(board: &mut Board, solution: &mut Vec<Board>) {
    let (i, j, numbers) = if let Some(candidate) = board.find_empty() {
        candidate
    } else {
        solution.push(*board);
        return;
    };

    for num in &numbers {
        board.set(i, j, *num);
        solve_helper(board, solution);
        board.reset(i, j);
    }
}

fn main() {
    let mut field = [[None; 9]; 9];
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    for (i, line) in handle.lines().enumerate().take(9) {
        let line = line.unwrap();
        for (j, num) in line.split_whitespace().enumerate() {
            match num {
                "1" => field[i][j] = Some(Number::N1),
                "2" => field[i][j] = Some(Number::N2),
                "3" => field[i][j] = Some(Number::N3),
                "4" => field[i][j] = Some(Number::N4),
                "5" => field[i][j] = Some(Number::N5),
                "6" => field[i][j] = Some(Number::N6),
                "7" => field[i][j] = Some(Number::N7),
                "8" => field[i][j] = Some(Number::N8),
                "9" => field[i][j] = Some(Number::N9),
                _ => {}
            }
        }
    }
    let mut board = Board::new(field);
    let solution = solve(&mut board);

    for answer in solution {
        println!("{}", answer);
    }
}

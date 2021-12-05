use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Number {
    n: usize,
    marked: bool,
}

impl Number {
    pub fn new(n: usize) -> Self {
        Self { n, marked: false }
    }

    pub fn mark(&mut self) {
        self.marked = true
    }
}

#[derive(Debug)]
pub struct Board {
    rows: Vec<Vec<Number>>,
    done: bool,
}

impl Board {
    pub fn new(selected_numbers: Vec<Vec<Number>>) -> Self {
        Self {
            rows: selected_numbers,
            done: false,
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            rows: self.rows.to_vec(),
            done: self.done,
        }
    }

    pub fn mark(&mut self, n: usize) -> bool {
        let mut start: Option<(usize, usize)> = None;

        self.rows.iter_mut().enumerate().for_each(|(i, row)| {
            for (j, entry) in row.iter_mut().enumerate() {
                if entry.n != n {
                    continue;
                }

                entry.marked = true;
                start = Some((i, j));

                break;
            }
        });

        if let Some(x) = start {
            self.consolidate(x)
        } else {
            false
        }
    }

    pub fn consolidate(&mut self, (potential_row, potential_col): (usize, usize)) -> bool {
        let bound = 5;
        let row_count =
            self.rows[potential_row]
                .iter()
                .fold(0, |acc, n| if n.marked { acc + 1 } else { acc });
        let col_count = self
            .rows
            .iter()
            .filter(|r| r[potential_col].marked)
            .fold(0, |acc, _| acc + 1);
        self.done = bound == row_count || bound == col_count;

        self.done
    }

    pub fn get_unmarked(&self) -> Vec<usize> {
        let r: Vec<usize> = self
            .rows
            .iter()
            .flat_map(|row| row.iter().filter(|&n| !n.marked).map(|n| n.n))
            .collect();

        r
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("\n");

        for row in self.rows.iter() {
            for n in row.iter() {
                let r = format!(
                    "{:5}",
                    match n.marked {
                        true => format!("({})", n.n),
                        false => format!("{}", n.n),
                    }
                );

                output.push_str(&r);
            }

            output.push('\n')
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_winners, Board, Number};
    use shared::read_lines;
    use std::vec;

    #[test]
    fn d_04_a() {
        let sequence = [
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let selected_nums: Vec<Vec<Number>> = [
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ]
        .chunks(5)
        .fold(vec![], |mut acc, n| {
            let r = n.iter().map(|n| Number::new(*n)).collect();

            acc.push(r);
            acc
        });

        let mut b = Board::new(selected_nums);
        let mut stop = 0;

        for (i, &s) in sequence.iter().enumerate() {
            if b.mark(s) {
                stop = i;
                break;
            }
        }

        format!("stop: {}\nb1: {}", stop, b);
        println!("{:?}", b.get_unmarked())
    }

    fn read_dat() {}

    #[test]
    fn d_04a() {
        let input = read_lines("data/input.txt").expect("failed to read data for d4 a");
        let mut line_itr = input.iter();
        let game_sequence: Vec<usize> = line_itr
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut boards = vec![];
        let mut current = vec![];

        for line in line_itr {
            if line.is_empty() {
                boards.push(current);
                current = vec![];
                continue;
            }

            let row: Vec<Number> = line
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| Number::new(s.parse().expect("failed to parse number")))
                .collect();

            current.push(row)
        }

        boards.push(current);

        let mut boards: Vec<Board> = boards.into_iter().map(Board::new).collect();
        let mut winners: Vec<(usize, Vec<Board>)> = vec![];
        let mut game_sequence: Vec<usize> = game_sequence.into_iter().rev().collect();

        while let Some(ball) = game_sequence.pop() {
            let round_winners = find_winners(ball, &mut boards);

            if !round_winners.is_empty() {
                winners.push((ball, round_winners));

                break;
            }
        }

        let (ball, b) = &winners[0];
        let sum = b[0].get_unmarked().iter().sum::<usize>();
        let score = sum * ball;

        assert_eq!(score, 34506)
    }

    #[test]
    fn d_04_b() {
        let input = read_lines("data/input.txt").expect("failed to read data for d4 a");
        let mut line_itr = input.iter();
        let game_sequence: Vec<usize> = line_itr
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut boards = vec![];
        let mut current = vec![];

        for line in line_itr {
            if line.is_empty() {
                boards.push(current);
                current = vec![];
                continue;
            }

            let row: Vec<Number> = line
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| Number::new(s.parse().expect("failed to parse number")))
                .collect();

            current.push(row)
        }

        boards.push(current);

        let mut boards: Vec<Board> = boards.into_iter().map(Board::new).collect();
        let mut winners: Vec<(usize, Vec<Board>)> = vec![];
        let mut game_sequence: Vec<usize> = game_sequence.into_iter().rev().collect();

        while let Some(ball) = game_sequence.pop() {
            let round_winners = find_winners(ball, &mut boards);

            if round_winners.is_empty() {
                continue;
            }

            winners.push((ball, round_winners));
        }

        let (ball, boards) = winners.last().unwrap();
        let sum = boards[0].get_unmarked().iter().sum::<usize>() * ball;

        assert_eq!(sum, 7686);
    }
}

pub fn find_winners(n: usize, boards: &mut Vec<Board>) -> Vec<Board> {
    let mut winners: Vec<Board> = vec![];

    for b in &mut boards.iter_mut() {
        if b.mark(n) {
            winners.push(b.copy());
        }
    }

    boards.retain(|b| !b.done);

    winners
}

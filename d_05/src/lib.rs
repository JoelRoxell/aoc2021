use std::cmp;
use std::vec;

type VecPair = ((i32, i32), (i32, i32));

pub fn parse_field(data: Vec<String>) -> (i32, Vec<VecPair>) {
    let mut pairs = vec![];
    let mut outer_banks = 0;

    for line in data {
        let pair = line
            .split("->")
            .map(|s| s.trim())
            .map(String::from)
            .map(|s| {
                s.split(',')
                    .map(String::from)
                    .map(|s_n| s_n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        let x1 = pair[0][0];
        let y1 = pair[0][1];
        let x2 = pair[1][0];
        let y2 = pair[1][1];
        let max = [x1, y1, x2, y2].into_iter().max().unwrap();

        if max > outer_banks {
            outer_banks = max;
        }

        pairs.push(((pair[0][0], pair[0][1]), (pair[1][0], pair[1][1])));
    }

    (outer_banks + 1, pairs)
}

pub fn draw_field(outer_banks: i32, pairs: Vec<VecPair>, dia: bool) -> Vec<Vec<i32>> {
    let mut field: Vec<Vec<i32>> = vec![vec![0; outer_banks as usize]; outer_banks as usize];

    for (p1, p2) in pairs {
        if !dia && (p1.0 != p2.0 && p1.1 != p2.1) {
            continue;
        }

        let x_dist = p1.0 - p2.0;
        let y_dist = p1.1 - p2.1;
        let vec_len = cmp::max(x_dist.abs(), y_dist.abs());

        let x_coordinates: Vec<i32> = match x_dist {
            d if d < 0 => (p1.0..=p2.0).rev().collect(),
            0 => vec![p1.0; (vec_len + 1) as usize],
            _ => (p2.0..=p1.0).collect(),
        };
        let y_coordinates: Vec<i32> = match y_dist {
            d if d < 0 => (p1.1..=p2.1).rev().collect(),
            0 => vec![p1.1; (vec_len + 1) as usize],
            _ => (p2.1..=p1.1).collect(),
        };

        for i in 0..y_coordinates.len() {
            field[(y_coordinates[i]) as usize][(x_coordinates[i]) as usize] += 1;
        }
    }

    field
}

#[cfg(test)]
mod tests {
    use crate::{draw_field, parse_field};
    use shared::read_lines;

    #[test]
    fn read_field_test() {
        let raw_lines = read_lines("data/demo.txt").unwrap();
        let (outer_banks, pairs) = parse_field(raw_lines);

        assert_eq!(outer_banks, 10);
        assert_eq!(pairs.len(), 10);
    }

    #[test]
    fn d05_a() {
        let raw_lines = read_lines("data/demo.txt").unwrap();
        let (outer_banks, pairs) = parse_field(raw_lines);
        let field = draw_field(outer_banks, pairs, false);

        assert_eq!(5, sum(field));
    }

    #[test]
    fn d05_b() {
        let raw_lines = read_lines("data/demo.txt").unwrap();
        let (outer_banks, pairs) = parse_field(raw_lines);
        let field = draw_field(outer_banks, pairs, true);

        assert_eq!(12, sum(field));
    }

    #[test]
    fn d05a() {
        let raw_lines = read_lines("data/input.txt").unwrap();
        let (outer_banks, pairs) = parse_field(raw_lines);
        let field = draw_field(outer_banks, pairs, false);

        assert_eq!(3990, sum(field));
    }

    #[test]
    fn d05b() {
        let raw_lines = read_lines("data/input.txt").unwrap();
        let (outer_banks, pairs) = parse_field(raw_lines);
        let field = draw_field(outer_banks, pairs, true);

        assert_eq!(21305, sum(field));
    }

    fn sum(field: Vec<Vec<i32>>) -> usize {
        let mut sum = 0;

        for row in &field {
            for i in row {
                let v = *i;

                if v > 1 {
                    sum += 1;
                }
            }
        }

        sum
    }
}

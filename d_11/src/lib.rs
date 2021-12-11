use shared::read_lines;
use std::collections::HashMap;

type Field = Vec<Vec<usize>>;

pub fn read_field(file_path: &str) -> Field {
    let lines = read_lines(file_path).unwrap();

    let field: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    field
}

pub fn get_cache_key(i: usize, j: usize) -> String {
    format!("i{}j{}", i, j)
}

pub fn flash(
    i: usize,
    j: usize,
    flashed: &mut HashMap<String, bool>,
    field: &mut Field,
    flash_count: &mut usize,
    h: usize,
    w: usize,
) {
    let key = get_cache_key(i, j);
    let current = field[i][j];

    if current < 10 || flashed.contains_key(&key) {
        return;
    }

    field[i][j] = 0;
    *flash_count += 1;
    flashed.insert(key, true);

    let charged_neighbors = charge_neighbors(i, j, field, flashed, h, w);

    for (i, j) in charged_neighbors {
        flash(i, j, flashed, field, flash_count, h, w)
    }
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
];

pub fn charge_neighbors(
    i: usize,
    j: usize,
    field: &mut Field,
    flashed: &mut HashMap<String, bool>,
    h: usize,
    w: usize,
) -> Vec<(usize, usize)> {
    let mut loads = vec![];

    for (x, y) in DIRECTIONS {
        let next_x = j as i32 + x;
        let next_y = i as i32 + y;

        if next_x < 0 || next_x > w as i32 || next_y < 0 || next_y > h as i32 {
            continue;
        }

        let next_y = next_y as usize;
        let next_x = next_x as usize;

        if flashed.contains_key(&get_cache_key(next_y, next_x)) {
            continue;
        }

        field[next_y][next_x] += 1;
        loads.push((next_y, next_x));
    }

    loads
}

pub fn run(field: &mut Field, rounds: usize, stop_at_sync: bool) -> (usize, isize) {
    let mut flash_count = 0;
    let (h, w) = (field.len(), field[0].len());
    let mut sync = -1;

    for round in 0..rounds {
        let mut n = HashMap::new();

        for i in 0..h {
            for j in 0..w {
                if n.contains_key(&get_cache_key(i, j)) {
                    continue;
                }

                field[i][j] += 1;

                flash(i, j, &mut n, field, &mut flash_count, h - 1, w - 1);
            }
        }

        let field_sum = field
            .iter()
            .fold(0, |acc, row| acc + row.iter().sum::<usize>());

        if field_sum == 0 && stop_at_sync {
            sync = round as isize;

            break;
        }
    }

    (flash_count, sync + 1)
}

#[cfg(test)]
mod tests {
    use crate::{read_field, run};

    #[test]
    fn d11_a() {
        let mut field = read_field("data/input.txt");
        let (flash_count, _) = run(&mut field, 100, false);

        assert_eq!(1591, flash_count)
    }

    #[test]
    fn d11_b() {
        let mut field = read_field("data/input.txt");
        let (_, itr) = run(&mut field, usize::MAX, true);

        assert_eq!(314, itr)
    }
}

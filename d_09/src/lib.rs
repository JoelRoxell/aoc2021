use shared::read_lines;
use std::collections::HashMap;

type Field = Vec<Vec<usize>>;

pub fn read_field(file: &str) -> (Field, usize, usize) {
    let lines = read_lines(file).unwrap();
    let field: Vec<Vec<usize>> = lines
        .iter()
        .map(|s| s.chars().map(|s| s.to_string().parse().unwrap()).collect())
        .collect();

    let w = field[0].len();
    let h = field.len();

    (field, h, w)
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

pub fn is_low_point(field: &Field, h: usize, w: usize, i: usize, j: usize) -> bool {
    let current = field[i][j];

    for (y, x) in DIRECTIONS {
        let c_y = (i as i32) + y;
        let c_x = (j as i32) + x;

        if c_y < 0 || c_x < 0 || c_y >= (h as i32) || c_x >= (w as i32) {
            continue;
        }

        if current >= field[c_y as usize][c_x as usize] {
            return false;
        }
    }

    true
}

pub fn expand(
    field: &Field,
    h: usize,
    w: usize,
    i: usize,
    j: usize,
    explored: &mut HashMap<String, bool>,
) -> usize {
    let current = field[i][j];

    if current == 9 || explored.contains_key(&format!("{}:{}", i, j)) {
        return 0;
    }

    explored.insert(format!("{}:{}", i, j), true);

    let mut n_count = 1;

    for (y, x) in DIRECTIONS {
        let c_y = (i as i32) + y;
        let c_x = (j as i32) + x;

        if c_y < 0 || c_x < 0 || c_y >= (h as i32) || c_x >= (w as i32) {
            continue;
        }

        n_count += expand(field, h, w, c_y as usize, c_x as usize, explored)
    }

    n_count
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{expand, is_low_point, read_field};

    #[test]
    fn a() {
        let (field, h, w) = read_field("data/input.txt");
        let mut sum = 0;

        for (i, row) in field.iter().enumerate() {
            for (j, pos) in row.iter().enumerate() {
                let x = is_low_point(&field, h, w, i, j);

                if x {
                    sum += *pos + 1;
                }
            }
        }

        assert_eq!(sum, 486)
    }

    #[test]
    fn b() {
        let (field, h, w) = read_field("data/input.txt");
        let mut explored = HashMap::new();
        let mut basins = vec![];

        for (i, row) in field.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let n = expand(&field, h, w, i, j, &mut explored);

                if n > 0 {
                    basins.push(n)
                }
            }
        }

        basins.sort_unstable();

        let top_three = &basins[basins.len() - 3..];
        let res: usize = top_three.iter().product();

        assert_eq!(res, 1059300);
    }
}

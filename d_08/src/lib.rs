use std::collections::HashMap;

use shared::read_lines;

pub type Entry = (Vec<String>, Vec<String>);

pub fn read_input(input: &str) -> Vec<Entry> {
    let x = read_lines(input).unwrap();
    let mut input_vec = vec![];

    for l in x {
        let l_vec: Vec<_> = l.split('|').collect();
        let chunk_one: Vec<_> = l_vec[0]
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let chunk_two: Vec<_> = l_vec[1]
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();

        input_vec.push((chunk_one, chunk_two));
    }

    input_vec
}

pub enum N {
    One,
    Four,
    Seven,
}

pub fn find_number(n: N, v: &[String]) -> Option<String> {
    let n = match n {
        N::One => 2,
        N::Four => 4,
        N::Seven => 3,
    };

    let n = v.iter().find(|&s| s.len() == n);

    match n {
        Some(n) => {
            let mut n: Vec<_> = n.chars().collect();

            n.sort_unstable();

            let n = String::from_iter(n.iter());

            Some(n)
        }
        None => None,
    }
}

const One: &str = "cf";
const Four: &str = "bdcf";
const Seven: &str = "acf";

pub fn str_permutation(a: &str, b: &str) -> Vec<HashMap<char, char>> {
    let mut combos = vec![];
    let rounds = a.len();
    let a: Vec<_> = a.chars().collect();
    let b: Vec<_> = b.chars().collect();

    for shift in 0..rounds {
        let mut combo = HashMap::new();

        for i in 0..rounds {
            combo.insert(a[i], b[(i + shift) % rounds]);
        }

        combos.push(combo)
    }

    combos
}

pub fn solve_given(n: N, input: &str, given: HashMap<char, char>) -> Vec<HashMap<char, char>> {
    let n = match n {
        N::Seven => Seven,
        N::One => One,
        N::Four => Four,
    };

    let a: String = input.chars().filter(|c| !given.contains_key(c)).collect();
    let b: String = n
        .chars()
        .filter(|c| {
            for v in given.values() {
                if v == c {
                    return false;
                }
            }

            true
        })
        .collect();

    let mut r = str_permutation(&a, &b);

    for ir in &mut r {
        ir.extend(given.clone())
    }

    r
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{find_number, read_input, solve_given, str_permutation, One, Seven, N};
    #[test]
    fn a() {
        let input = read_input("data/input.txt");
        let mut sum = 0;

        for (_, chunk) in input {
            sum += chunk
                .iter()
                .filter(|&c| {
                    let n = c.len();

                    matches!(n, 2 | 4 | 3 | 7)
                })
                .count();
        }

        assert_eq!(sum, 321)
    }

    #[test]
    fn b() {
        let x = read_input("data/single.txt");

        let single = &x[0].0;

        let one = find_number(N::One, single).unwrap();
        let four = find_number(N::Four, single).unwrap();
        let seven = find_number(N::Seven, single).unwrap();

        println!("\t1={}\t4={}\t7={}", one, four, seven);
        let g = HashMap::new();

        let one_s = solve_given(N::One, &one, g);
        println!("{:?}", one_s[0]);
        let seven_s = solve_given(N::Seven, &seven, one_s[0].clone());
        println!("{:?}", seven_s[0]);
        let four_s = solve_given(N::Four, &four, seven_s[0].clone());
        println!("{:?}", four_s);
    }

    #[test]
    fn str_perm_test() {
        let ps = str_permutation("ab", "cf");

        let y = &ps[0];
        let a = y.get(&'a').unwrap();
        let b = y.get(&'b').unwrap();

        assert_eq!(*a, 'c');
        assert_eq!(*b, 'f');

        let y = &ps[1];
        let a = y.get(&'a').unwrap();
        let b = y.get(&'b').unwrap();

        assert_eq!(*a, 'f');
        assert_eq!(*b, 'c');

        assert_eq!(ps.len(), 2);
    }

    #[test]
    fn solve_given_test() {
        let g = HashMap::from([('a', 'c'), ('b', 'f')]);

        let solutions = solve_given(N::Seven, "dab", g);

        println!("{:?}", solutions);

        let g = HashMap::from([]);
        let solutions = solve_given(N::Seven, "dab", g);

        println!("{:?}", solutions);

        let g = HashMap::from([]);
        let solutions = solve_given(N::One, "ab", g);

        println!("{:?}", solutions);
    }
}

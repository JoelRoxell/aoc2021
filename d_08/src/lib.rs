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
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

pub fn find_number(n: &N, v: &[String]) -> Vec<String> {
    let n = match n {
        N::One => ONE.len(),
        N::Two => TWO.len(),
        N::Three => THREE.len(),
        N::Four => FOUR.len(),
        N::Five => FIVE.len(),
        N::Six => SIX.len(),
        N::Seven => SEVEN.len(),
        N::Eight => EIGHT.len(),
        N::Nine => NINE.len(),
    };

    let n: Vec<_> = v.iter().filter(|&s| s.len() == n).cloned().collect();

    n
}

const NUMBERS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bdcf", "abdfg", "abdefg", "acf", "acedgfb", "abcdfg",
];

const ONE: &str = "cf";
const TWO: &str = "acdeg";
const THREE: &str = "acdfg";
const FOUR: &str = "bdcf";
const FIVE: &str = "abdfg";
const SIX: &str = "abdefg";
const SEVEN: &str = "acf";
const NINE: &str = "abcdfg";
const EIGHT: &str = "acedgfb";

pub fn str_permutation(a: &str, b: &str) -> Vec<HashMap<char, char>> {
    let mut combos = vec![];
    let rounds = a.len();
    let a: Vec<_> = a.chars().collect();
    let b: Vec<_> = b.chars().collect();

    if a.len() == 1 && b.is_empty() {
        combos.push(HashMap::from([(a[0], a[0])]));

        return combos;
    }

    if b.len() == 1 {
        combos.push(HashMap::from([(a[0], b[0])]));

        return combos;
    }

    for shift in 0..rounds {
        let mut combo = HashMap::new();

        for i in 0..rounds {
            combo.insert(a[i], b[(i + shift) % rounds]);
        }

        combos.push(combo)
    }

    combos
}

pub fn solve_given(
    n: &N,
    input: &str,
    given: HashMap<char, char>,
) -> Option<Vec<HashMap<char, char>>> {
    let n = match n {
        N::Seven => SEVEN,
        N::One => ONE,
        N::Four => FOUR,
        N::Two => TWO,
        N::Three => THREE,
        N::Five => FIVE,
        N::Six => SIX,
        N::Eight => EIGHT,
        N::Nine => NINE,
    };

    // validate given map for N
    for char_input in input.chars() {
        let x = given.get(&char_input);

        if let Some(x) = x {
            if n.find(*x).is_none() {
                // println!(
                //     "should stop, {} -> {} cannot be set to create {}",
                //     char_input, *x, n
                // );

                return None;
            }
        }
    }

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

    if a.len() != b.len() {
        return None;
    }

    if a.is_empty() && b.is_empty() {
        return Some(vec![given]);
    }

    let mut r = str_permutation(&a, &b);

    for ir in &mut r {
        ir.extend(given.clone())
    }

    Some(r)
}

const ORDER: [N; 5] = [N::One, N::Seven, N::Four, N::Six, N::Three];

pub fn filter_vec_str(s: &str, s_vec: &[String]) -> Vec<String> {
    let x: Vec<_> = s_vec
        .iter()
        .filter(|&s1| *s1 != s)
        .map(|s| s.to_string())
        .collect();

    x
}

pub fn solve_row(
    row: &[String],
    i: usize,
    mem: HashMap<char, char>,
) -> Option<HashMap<char, char>> {
    if i == 5 {
        return Some(mem);
    }

    let n = &ORDER[i];
    let potential_matches = find_number(n, row);

    for block in potential_matches {
        let solutions = solve_given(n, &block, mem.clone());

        if let Some(solutions) = solutions {
            // let mut branches = vec![];
            for s in solutions {
                let s1 = filter_vec_str(&block, row);
                let ss = solve_row(&s1, i + 1, s);

                if ss.is_some() {
                    return Some(match ss {
                        Some(val) => val,
                        None => panic!("called `Option::unwrap()` on a `None` value"),
                    });
                }
            }
        }
    }

    None
}

pub fn transform_puzzle(code: &str, m: &HashMap<char, char>) -> String {
    code.chars().map(|c| m.get(&c).unwrap()).collect()
}

pub fn sort(a: &str) -> String {
    let mut v: Vec<_> = a.chars().into_iter().collect();

    v.sort_unstable();
    v.into_iter().collect()
}

pub fn get_number(n: &str) -> Option<usize> {
    for (i, x) in NUMBERS.iter().enumerate() {
        let a = sort(*x);
        let b = sort(n);

        if a == b {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        get_number, read_input, solve_given, solve_row, sort, str_permutation, transform_puzzle, N,
    };
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
        let data = read_input("data/input.txt");
        let mut sum = 0;

        for (blocks, puzzle) in data {
            let m = HashMap::new();
            let key = solve_row(&blocks, 0, m);

            match key {
                Some(m) => {
                    let mut res = String::new();

                    for item in puzzle {
                        let n_str = transform_puzzle(&item, &m);
                        // println!("{} = {}", sort(&item), sort(&n_str));

                        let n = get_number(&n_str).unwrap();

                        res.push_str(&n.to_string());
                    }

                    sum += res.parse::<usize>().unwrap();
                }
                None => println!("found no solution"),
            }
        }

        println!("{}", sum)
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

        let solutions = solve_given(&N::Seven, "dab", g);

        println!("{:?}", solutions);

        let g = HashMap::from([]);
        let solutions = solve_given(&N::Seven, "dab", g);

        println!("{:?}", solutions);

        let g = HashMap::from([]);
        let solutions = solve_given(&N::One, "ab", g);

        println!("{:?}", solutions);
    }
}

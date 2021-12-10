use std::{collections::HashMap, fmt::LowerExp};

pub fn find_lowest_cost_1(v: &mut Vec<i32>) -> i32 {
    v.sort_unstable();

    let i = v.len() / 2;
    let m = v[i];
    let cost = v.iter().fold(0, |acc, n| acc + (n - m).abs());

    cost
}

pub fn sum_to(n: i32) -> i32 {
    let mut sum = 0;

    for i in 1..=n {
        sum += i;
    }

    sum
}

pub fn find_lowest_cost_2(v: &mut Vec<i32>) -> i32 {
    let mut h = HashMap::new();

    v.iter().for_each(|i| {
        if !h.contains_key(i) {
            h.insert(*i, *i);
        }
    });

    let mut unique: Vec<i32> = h.iter().map(|(_, i)| *i).collect();

    let max = v.iter().max().unwrap();
    let mut lowest_cost = i32::MAX;

    for val in 1..*max {
        let l = (-val).abs();

        let tmp_cost = v.clone().iter().fold(0, |acc, n| {
            let r = acc + sum_to((l - *n).abs());
            // println!("{}", r);

            r
        });

        if tmp_cost < lowest_cost {
            lowest_cost = tmp_cost;
        }
    }

    lowest_cost
}

#[cfg(test)]
mod tests {
    use shared::read_symbol_separated_items;

    use crate::{find_lowest_cost_1, find_lowest_cost_2};

    #[test]
    fn d07a() {
        let v = read_symbol_separated_items("data/demo.txt", ',').unwrap();
        let mut v: Vec<i32> = v.iter().map(|l| l.parse().unwrap()).collect();

        println!("score: {}", find_lowest_cost_1(&mut v));
    }

    #[test]
    #[ignore = "slowpoke"]
    fn d07b() {
        let v = read_symbol_separated_items("data/input.txt", ',').unwrap();
        let mut v: Vec<i32> = v.iter().map(|l| l.parse().unwrap()).collect();

        println!("score: {}", find_lowest_cost_2(&mut v));
    }
}

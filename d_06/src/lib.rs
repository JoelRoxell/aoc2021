use std::collections::HashMap;

pub fn read_data(file: &str) -> Vec<usize> {
    let lines =
        shared::read_symbol_separated_items(file, ',').expect("fail to read lanternfish input");

    lines.iter().map(|s| s.parse().unwrap()).collect()
}

fn get_cache_key(fish: &str, day: &str) -> String {
    format!("f{}d{}", fish, day)
}

pub fn calculate_day(
    fish: usize,
    day: usize,
    sum: usize,
    mem: &mut HashMap<String, usize>,
) -> usize {
    if day == 0 {
        return 1 + sum;
    }

    let tmr = day - 1;

    if fish == 0 {
        calculate_day(8, tmr, sum, mem) + calculate_day(6, tmr, sum, mem)
    } else {
        let fish_key = fish.to_string();
        let day_key = day.to_string();
        let key = get_cache_key(&fish_key, &day_key);
        let fish_memory = mem.get(&key);

        match fish_memory {
            Some(val) => *val,
            None => {
                let res = calculate_day(fish - 1, tmr, sum, mem);

                mem.insert(key, res);

                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{calculate_day, read_data};

    #[test]
    fn d05_a() {
        let d = read_data("data/demo.txt");
        let mut h = HashMap::new();
        let sum = d
            .iter()
            .fold(0, |acc, fish| acc + calculate_day(*fish, 18, 0, &mut h));

        assert_eq!(sum, 26);

        let mut h = HashMap::new();
        let sum = d
            .iter()
            .fold(0, |acc, fish| acc + calculate_day(*fish, 80, 0, &mut h));

        assert_eq!(sum, 5934);
    }

    #[test]
    fn d05a() {
        let d = read_data("data/input.txt");
        let mut h = HashMap::new();
        let sum = d
            .iter()
            .fold(0, |acc, fish| acc + calculate_day(*fish, 80, 0, &mut h));

        assert_eq!(sum, 353274);
    }

    #[test]
    fn d05b() {
        let d = read_data("data/input.txt");
        let mut sum = 0;
        let mut h = HashMap::new();

        for f in d {
            sum += calculate_day(f, 256, 0, &mut h);
        }

        assert_eq!(sum, 1609314870967)
    }

    #[test]
    fn read_data_test() {
        let d = read_data("data/demo.txt");

        assert_eq!(d.len(), 5)
    }
}

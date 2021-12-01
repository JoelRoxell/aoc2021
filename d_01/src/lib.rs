pub fn count_increases(measurements: Vec<usize>) -> usize {
    if measurements.is_empty() {
        return 0;
    }

    let mut last_measure = measurements[0];
    let mut n = 0;

    measurements.iter().skip(1).for_each(|ele| {
        let next = *ele;

        if next > last_measure {
            n += 1
        }

        last_measure = next;
    });

    n
}

pub fn increase_by_window(measurements: Vec<usize>, window: usize) -> Vec<usize> {
    let window = window - 1;

    measurements
        .iter()
        .enumerate()
        .skip(window)
        .map(|(i, &_n)| {
            let w = &measurements[i - window..=i];
            let position_sum: usize = w.iter().sum();

            position_sum
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{count_increases, increase_by_window};
    use shared::read_lines;
    use std::vec;

    #[test]
    #[ignore]
    fn d01_01() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increases(measurements), 7);
    }

    #[test]
    fn d01_1() {
        let lines = read_lines("./src/input.txt")
            .iter()
            .map(|f| f.parse().unwrap())
            .collect();

        assert_eq!(count_increases(lines), 1482);
    }

    #[test]
    #[ignore]
    fn d01_02() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let n = count_increases(increase_by_window(measurements, 3));

        assert_eq!(n, 5)
    }

    #[test]
    fn d01_2() {
        let entries = read_lines("./src/input.txt")
            .iter()
            .map(|f| f.parse().unwrap())
            .collect();
        let n = count_increases(increase_by_window(entries, 3));

        assert_eq!(n, 1518)
    }
}

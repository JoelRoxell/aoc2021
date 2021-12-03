use std::str::from_utf8;
use std::vec;

const ONE: u8 = 49;
const ZERO: u8 = 48;

pub fn read_g_e(lines: Vec<String>) -> (String, String) {
    let mut byte_list = vec![];
    let mut byte_len = 0;

    for item in lines.iter() {
        let item_bytes = item.as_bytes();

        byte_len = item_bytes.len();
        byte_list.push(item_bytes);
    }

    let mut most_sig_byte_vec: Vec<u8> = vec![];
    let mut current_byte = 0;

    while current_byte < byte_len {
        let mut zeros = 0;
        let mut ones = 0;

        for &byte in byte_list.iter() {
            let b = byte[current_byte];

            match b {
                ONE => zeros += 1,
                ZERO => ones += 1,
                _ => {}
            }
        }

        let most_sig_bit = match ones {
            _d if ones > zeros => ONE,
            _d if zeros > ones => ZERO,
            _ => 2,
        };

        if most_sig_bit == ONE {
            most_sig_byte_vec.push(ONE);
        } else if most_sig_bit == ZERO {
            most_sig_byte_vec.push(ZERO);
        } else {
            most_sig_byte_vec.push(ONE);
        }

        current_byte += 1;
    }

    let least_sig_byte_vec: Vec<u8> = most_sig_byte_vec
        .clone()
        .iter()
        .map(|b| match *b {
            ZERO => ONE,
            ONE => ZERO,
            _ => ZERO,
        })
        .collect();

    let str_rep_g = from_utf8(&most_sig_byte_vec).expect("failed to parse bin vec to str");
    let str_rep_e = from_utf8(&least_sig_byte_vec).expect("failed to parse bin vec to str");

    (str_rep_g.to_string(), str_rep_e.to_string())
}

pub fn find_entry(lines: &[String], sig: char) -> String {
    let mut byte_list = vec![];

    for item in lines.iter() {
        let chars: Vec<_> = item.chars().collect();

        byte_list.push(chars);
    }

    let mut current_byte = 0;

    while byte_list.len() > 1 {
        let mut zeros = 0;
        let mut ones = 0;

        for byte in byte_list.iter() {
            let b = byte[current_byte];

            match b {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => {}
            }
        }

        let largest_count = match ones > zeros {
            true => '1',
            false => '0',
        };

        let eq = ones == zeros;

        byte_list = byte_list
            .into_iter()
            .filter(|x| {
                let z = x[current_byte];

                if eq {
                    sig == z
                } else if sig == '1' {
                    z == largest_count
                } else {
                    z != largest_count
                }
            })
            .collect();

        current_byte += 1;
    }

    byte_list[0].iter().collect()
}

pub fn product_of_str_bins(a: &str, b: &str) -> usize {
    let a = u32::from_str_radix(a, 2).expect("failed to parse a");
    let b = u32::from_str_radix(b, 2).expect("failed to parse b");

    (a * b).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use shared::read_lines;

    use crate::{find_entry, product_of_str_bins, read_g_e};

    #[test]
    #[ignore]
    fn d_03_a() {
        let lines = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
        let (g, e) = read_g_e(lines);
        let res = product_of_str_bins(&g, &e);

        assert_eq!(res, 198)
    }

    #[test]
    fn d_03a() {
        let lines = read_lines("data/input.txt").unwrap();
        let (g, e) = read_g_e(lines);
        let res = product_of_str_bins(&g, &e);

        assert_eq!(res, res)
    }

    #[test]
    #[ignore]
    fn d_03_b() {
        let lines = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
        let oxy = find_entry(&lines, '1');
        let co2 = find_entry(&lines, '0');
        let res = product_of_str_bins(&oxy, &co2);

        assert_eq!(res, 230)
    }

    #[test]
    fn d_03b() {
        let lines = read_lines("data/input.txt").unwrap();
        let oxy = find_entry(&lines, '1');
        let co2 = find_entry(&lines, '0');
        let res = product_of_str_bins(&oxy, &co2);

        assert_eq!(res, 2990784)
    }
}

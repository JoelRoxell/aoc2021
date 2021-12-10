use std::collections::HashMap;

fn is_insert(c: &char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

pub fn rev(c: &char) -> char {
    match *c {
        ')' => '(',
        '(' => ')',
        ']' => '[',
        '[' => ']',
        '}' => '{',
        '{' => '}',
        '>' => '<',
        '<' => '>',
        _ => '_',
    }
}

pub fn parse_chunk(chunk: &str) -> Result<Vec<char>, usize> {
    let score_map: HashMap<char, usize> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut stack = vec![];

    for symbol in chunk.chars() {
        let is_insert = is_insert(&symbol);

        if is_insert {
            stack.push(symbol);

            continue;
        }

        let o = stack.pop().unwrap();
        let p = rev(&symbol);

        if o != p {
            let s = score_map.get(&symbol).unwrap();
            return Err(*s);
        }
    }

    Ok(stack.iter().map(|c| rev(c)).rev().collect())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parse_chunk;
    use shared::read_lines;

    #[test]
    fn single() {
        // match parse_chunk("(]") {
        //     Ok(_) => {}
        //     Err(n) => {
        //         assert_eq!(n, 57);
        //     }
        // }

        // match parse_chunk("{()()()>") {
        //     Ok(_) => {}
        //     Err(n) => {
        //         assert_eq!(n, 25137);
        //     }
        // }

        // match parse_chunk("{()()()}") {
        //     Ok(_) => {}
        //     Err(_) => {
        //         panic!("should't fail");
        //     }
        // }

        match parse_chunk("{()()(") {
            Ok(incomplete) => {
                println!("incomplete {:?}", incomplete);
                assert_eq!(incomplete[0], ')');
                assert_eq!(incomplete[1], '}');
            }
            Err(_) => {
                panic!("should't fail");
            }
        }
    }

    #[test]
    fn a() {
        let lines = read_lines("data/input.txt").unwrap();
        let mut errs = vec![];

        for line in lines {
            match parse_chunk(&line) {
                Ok(_) => {}
                Err(n) => errs.push(n),
            }
        }

        let res = errs.iter().sum::<usize>();

        assert_eq!(res, 316851)
    }

    #[test]
    fn b() {
        let score_map = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
        let lines = read_lines("data/input.txt").unwrap();
        let mut uncompleted = vec![];

        for line in lines {
            if let Ok(incomplete) = parse_chunk(&line) {
                uncompleted.push(incomplete)
            }
        }

        let mut points = vec![];
        let k = 5;

        for line in uncompleted {
            let mut sum: i64 = 0;

            for i in &line {
                let x = score_map.get(i).unwrap();

                sum = (sum * k) + *x;
            }

            points.push(sum);
            println!("{:?} = {}", line, sum);
        }

        points.sort_unstable();

        let middle = points.len() / 2;

        for p in &points {
            println!("{:?}", p)
        }

        println!("len {}, m {}", points.len(), middle);
        println!("{}", points[middle]);
    }
}

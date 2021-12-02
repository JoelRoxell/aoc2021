#[derive(Debug)]
pub struct Sub {
    horizontal: i32,
    depth: i32,
    aim: Option<i32>,
}

impl Sub {
    pub fn new(horizontal: i32, depth: i32, aim: Option<i32>) -> Self {
        Self {
            horizontal,
            depth,
            aim,
        }
    }

    pub fn forward(&mut self, n: i32) {
        self.horizontal += n;

        if let Some(aim) = self.aim {
            self.depth += aim * n
        }
    }

    pub fn down(&mut self, n: i32) {
        if let Some(aim) = self.aim {
            self.aim = Some(n + aim);
        } else {
            self.depth += n;
        }
    }

    pub fn up(&mut self, n: i32) {
        if let Some(aim) = self.aim {
            self.aim = Some(aim - n);
        } else {
            self.depth -= n;
        }
    }

    pub fn guide(&mut self, lines: Vec<String>) {
        lines.iter().for_each(|cmd| {
            let mut r = cmd.split(' ');
            let action = r.next().expect("failed to read action");
            let val = r.next().expect("failed to read val");
            let val: i32 = val.parse().expect("failed to parse val");

            match action {
                "forward" => self.forward(val),
                "down" => self.down(val),
                "up" => self.up(val),
                _ => panic!("failed to parse action: {}", action),
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::Sub;
    use shared::read_lines;

    #[test]
    #[ignore]
    fn d_02_a() {
        let data = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let mut s = Sub::new(0, 0, None);

        s.guide(data);

        assert_eq!(150, s.horizontal * s.depth)
    }

    #[test]
    fn d_02a() {
        let data = read_lines("data/input.txt").unwrap();
        let mut s = Sub::new(0, 0, None);

        s.guide(data);

        assert_eq!(1693300, s.horizontal * s.depth)
    }

    #[test]
    #[ignore]
    fn d_02_b() {
        let data = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let mut s = Sub::new(0, 0, Some(0));

        s.guide(data);

        assert_eq!(900, s.horizontal * s.depth)
    }

    #[test]
    fn d_02b() {
        let data = read_lines("data/input.txt").unwrap();
        let mut s = Sub::new(0, 0, Some(0));

        s.guide(data);

        assert_eq!(1857958050, s.depth * s.horizontal);
    }
}

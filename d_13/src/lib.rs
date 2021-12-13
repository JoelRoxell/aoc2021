use shared::read_lines;
use std::vec;

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }

    pub fn fold(self, index: usize, fold: &Fold) -> Self {
        if *fold == Fold::Y {
            let dist = self.y - ((self.y - index) * 2);

            Self { x: self.x, y: dist }
        } else {
            let dist = self.x - ((self.x - index) * 2);

            Self { x: dist, y: self.y }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Fold {
    X,
    Y,
}

type Field = Vec<Vec<Option<Coordinate>>>;

pub fn parse_field(filepath: &str) -> (Vec<Coordinate>, Vec<(Fold, usize)>) {
    let lines = read_lines(filepath).unwrap();
    let mut coordinates = vec![];
    let mut folds = vec![];
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        let l: Vec<_> = line.split(',').collect();
        let fold: Vec<_> = line.split(' ').collect();

        if l.len() == 2 {
            let x = l[0].parse::<usize>().unwrap();
            let y = l[1].parse::<usize>().unwrap();

            if x > max_x {
                max_x = x
            }

            if y > max_y {
                max_y = y
            }

            coordinates.push(Coordinate { x, y });
        } else if fold.is_empty() || fold.len() == 1 {
            continue;
        } else {
            let p = fold[2];
            let v: Vec<_> = p.split('=').collect();
            let direction = match v[0] {
                "y" => Fold::Y,
                "x" => Fold::X,
                _ => panic!("failed to parse fold string"),
            };
            let index = v[1].parse::<usize>().expect("failed to parse fold index");

            folds.push((direction, index));
        }
    }

    (coordinates, folds)
}

pub fn print_field(f: &Field, print: bool) -> usize {
    let mut sum = 0;

    for row in f {
        let mut row_rep = String::new();

        for c in row {
            row_rep.push_str(match c {
                Some(_) => {
                    sum += 1;

                    "#"
                }
                None => ".",
            })
        }

        if print {
            println!("{}", row_rep)
        }
    }

    if print {
        println!()
    }

    sum
}

pub fn build_field(coordinates: &[Coordinate]) -> Field {
    let max_y = coordinates.iter().map(|c| c.y).max().unwrap();
    let max_x = coordinates.iter().map(|c| c.x).max().unwrap();
    let mut field: Vec<Vec<Option<Coordinate>>> = vec![];

    for y in 0..=max_y {
        field.push(vec![]);

        for _ in 0..=max_x {
            field[y as usize].push(None);
        }
    }

    coordinates.iter().for_each(|c| field[c.y][c.x] = Some(*c));

    field
}

pub fn fold(coordinate: &[Coordinate], fold: Fold, index: usize) -> Vec<Coordinate> {
    let is_y_fold = fold == Fold::Y;
    let mut first_half: Vec<_> = coordinate
        .iter()
        .filter(|c| if is_y_fold { c.y < index } else { c.x < index })
        .copied()
        .collect();
    let second_half = coordinate
        .iter()
        .filter(|c| if is_y_fold { c.y > index } else { c.x > index });
    let mut new_coordinates: Vec<_> = second_half.map(|c| c.fold(index, &fold)).collect();

    first_half.append(&mut new_coordinates);

    first_half
}

#[cfg(test)]
mod tests {
    use crate::{build_field, fold, parse_field, print_field};

    #[test]
    fn a() {
        let (coordinates, folds) = parse_field("data/input.txt");
        let coordinates = folds
            .iter()
            .take(1)
            .fold(coordinates, |acc, (f, index)| fold(&acc, *f, *index));
        let field = build_field(&coordinates);
        let n = print_field(&field, false);

        assert_eq!(n, 765)
    }

    #[test]
    fn coordinate_sub() {
        let (coordinates, folds) = parse_field("data/input.txt");
        let coordinates = folds
            .iter()
            .fold(coordinates, |acc, (f, index)| fold(&acc, *f, *index));
        let field = build_field(&coordinates);

        print_field(&field, true);
    }
}

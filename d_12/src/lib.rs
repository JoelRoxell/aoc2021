use shared::read_lines;
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
    vec,
};

#[derive(Debug, PartialEq)]
pub enum Cavern {
    Small,
    Large,
}

#[derive(Debug)]
pub struct Node {
    value: String,
    size: Cavern,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(value: &str, size: Cavern, children: Vec<Rc<RefCell<Node>>>) -> Self {
        Self {
            size,
            value: value.to_string(),
            children,
        }
    }

    pub fn connect(&mut self, n: Rc<RefCell<Node>>) {
        let in_list = self
            .children
            .iter()
            .find(|c| c.borrow().value == n.borrow().value);

        if in_list.is_some() {
            return;
        }

        self.children.push(n);
    }

    pub fn print(&self) {
        println!("{} - {:?}", self.value, self.size);

        for c in &self.children {
            println!("\t>{}", c.borrow().value)
        }
    }
}

pub fn get_cave_type(c: char) -> Cavern {
    match c as usize {
        a if a < 97 => Cavern::Large,
        _ => Cavern::Small,
    }
}

pub fn insert_node(n: &str, m: &mut HashMap<String, Rc<RefCell<Node>>>) {
    m.entry(n.to_string()).or_insert_with(|| {
        Rc::new(RefCell::new(Node::new(
            n,
            get_cave_type(n.chars().next().unwrap()),
            vec![],
        )))
    });
}

pub fn parse_input(filepath: &str) -> HashMap<String, Rc<RefCell<Node>>> {
    let lines = read_lines(filepath).unwrap();
    let mut nodes: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();

    for l in lines {
        let entry: Vec<String> = l.split('-').map(|l| l.to_string()).collect();
        let parent = &entry[0];
        let child = &entry[1];

        insert_node(parent, &mut nodes);
        insert_node(child, &mut nodes);

        let child = nodes.get(child).unwrap();
        let parent = nodes.get(parent).unwrap();

        parent.borrow_mut().connect(Rc::clone(child));
        child.borrow_mut().connect(Rc::clone(parent));
    }

    nodes
}

pub fn explore(
    node: Ref<Node>,
    path: String,
    paths: &mut Vec<String>,
    explored: HashMap<String, usize>,
    allow_dup: bool,
) {
    let new_path = format!("{} {}", path, node.value);
    let mut visited_a_cavern_twice = false;

    if !allow_dup && explored.contains_key(&node.value)
        || (allow_dup && node.value == "start" && explored.contains_key("start"))
    {
        return;
    } else if allow_dup && node.value != "end" && Cavern::Small == node.size {
        for val in explored.values() {
            if *val > 1 {
                visited_a_cavern_twice = true;
                break;
            }
        }
    } else if node.value == "end" {
        paths.push(new_path);

        return;
    }

    let mut explored = explored;

    if let Cavern::Small = node.size {
        let r = explored.entry(node.value.clone()).or_insert(0);

        *r += 1;

        if visited_a_cavern_twice && *r > 1 {
            return;
        }
    }

    for child in &node.children {
        let c = child.borrow();

        explore(c, new_path.clone(), paths, explored.clone(), allow_dup);
    }
}

#[cfg(test)]
mod tests {
    use crate::{explore, parse_input};
    use std::collections::HashMap;

    #[test]
    fn d_12a() {
        let field = parse_input("data/input.txt");
        let explored = HashMap::new();
        let start = field.get("start").unwrap();
        let start = start.borrow();
        let mut paths = vec![];

        explore(start, String::new(), &mut paths, explored, false);

        assert_eq!(4241, paths.len());
    }

    #[test]
    fn d_12b() {
        let field = parse_input("data/input.txt");
        let explored = HashMap::new();
        let start = field.get("start").unwrap();
        let start = start.borrow();
        let mut paths = vec![];

        explore(start, String::new(), &mut paths, explored, true);

        assert_eq!(122134, paths.len());
    }
}

use crate::config::Config;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub mod config;

#[derive(Debug)]
struct Object {
    name: String,
    children: Vec<Box<&'static Object>>,
}

impl Object {
    fn new(name: String) -> Self {
        Self {
            name,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: &'static Object) {
        self.children.push(Box::new(child));
    }
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(config.filename)?;
    let orbits = parse_orbits(&input);

    match config.part {
        config::Part::PartOne => {
            let result = part_one(&orbits);
            println!("{}", result);
        }
        config::Part::PartTwo => {
            println!("Part two will go here");
        }
    }

    Ok(())
}

fn parse_orbits(input: &str) -> Vec<(String, String)> {
    let orbits = input
        .lines()
        .map(|s| {
            let orbit = s.split(")").map(|x| x.trim()).collect::<Vec<&str>>();
            (orbit[0], orbit[1])
        })
        .collect::<Vec<(&str, &str)>>();

    let mut node_map = HashMap::new();
    for (parent_name, child_name) in orbits {
        let child = node_map
            .entry(String::from(child_name))
            .or_insert(Object::new(String::from(child_name)));

//        let parent = node_map
//            .entry(String::from(parent_name))
//            .or_insert(Object::new(String::from(parent_name)));
//        parent.add_child(child);
    }

    input
        .lines()
        .map(|s| {
            let orbit = s.split(")").map(|x| x.trim()).collect::<Vec<&str>>();
            (String::from(orbit[0]), String::from(orbit[1]))
        })
        .collect::<Vec<(String, String)>>()
}

fn part_one(orbits: &Vec<(String, String)>) -> usize {
    let mut orbit_map = HashMap::new();

    for (parent, child) in orbits {
        let children = orbit_map.entry(parent).or_insert(Vec::new());
        children.push(child);
    }

    let mut level = 0;
    let mut count = 1;
    for (parent, children) in orbit_map {
        println!("{} ({}) => {:?}", parent, level, children);
        count += level + children.len() + 1;
        level += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_one_should_return_42() {
        let input = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";
        let orbits = parse_orbits(&input);

        assert_eq!(42, part_one(&orbits));
    }
}

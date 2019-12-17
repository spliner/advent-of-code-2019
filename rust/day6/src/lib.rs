use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(config.filename)?;


    match config.part {
        config::Part::PartOne => {
            let orbits = parse_orbits_part_one(&input);
            let result = part_one(&orbits);
            println!("{}", result);
        }
        config::Part::PartTwo => {
            let orbits = parse_orbits_part_two(&input);
            let result = part_two(&orbits);
            println!("{}", result);
        }
    }

    Ok(())
}

fn parse_orbits_part_one(input: &str) -> HashMap<String, Vec<String>> {
    let orbits = input
        .lines()
        .map(|s| {
            let orbit = s.split(")").map(|x| x.trim()).collect::<Vec<&str>>();
            (orbit[0], orbit[1])
        })
        .collect::<Vec<(&str, &str)>>();

    let mut node_map = HashMap::new();
    orbits.iter()
        .for_each(|(p, c)| {
            let parent_name = String::from(*p);
            let child_name = String::from(*c);
            let parent = node_map.entry(parent_name).or_insert(Vec::new());

            parent.push(child_name.clone());
        });

    node_map
}

fn part_one(orbits: &HashMap<String, Vec<String>>) -> usize {
    part_one_rec(&String::from("COM"), 0, orbits)
}

fn part_one_rec(parent_name: &String, current_distance: usize, orbits: &HashMap<String, Vec<String>>) -> usize {
    let children = orbits.get(parent_name);
    match children {
        Some(children) if children.len() > 0 => {
            let mut total = 0;
            for c in children {
                total += part_one_rec(c, current_distance + 1, orbits);
            }

            total + current_distance
        },
        _ => current_distance
    }
}

fn parse_orbits_part_two(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .map(|s| {
            let orbit = s.split(")").map(|x| x.trim()).collect::<Vec<&str>>();
            (orbit[1].into(), orbit[0].into())
        })
        .collect()
}

fn part_two(orbits: &HashMap<String, String>) -> usize {
    let my_path = get_path("COM", "YOU", orbits);
    let santa_path = get_path("COM", "SAN", orbits);

    let my_difference = my_path.difference(&santa_path);
    let santa_difference = santa_path.difference(&my_path);

    my_difference.count() - 1 + santa_difference.count() - 1
}

fn get_path(source: &str, destination: &str, orbits: &HashMap<String, String>) -> HashSet<String> {
    let mut path = HashSet::new();
    let mut current_node = destination;

    while current_node != source {
        path.insert(current_node.into());

        match orbits.get(current_node) {
            Some(n) => current_node = n,
            None => break,
        };
    }

    path.insert(source.into());

    path
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
        let orbits = parse_orbits_part_one(&input);

        assert_eq!(42, part_one(&orbits));
    }

    #[test]
    fn part_two_example_one_should_return_4() {
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
K)YOU
I)SAN";

        let orbits = parse_orbits_part_two(&input);

        assert_eq!(4, part_two(&orbits));
    }
}

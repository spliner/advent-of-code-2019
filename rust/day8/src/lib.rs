use std::error::Error;
use std::fs;

pub mod config;

#[derive(Debug)]
struct Layer {
    rows: Vec<Vec<i32>>
}

impl Layer {
    fn new() -> Self {
        Self {
            rows: Vec::new(),
        }
    }

    fn add_row(&mut self, rows: Vec<i32>) {
        self.rows.push(rows);
    }

    fn count_digit(&self, digit: i32) -> usize {
        self.rows.iter().flatten().filter(|d| **d == digit).count()
    }
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(config.filename)?;
    let layers = parse_layers(&input, 25, 6);

    match config.part {
        config::Part::PartOne => {
            println!("{}", part_one(&layers));
        }
        config::Part::PartTwo => {
            let image = part_two(&layers);
            for row in image {
                println!("{}", row);
            }
        }
    }

    Ok(())
}

fn parse_layers(input: &str, width: usize, height: usize) -> Vec<Layer> {
    let layer_total = width * height;
    let mut index = 0;

    let mut layers = Vec::new();

    while index * layer_total < input.len() {
        let mut layer = Layer::new();
        let chars = input
            .chars()
            .skip(index * layer_total)
            .take(layer_total)
            .collect::<Vec<char>>();
        let mut y = 0;
        while y < height {
            let row = chars
                .iter()
                .skip(y * width)
                .take(width)
                .map(|c| c.clone() as i32 - 48)
                .collect::<Vec<i32>>();
            layer.add_row(row);

            y += 1;
        }

        layers.push(layer);

        index += 1;
    }

    layers
}

fn part_one(layers: &Vec<Layer>) -> usize {
    let least_zeroes = layers.iter().min_by_key(|l| l.count_digit(0)).unwrap();
    least_zeroes.count_digit(1) * least_zeroes.count_digit(2)
}

fn part_two(layers: &Vec<Layer>) -> Vec<String> {
    let mut result = Vec::new();

    let first_layer = &layers[0];

    for row in &first_layer.rows {
        result.push(row.clone());
    }

    for layer in layers.iter().skip(1) {
        for y in 0..layer.rows.len() {
            let row = &layer.rows[y];
            for x in 0..row.len() {
                let current_digit = &result[y][x];
                if *current_digit != 2 {
                    continue;
                }

                result[y][x] = layer.rows[y][x].clone();
            }
        }
    }

    result.iter()
        .map(|r| {
            r.iter()
                .map(|i| {
                    if *i == 0 {
                        " "
                    } else {
                        "*"
                    }
                })
                .collect::<String>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_layers_test() {
        let input = "123456789012";

        let layers = parse_layers(input, 3, 2);

        assert_eq!(2, layers.len());
        assert_eq!(2, layers[0].rows.len());
        assert_eq!(vec![1, 2, 3], layers[0].rows[0]);
        assert_eq!(vec![4, 5, 6], layers[0].rows[1]);

        assert_eq!(2, layers[1].rows.len());
        assert_eq!(vec![7, 8, 9], layers[1].rows[0]);
        assert_eq!(vec![0, 1, 2], layers[1].rows[1]);
    }

    #[test]
    fn part_two_test() {
        let layers = parse_layers("0222112222120000", 2, 2);
        let result = part_two(&layers);
        assert_eq!(" *", result[0]);
        assert_eq!("* ", result[1]);
    }
}

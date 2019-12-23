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

    match config.part {
        config::Part::PartOne => {
            let layers = parse_layers(&input, 25, 6);
            println!("{}", part_one(&layers));
        }
        config::Part::PartTwo => {
            // TODO: Part two
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
}

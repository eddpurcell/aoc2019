use std::fs::File;
use std::io::{BufReader, BufRead};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() -> Result<(), std::io::Error> {
    let input = File::open("res/input.txt")?;
    let mut buffered = BufReader::new(input);
    let mut data = String::new();
    buffered.read_line(&mut data)?;
    let data = data.trim_end();

    let layers = parse(data);

    let mut sorted_layers = layers.clone();
    let fewest_zero_layer = fewest_zeroes(&mut sorted_layers);
    println!("Part 1: {}", num_value(fewest_zero_layer, 1) * num_value(fewest_zero_layer, 2));

    let mut actual_image = [0u32; 150];
    println!("{:?}", all_values_for_pos(&layers, 148));
    for pos in 0..WIDTH*HEIGHT {
        actual_image[pos] = *all_values_for_pos(&layers, pos).iter().find(|&&x| x != 2).unwrap();
    }

    print_layer(actual_image);

    Ok(())
}

type Layer = [u32; 150];

fn parse(input: &str) -> Vec<Layer> {
    let n = WIDTH * HEIGHT;
    let mut layers: Vec<Layer> = Vec::new();
    let mut start = 0;
    while start < input.len() {
        let mut layer = [0u32; 150];
        for (pos, c) in input[start..start + n].char_indices() {
            layer[pos] = c.to_digit(10).unwrap();
        }
        layers.push(layer);
        start += n;
    }
    layers
}

fn fewest_zeroes(layers: &mut Vec<Layer>) -> &Layer {
    layers.sort_by(|a, b| num_value(a, 0).partial_cmp(&num_value(b, 0)).unwrap());
    layers.first().unwrap()
}

fn num_value(layer: &Layer, value: u32) -> u32 {
    layer.iter().filter(|n| **n == value).count() as u32
}

fn all_values_for_pos(layers: &Vec<Layer>, pos: usize) -> Vec<u32> {
    layers.iter().map(|layer| layer[pos]).collect()
}

fn print_layer(layer: Layer) {
    for pos in 0..WIDTH*HEIGHT {
        if pos % 25 == 0 {
            println!("");
        }
        print!("{}", u32_to_char(layer[pos]));
    }
}

fn u32_to_char(c: u32) -> char {
    match c {
        0 => ' ',
        1 => '#',
        _ => '~',
    }
}

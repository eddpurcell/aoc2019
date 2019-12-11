use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "res/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut masses: Vec<i64> = Vec::new();

    for line in reader.lines() {
        masses.push(line.unwrap().parse::<i64>().unwrap());
    }

    println!("Day 1 first: {}", first_star(&masses));
    println!("Day 1 second: {}", second_star(&masses, 0));
}

fn first_star(masses: &Vec<i64>) -> i64 {
    masses.iter().map(|mass| (mass / 3 ) - 2).sum()
}

fn second_star(masses: &Vec<i64>, running_sum: i64) -> i64 {
    if masses.iter().sum::<i64>() == 0i64 {
        return running_sum;
    }
    let new_masses: Vec<i64> = masses.iter().map(|mass| (mass / 3) - 2).filter(|mass| mass.is_positive()).collect();
    second_star(&new_masses, running_sum + new_masses.iter().sum::<i64>())
}


extern crate num;

mod mass;

use std::cell::RefCell;

use num::integer::lcm;

use crate::mass::{Mass, Vector};

fn main() {
    let moons = vec![
        RefCell::new(Mass::new((-6, 2, -9))),
        RefCell::new(Mass::new((12, -14, -4))),
        RefCell::new(Mass::new((9, 5, -6))),
        RefCell::new(Mass::new((-1, -4, 9))),
    ];

    for _ in 0..1000 {
        step(&moons);
    }
    println!("Part 1: {}", moons.iter().map(|m| m.borrow().total_energy()).sum::<i64>());

    let moons = vec![
        RefCell::new(Mass::new((-6, 2, -9))),
        RefCell::new(Mass::new((12, -14, -4))),
        RefCell::new(Mass::new((9, 5, -6))),
        RefCell::new(Mass::new((-1, -4, 9))),
    ];
    let mut x_rep = None;
    let mut y_rep = None;
    let mut z_rep = None;
    let init_xs: Vec<i64> = moons.iter().map(|m| m.borrow().pos.0).collect();
    let init_ys: Vec<i64> = moons.iter().map(|m| m.borrow().pos.1).collect();
    let init_zs: Vec<i64> = moons.iter().map(|m| m.borrow().pos.2).collect();
    let mut step_count = 1i128;
    while x_rep.is_none() || y_rep.is_none() || z_rep.is_none() {
        step(&moons);
        step_count += 1;
        if x_rep == None && moons.iter().map(|m| m.borrow().pos.0).collect::<Vec<i64>>() == init_xs && moons.iter().map(|m| m.borrow().vel.0).sum::<i64>() == 0 {
            x_rep = Some(step_count);
        }
        if y_rep == None && moons.iter().map(|m| m.borrow().pos.1).collect::<Vec<i64>>() == init_ys && moons.iter().map(|m| m.borrow().vel.1).sum::<i64>() == 0 {
            y_rep = Some(step_count);
        }
        if z_rep == None && moons.iter().map(|m| m.borrow().pos.2).collect::<Vec<i64>>() == init_zs && moons.iter().map(|m| m.borrow().vel.2).sum::<i64>() == 0 {
            z_rep = Some(step_count);
        }
    }
    println!("{}, {}, {}", x_rep.unwrap(), y_rep.unwrap(), z_rep.unwrap());
    
    println!("Part 2: {}", lcm(x_rep.unwrap(), lcm(y_rep.unwrap(), z_rep.unwrap())));
}

fn step(moons: &Vec<RefCell<Mass>>) {
    for moon in moons.iter() {
        for other_pos in get_other_positions(moons, moon).iter() {
            moon.borrow_mut().apply_gravity(*other_pos);
        }
    }
    for moon in moons.iter() {
        moon.borrow_mut().apply_velocity();
    }
}

fn get_other_positions(moons: &Vec<RefCell<Mass>>, moon: &RefCell<Mass>) -> Vec<Vector> {
    moons.iter().filter(|&m| m != moon).map(|m| m.borrow().pos).collect()
}

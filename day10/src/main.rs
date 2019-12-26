use std::fs::File;
use std::io::Read;

extern crate num;

use crate::num::Integer;

fn main() -> Result<(), std::io::Error> {
    let mut asteroid_map = parse_input()?;

    println!("Num asteroids: {}", asteroid_map.iter().map(|v| v.iter().filter(|o| o.is_some()).count()).sum::<usize>());

    let width = asteroid_map[0].len();
    let height = asteroid_map.len();
    let slopes = uniq_slopes(width, height);

    for y in 0..height {
        for x in 0..width {
            if let Some(_) = asteroid_map[y][x] {
                asteroid_map[y][x] = Some(slopes.iter().map(|s| {
                    if is_asteroid_on_slope(&asteroid_map, *s, (x, y)) {
                        1
                    } else {
                        0
                    }
                }).sum());
            }
        }
    }

    let best_asteroid_count = asteroid_map.iter().map(|v| v.iter().map(|o| match o { Some(n) => *n, None => 0, }).max().unwrap()).max().unwrap();
    println!("Part 1: {}", best_asteroid_count);

    let y = asteroid_map.iter().position(|v| v.contains(&Some(best_asteroid_count))).unwrap();
    let x = asteroid_map[y].iter().position(|&o| o == Some(best_asteroid_count)).unwrap();
    let pos = (x as usize, y as usize);

    let mut n = 0;
    loop {
        slopes.iter().for_each(|&s| {
            match first_asteroid_on_slope(&asteroid_map, s, pos) {
                Some(p) => {
                    asteroid_map[p.1][p.0] = None;
                    n += 1;
                    if n == 200 {
                        println!("Part 2: {}", p.0 * 100 + p.1);
                    }
                },
                None => (),
            };
        });
        if n > 200 {
            break;
        }
    }

    Ok(())
}

fn parse_input() -> Result<Vec<Vec<Option<u32>>>, std::io::Error> {
    let mut file = File::open("res/input.txt")?;
    let mut map_text = String::new();
    file.read_to_string(&mut map_text)?;

    let mut asteroid_map: Vec<Vec<Option<u32>>> = Vec::new();

    let mut line_num = 0;
    for line in map_text.lines() {
        asteroid_map.push(Vec::new());
        for c in line.chars() {
            match c {
                '#' => asteroid_map[line_num].push(Some(0)),
                '.' => asteroid_map[line_num].push(None),
                c => panic!("What is this input??? {}", c),
            };
        }
        line_num += 1;
    }

    Ok(asteroid_map)
}

type Slope = (i32, i32);

fn uniq_slopes(width: usize, height:usize) -> Vec<Slope> {
    let mut slopes = uniq_slopes_octant(width, height);
    slopes.append(&mut mirror_octant(&slopes));
    slopes.push((1,1));
    let mut x_mir = mirror_x(&slopes);
    let mut y_mir = mirror_y(&slopes);
    let mut xy_mir = mirror_xy(&slopes);
    slopes.append(&mut x_mir);
    slopes.append(&mut y_mir);
    slopes.append(&mut xy_mir);
    slopes.sort_by(|a, b| better_atan2(a).partial_cmp(&better_atan2(b)).unwrap());
    slopes
}

fn uniq_slopes_octant(width: usize, height: usize) -> Vec<Slope> {
    let mut slopes = vec![(1,0)];
    for y in 1..height {
        for x in y + 1..width {
            if x.gcd(&y) == 1 {
                slopes.push((x as i32, y as i32));
            }
        }
    }
    slopes
}

fn mirror_octant(slopes: &Vec<Slope>) -> Vec<Slope> {
    slopes.iter().map(|s| (s.1, s.0)).collect()
}

fn mirror_y(slopes: &Vec<Slope>) -> Vec<Slope> {
    slopes.iter().filter(|s| s.0 != 0).map(|s| (-s.0, s.1)).collect()
}

fn mirror_x(slopes: &Vec<Slope>) -> Vec<Slope> {
    slopes.iter().filter(|s| s.1 != 0).map(|s| (s.0, -s.1)).collect()
}

fn mirror_xy(slopes: &Vec<Slope>) -> Vec<Slope> {
    mirror_y(&mirror_x(slopes))
}

fn is_asteroid_on_slope(asteroid_map: &Vec<Vec<Option<u32>>>, slope: Slope, pos: (usize, usize)) -> bool {
    match first_asteroid_on_slope(asteroid_map, slope, pos) {
        Some(_) => true,
        None => false,
    }
}

fn first_asteroid_on_slope(asteroid_map: &Vec<Vec<Option<u32>>>, slope: Slope, pos: (usize, usize)) -> Option<(usize, usize)> {
    let mut search_pos = (pos.0 as i32, pos.1 as i32);
    loop {
        search_pos = (search_pos.0 + slope.0, search_pos.1 + slope.1);
        if search_pos.1 < 0 { break; }
        match asteroid_map.get(search_pos.1 as usize) {
            Some(v) => {
                if search_pos.0 < 0 { break; }
                match v.get(search_pos.0 as usize) {
                    Some(Some(_)) => {
                        return Some((search_pos.0 as usize, search_pos.1 as usize));
                    },
                    Some(None) => continue,
                    None => break,
                };
            },
            None => break,
        }
    }
    None
}

fn _print_asteroid_map(map: &Vec<Vec<Option<u32>>>) {
    map.iter().for_each(|v| {
        v.iter().for_each(|x| match x { Some(n) => print!("{}|", n), None => print!(". "), });
        println!("");
    });
}

fn better_atan2(a: &(i32, i32)) -> f64 {
    let p = (a.0 as f64, a.1 as f64);
    if p.0 == 0.0 && p.1 == -1.0 {
        0.0
    } else if p.0.is_sign_positive() && p.1.is_sign_positive() {
        p.0.atan2(-p.1)
    } else if p.0.is_sign_positive() && p.1.is_sign_negative() {
        p.0.atan2(-p.1)
    } else if p.0.is_sign_negative() && p.1.is_sign_negative() {
        -p.1.atan2(-p.0) + std::f64::consts::PI + std::f64::consts::PI / 2.0
    } else {
        -p.0.atan2(p.1) + std::f64::consts::PI
    }
}

#[cfg(test)]
mod test {
    use super::*;

    ///
    ///  0123456789
    /// 0~         
    /// 1 ~xxxxxxxx
    /// 2  ~x x x x
    /// 3   ~xx xx 
    /// 4    ~x x x
    /// 5     ~xxxx
    /// 6      ~x  
    /// 7       ~xx
    /// 8        ~x
    /// 9         ~
    #[test]
    fn octant_gen_is_accurate() {
        let expected = vec![
            (1,0),
            (2,1),(3,1),(4,1),(5,1),(6,1),(7,1),(8,1),(9,1),
            (3,2),(5,2),(7,2),(9,2),
            (4,3),(5,3),(7,3),(8,3),
            (5,4),(7,4),(9,4),
            (6,5),(7,5),(8,5),(9,5),
            (7,6),
            (8,7),(9,7),
            (9,8),
        ];
        assert_eq!(expected, uniq_slopes_octant(10,10));
    }

    #[test]
    fn octant_mirror_is_accurate() {
        let expected = vec![
            (0,1),
            (1,2),(1,3),(1,4),(1,5),(1,6),(1,7),(1,8),(1,9),
            (2,3),(2,5),(2,7),(2,9),
            (3,4),(3,5),(3,7),(3,8),
            (4,5),(4,7),(4,9),
            (5,6),(5,7),(5,8),(5,9),
            (6,7),
            (7,8),(7,9),
            (8,9),
        ];
        assert_eq!(expected, mirror_octant(&uniq_slopes_octant(10,10)));
    }
}

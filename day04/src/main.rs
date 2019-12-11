fn main() {
    let passwords: Vec<i32> = (165432..707912_i32).filter(|&n| digits_never_decrease(n) && two_adjacent_digits_are_same(n)).collect();
    println!("Part 1: {}", passwords.len());
    let passwords: Vec<i32> = (165432..707912_i32).filter(|&n| digits_never_decrease(n) && adj_digits_rule_2(n)).collect();
    println!("Part 2: {}", passwords.len());
}

fn two_adjacent_digits_are_same(n: i32) -> bool {
    let as_str = n.to_string();
    for pair in pairs(&as_str[..]) {
        if pair[0..1] == pair[1..2] {
            return true;
        }
    }
    false
}

fn adj_digits_rule_2(n: i32) -> bool {
    let as_str = n.to_string();
    let mut groups = groups(&as_str[..]);
    groups.retain(|g| g.len() == 2);
    !groups.is_empty()
}

fn digits_never_decrease(n: i32) -> bool {
    let as_str = n.to_string();
    let mut last_digit = 0;
    for digit in as_str.chars().map(|c| c.to_digit(10).unwrap()) {
        if digit < last_digit {
            return false;
        }
        last_digit = digit;
    }
    true
}

fn pairs(s: &str) -> Vec<&str> {
    let mut pairs = Vec::new();
    for i in 0..s.len() - 1 {
        pairs.push(&s[i..i+2]);
    }
    pairs
}

fn groups(s: &str) -> Vec<&str> {
    let mut groups = Vec::new();
    let mut start_ind = 0;
    let mut end_ind = 1;
    while end_ind < s.len() {
        let next_char = &s[end_ind..end_ind + 1];
        if next_char == &s[start_ind..start_ind + 1] {
            end_ind += 1;
        } else {
            groups.push(&s[start_ind..end_ind]);
            start_ind = end_ind;
        }
    }
    groups.push(&s[start_ind..end_ind]);
    groups
}
        

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn adjacent_digits_works() {
        assert!(two_adjacent_digits_are_same(122345));
        assert_eq!(false, two_adjacent_digits_are_same(12345));
    }

    #[test]
    fn digits_never_decrease_works() {
        assert!(digits_never_decrease(12345));
        assert_eq!(false, digits_never_decrease(12343));
    }

    #[test]
    fn adjacent_groups_works() {
        assert!(adj_digits_rule_2(112233));
        assert!(adj_digits_rule_2(111122));
        assert_eq!(false, adj_digits_rule_2(123444));
    }
}

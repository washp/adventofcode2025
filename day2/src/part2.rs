use core::panic;

use itertools::Itertools;

#[allow(unused_variables)]
fn parse(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.split(',').map(|x| {
        x.split('-')
            .map(|x| {
                x.trim()
                    .parse::<u64>()
                    .expect("Failed to convert to number")
            })
            .collect_tuple::<(u64, u64)>()
            .expect("Failed to collect range tuple")
    })
}
fn verify_id(num_id: u64) -> bool {
    let id = num_id.to_string();
    for i in 1..(id.len() / 2) + 1 {
        if !id.len().is_multiple_of(i) {
            continue;
        }
        let repeated_id = id.split_at(i).0.repeat(id.len() / i);
        if repeated_id.len() != id.len() {
            panic!("Sanity check! id:s cannot be different length!")
        }
        if id == repeated_id {
            return true;
        }
    }
    false
}

fn verify_all_id<T: Iterator<Item = (u64, u64)>>(input: T) -> u64 {
    let mut score = 0;
    for range in input {
        for num_id in range.0..range.1 + 1 {
            if verify_id(num_id) {
                score += num_id;
            }
        }
    }
    score
}

#[allow(unused_variables)]
pub fn run(input: &str) -> u64 {
    let parsed = parse(input);
    verify_all_id(parsed)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part2() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 4174379265);
    }
}

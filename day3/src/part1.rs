fn get_joltage(input: &str) -> u8 {
    let mut first = '0';
    let mut second = '0';
    for (i, c) in input.trim().chars().rev().enumerate() {
        if i == 0 {
            second = c;
            continue;
        }
        if c >= first {
            if first > second {
                second = first;
            }
            first = c;
            continue;
        }
    }
    (first.to_digit(10).expect("First is not a number") * 10
        + second.to_digit(10).expect("Second is not a number"))
    .try_into()
    .expect("Overflow on Joltage number!")
}

#[allow(unused_variables)]
pub fn run(input: &str) -> u16 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut score = 0;
    for line in lines {
        score += get_joltage(line) as u16;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 357);
    }
}

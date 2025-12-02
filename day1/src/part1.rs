use std::str::Lines;

#[allow(unused_variables)]
fn parse(lines: Lines) -> impl Iterator<Item = (char, u16)> {
    lines.map(|s| {
        let mut iter = s.chars();
        (
            iter.next().expect("No Direction in Row"),
            iter.as_str().parse::<u16>().expect("No value in Row"),
        )
    })
}

fn execute_turns<T: Iterator<Item = (char, u16)>>(rows: T) -> u16 {
    let mut dial = 50;
    let mut score = 0;
    for row in rows {
        dial += match row {
            ('L', n) => -(n as i32),
            ('R', n) => n as i32,
            _ => 0,
        };
        if dial < 0 {
            dial += (dial.abs() / 100) * 100;
        }
        if dial > 99 {
            dial -= (dial / 100) * 100;
        }
        if dial == 0 {
            score += 1;
        }
    }
    score
}

#[allow(unused_variables)]
pub fn run(input: &str) -> u16 {
    let lines = input.lines();
    let rows = parse(lines);
    execute_turns(rows)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_part1() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 3);
    }
}

use std::str::Lines;

#[allow(unused_variables)]
fn parse(lines: Lines) -> impl Iterator<Item = (char, u16)> {
    lines.map(|s| {
        let mut iter = s.trim().chars();
        (
            iter.next().expect("No Direction in Row"),
            iter.as_str().parse::<u16>().expect("No value in Row"),
        )
    })
}

fn execute_turns<T: Iterator<Item = (char, u16)>>(rows: T) -> u32 {
    let mut dial = 50;
    let mut score = 0;
    for row in rows {
        let start_at_zero = dial == 0;
        dial += match row {
            ('L', n) => -(n as i32),
            ('R', n) => n as i32,
            _ => 0,
        };
        if dial < 0 {
            let ticks = (dial.abs() / 100) + 1;
            score += ticks;
            if start_at_zero {
                score -= 1;
            }
            if dial % 100 == 0 {
                dial += (ticks - 1) * 100;
            } else {
                dial += ticks * 100;
            }
        } else if dial > 99 {
            let ticks = dial / 100;
            dial -= ticks * 100;
            score += ticks;
        } else if dial == 0 {
            score += 1;
        }
    }
    score as u32
}

#[allow(unused_variables)]
pub fn run(input: &str) -> u32 {
    let lines = input.lines();
    let rows = parse(lines);
    execute_turns(rows)
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn test_example_input() {
        let example_content = fs::read_to_string("example_input1.txt").unwrap();
        assert_eq!(run(&example_content), 6);
    }
    #[test]
    fn test_rotate_left() {
        let mut example_content = " L50
                                    L50";
        assert_eq!(run(example_content), 1);
        example_content = " L50
                            L100";
        assert_eq!(run(example_content), 2);
        example_content = " L25
                            L100";
        assert_eq!(run(example_content), 1);
        example_content = " L250";
        assert_eq!(run(example_content), 3);
        example_content = " L50
                            L200";
        assert_eq!(run(example_content), 3);
        example_content = " L150
                            L50";
        assert_eq!(run(example_content), 2);
    }
    #[test]
    fn test_rotate_right() {
        let mut example_content = " R50";
        assert_eq!(run(example_content), 1);
        example_content = " R100";
        assert_eq!(run(example_content), 1);
        example_content = " R150";
        assert_eq!(run(example_content), 2);
        example_content = " R50
                            R50";
        assert_eq!(run(example_content), 1);
        example_content = " R50
                            R100";
        assert_eq!(run(example_content,), 2);
        example_content = " R50
                            R150";
        assert_eq!(run(example_content), 2);
        example_content = " R50
                            R300";
        assert_eq!(run(example_content), 4);
        example_content = " R150
                            R50";
        assert_eq!(run(example_content), 2);
    }
    #[test]
    fn test_rotate_mix() {
        let mut example_content = " R150
                                    L50";
        assert_eq!(run(example_content), 2);
        example_content = " L150
                            R50";
        assert_eq!(run(example_content), 2);
    }
}

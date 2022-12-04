use std::str::Lines;

fn main() {
    let mut total = 0u32;
    let mut total_stickers = 0u32;

    for line in include_str!("./input.txt").lines() {
        let compartments = read_compartments(line);
        let c = find_common(compartments);
        total += priority(c);
    }

    let mut lines: Lines<'static> = include_str!("./input.txt").lines();
    while let Some(group) = read_group(&mut lines) {
        let c = find_common_three(group);
        total_stickers += priority(c);
    }

    println!("total: {}", total);
    println!("total stickers: {}", total_stickers);
}

fn priority(c: char) -> u32 {
    // a-z => 97-122
    // A-Z => 65-90

    let raw = c as u32;

    if raw >= 97 {
        raw - 96
    } else {
        raw - 38
    }
}

fn read_compartments(line: &'static str) -> (&str, &str) {
    let len = line.len();
    let half = len / 2;
    let comp1 = &line[..half];
    let comp2 = &line[half..];
    (comp1, comp2)
}

fn find_common((comp1, comp2): (&str, &str)) -> char {
    for c1 in comp1.chars() {
        for c2 in comp2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    }

    panic!("failed to find a common item")
}

fn read_group(lines: &mut Lines<'static>) -> Option<(&'static str, &'static str, &'static str)> {
    Some((lines.next()?, lines.next()?, lines.next()?))
}

fn find_common_three((line1, line2, line3): (&str, &str, &str)) -> char {
    for c1 in line1.chars() {
        for c2 in line2.chars() {
            for c3 in line3.chars() {
                if c1 == c2 && c2 == c3 {
                    return c1;
                }
            }
        }
    }

    panic!("failed to find a common item")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn priority_test() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('b'), 2);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('B'), 28);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn read_test() {
        assert_eq!(
            read_compartments("vJrwpWtwJgWrhcsFMMfFFhFp"),
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
        );
        assert_eq!(
            read_compartments("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")
        );
    }

    #[test]
    fn common_test() {
        assert_eq!(find_common(("vJrwpWtwJgWr", "hcsFMMfFFhFp")), 'p');
        assert_eq!(find_common(("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")), 'L');
    }

    #[test]
    fn common_three_test() {
        assert_eq!(
            find_common_three((
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            )),
            'r'
        );
        assert_eq!(
            find_common_three((
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            )),
            'Z'
        );
    }

    #[test]
    fn read_group_test() {
        let mut lines = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
            .lines();

        assert_eq!(
            read_group(&mut lines),
            Some((
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ))
        );
        assert_eq!(
            read_group(&mut lines),
            Some((
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ))
        );
        assert_eq!(read_group(&mut lines), None);
    }
}

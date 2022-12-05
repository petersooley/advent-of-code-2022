#[derive(Debug, PartialEq, Eq)]
struct Stacks([String; 9]);

impl Stacks {
    fn new() -> Self {
        Self([
            "BQC".into(),
            "RQWZ".into(),
            "BMRLV".into(),
            "CZHVTW".into(),
            "DZHBNVG".into(),
            "HNPCJFVQ".into(),
            "DGTRWZS".into(),
            "CGMNBWZP".into(),
            "NJBMWQFP".into(),
        ])
    }

    fn update_9000(&mut self, m: &Move) {
        for _ in  0..m.qty {
            let popped = self.0[m.from - 1].pop().expect("invalid move");
            self.0[m.to - 1].push(popped);
        }
    }

    fn update_9001(&mut self, m: &Move) {
        let from = &mut self.0[m.from - 1];
        let popped = from.split_off(from.len() - m.qty);
        self.0[m.to - 1].push_str(popped.as_str());
    }

    fn into_tops(mut self) -> String {
        let mut out = String::default();
        for i in 0..9 {
            out.push(self.0[i].pop().unwrap_or_default());
        }
        out
    }
}

struct Move {
    qty: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(line: &'static str) -> Option<Self> {
        if let Some(remainder) = line.strip_prefix("move ") {
            let mut parts = remainder.split_ascii_whitespace();
            let qty: usize = parts
                .next()
                .expect("invalid line")
                .parse()
                .expect("invalid move qty");
            let _from = parts.next().expect("invalid line");
            let from: usize = parts
                .next()
                .expect("invalid line")
                .parse()
                .expect("invalid move from");
            let _to = parts.next().expect("invalid line");
            let to: usize = parts
                .next()
                .expect("invalid line")
                .parse()
                .expect("invalid move to");

            assert!(to <= 9);
            assert!(from <= 9);

            return Some(Self { qty, from, to });
        }
        None
    }
}

fn main() {
    let mut stacks9000 = Stacks::new();
    let mut stacks9001 = Stacks::new();
    for current_move in include_str!("./input.txt").lines().filter_map(Move::new) {
        stacks9000.update_9000(&current_move);
        stacks9001.update_9001(&current_move);
    }

    println!("9000: {}", stacks9000.into_tops());
    println!("9001: {}", stacks9001.into_tops());
}

#[cfg(test)]
mod test {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn test_9000() {
        let mut s = Stacks([
            "ZN".into(), "MCD".into(), "P".into(),
             "".into(), "".into(), "".into(), "".into(), "".into(), "".into(),
        ]);

        s.update_9000(&Move::new("move 1 from 2 to 1").unwrap());

        let expected = Stacks([
            "ZND".into(), "MC".into(), "P".into(),
            "".into(), "".into(), "".into(), "".into(), "".into(), "".into(),
        ]);

        assert_eq!(s, expected);

        s.update_9000(&Move::new("move 3 from 1 to 3").unwrap());

        let expected = Stacks([
            "".into(), "MC".into(), "PDNZ".into(),
            "".into(), "".into(), "".into(), "".into(), "".into(), "".into(),
        ]);

        assert_eq!(s, expected);

        s.update_9000(&Move::new("move 2 from 2 to 1").unwrap());

        let expected = Stacks([
            "CM".into(), "".into(), "PDNZ".into(),
            "".into(), "".into(), "".into(), "".into(), "".into(), "".into(),
        ]);

        assert_eq!(s, expected);

        println!("{}", s.into_tops());
    }
    #[rustfmt::skip]
    #[test]
    fn test_9001() {
        let mut s = Stacks([
            "ZN".into(), "MCD".into(), "P".into(),
             "".into(), "".into(), "".into(), "".into(), "".into(), "".into(),
        ]);

        s.update_9001(&Move::new("move 1 from 2 to 1").unwrap());

        let expected = Stacks([
            "ZND".into(), "MC".into(), "P".into(),
            "".into(), "".into(), "".into(), "".into(), "".into(), "".into(),
        ]);

        assert_eq!(s, expected);

        s.update_9001(&Move::new("move 3 from 1 to 3").unwrap());

        let expected = Stacks([
            "".into(), "MC".into(), "PZND".into(),
            "".into(), "".into(), "".into(), "".into(), "".into(), "".into(),
        ]);

        assert_eq!(s, expected);

        s.update_9001(&Move::new("move 2 from 2 to 1").unwrap());

        let expected = Stacks([
            "MC".into(), "".into(), "PZND".into(),
            "".into(), "".into(), "".into(), "".into(), "".into(), "".into(),
        ]);

        assert_eq!(s, expected);

        println!("{}", s.into_tops());
    }
}

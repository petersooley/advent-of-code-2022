use std::str;

#[derive(Debug)]
struct AssignmentRange {
    min: u32,
    max: u32,
}

impl AssignmentRange {
    fn contains(&self, other: &Self) -> bool {
        self.max <= other.max && self.min >= other.min
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

impl str::FromStr for AssignmentRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let or_e = || "invalid range syntax".to_string();
        let map_e = |_| "invalid range syntax".to_string();
        let min_str = parts.next().ok_or_else(or_e)?;
        let max_str = parts.next().ok_or_else(or_e)?;
        let min: u32 = min_str.parse().map_err(map_e)?;
        let max: u32 = max_str.parse().map_err(map_e)?;
        Ok(Self { min, max })
    }
}

fn main() {
    let mut contains_count = 0u32;
    let mut overlaps_count = 0u32;

    for line in include_str!("./input.txt").lines() {
        let mut parts = line.split(',');
        let a: AssignmentRange = parts.next().expect("invalid line").parse().unwrap();
        let b: AssignmentRange = parts.next().expect("invalid line").parse().unwrap();

        // println!("a {:?}, b {:?}, overlaps: {}", a, b, a.overlaps(&b));

        if a.contains(&b) || b.contains(&a) {
            contains_count += 1;
        }

        if a.overlaps(&b) {
            overlaps_count += 1;
        }
    }

    println!("contains {}", contains_count);
    println!("overlaps {}", overlaps_count);
}

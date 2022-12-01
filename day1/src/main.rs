use std::collections::BinaryHeap;

fn main() {
    let mut totals = BinaryHeap::default();
    let mut current_elfs_total = 0usize;
    for line in include_str!("./input.txt").lines() {
        if line.is_empty() {
            totals.push(current_elfs_total);
            current_elfs_total = 0;
        } else {
            current_elfs_total += line.parse::<usize>().expect("failed to convert line to usize");
        }
    }

    let most = totals.pop().unwrap();
    let second = totals.pop().unwrap();
    let third = totals.pop().unwrap();
    println!("most calories an elf is carring: {}", most);

    println!("top three {}", most + second + third);

}
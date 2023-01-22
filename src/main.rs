use std::fs::read_to_string;

fn main() {

    let contents = read_to_string("input.txt")
    .expect("Should have been able to read the file");

    let mut x: Vec<_> = contents.split("\n\n").map(|x| -> usize {x.split("\n").map(|row| row.parse().unwrap_or(0)).sum()}).collect();

    x.sort();

    let top_g: _ = x.iter().rev().take(3).sum::<usize>();

    println!("{:?} {:?}", x, top_g);
}

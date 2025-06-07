use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let x: i32 = iter.next().unwrap().parse().unwrap();
    let y: i32 = iter.next().unwrap().parse().unwrap();

    let count = (1..=6)
        .flat_map(|i| (1..=6).map(move |j| (i, j)))
        .filter(|&(i, j)| i + j >= x || (i - j).abs() >= y)
        .count();

    let ans: f64 = count as f64 / 36.0;
    println!("{}", ans);
}

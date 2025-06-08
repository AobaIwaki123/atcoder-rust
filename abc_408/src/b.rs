use std::collections::BTreeSet;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    // 1 <= n <= 100
    let n: usize = iter.next().unwrap().parse().unwrap();

    // 1 <= a_i <= 100

    let a: BTreeSet<i32> = (0..n)
        .map(|_| iter.next().unwrap().parse::<i32>().unwrap())
        .collect();

    println!("{}", a.len());
    let vec: Vec<_> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", vec.join(" "));
}

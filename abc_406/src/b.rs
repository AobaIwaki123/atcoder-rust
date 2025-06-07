use std::io::{self, Read};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: u32 = iter.next().unwrap().parse().unwrap();
    let a: Vec<u128> = (0..n)
        .map(|_| iter.next().unwrap().parse::<u128>().unwrap())
        .collect();

    let y = 10_u128.pow(k) - 1;

    let x = a
        .iter()
        .fold(1_u128, |acc, &ai| if acc * ai > y { 1 } else { acc * ai });

    println!("{}", x);
}

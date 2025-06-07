use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let a: f32 = iter.next().unwrap().parse().unwrap();
    let b: f32 = iter.next().unwrap().parse().unwrap();

    let c = a / b;
    let ans = c.round();

    println!("{}", ans);
}

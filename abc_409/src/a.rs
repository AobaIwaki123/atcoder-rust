use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("Failed to read input");
    let mut iter = buf.split_whitespace();

    let _n: usize = iter.next().expect("Missing N").parse().expect("Invalid N");
    let t = iter.next().expect("Missing T").chars().collect::<Vec<_>>();
    let a = iter.next().expect("Missing A").chars().collect::<Vec<_>>();

    let ans = t
        .iter()
        .zip(a.iter())
        .any(|(&c1, &c2)| c1 == 'o' && c2 == 'o');

    println!("{}", if ans { "Yes" } else { "No" });
}

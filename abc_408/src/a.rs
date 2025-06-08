use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    // 1 <= n <= 100
    let n: usize = iter.next().unwrap().parse().unwrap();
    // 1 <= s <= 100
    let s: i32 = iter.next().unwrap().parse().unwrap();
    // 1 <= t_i <= 1000
    let t: Vec<i32> = (0..n)
        .map(|_| iter.next().unwrap().parse::<i32>().unwrap())
        .collect();

    let is_ok = t[0] <= s && t.windows(2).all(|w| w[1] - w[0] <= s);

    println!("{}", if is_ok { "Yes" } else { "No" });
}

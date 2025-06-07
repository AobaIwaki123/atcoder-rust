use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let x: i32 = iter.next().unwrap().parse().unwrap();
    let y: i32 = iter.next().unwrap().parse().unwrap();

    let mut cnt: i32 = 0;

    for i in 1..=6 {
        for j in 1..=6 {
            if i + j >= x || (i - j).abs() >= y{
                cnt += 1;
            }
        }
    }

    let ans: f64 = cnt as f64 / 36.0;
    println!("{}", ans);
}

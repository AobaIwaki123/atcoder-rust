use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    // 入力制約: |s| >= 1, |s| < 5 * 10^5
    let s: String = iter.next().unwrap().parse().unwrap();

    if s.len() == 1 {
        let value = s.chars().next().unwrap().to_digit(10).unwrap() as i32;
        println!("{}", value + 1);
        return;
    }

    let mut chars = s.chars();
    let mut current = chars.next().unwrap().to_digit(10).unwrap() as i32;
    let mut ans = 1;

    for (_, next) in chars.enumerate() {
        let next_value = next.to_digit(10).unwrap() as i32;
        // println!("Index {}: current = {}, next = {}", i, current, next_value);

        let diff = current - next_value;
        // println!("diff: {}", diff);
        let ops = if diff >= 0 { diff } else { 9 + diff + 1 };
        // println!("ops: {}", ops);

        ans += ops + 1;

        current = next_value;
    }

    println!("{}", ans + current);
}

use std::io::{self, Read};

fn main() {
  let mut buf: String  = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();

  let mut iter: std::str::SplitWhitespace<'_> = buf.split_whitespace();
  let r: i32 = iter.next().unwrap().parse().unwrap();
  let x: i32 = iter.next().unwrap().parse().unwrap();

  let ok: bool = match x {
    1 => 1600 <= r && r <= 2999,
    2 => 1200 <= r && r <= 2399,
    _ => false,
  };

  println!("{}", if ok { "Yes" } else { "No" });
}

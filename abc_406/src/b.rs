use std::io::{self, Read};

fn main() {
  let mut buf: String = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();
  let mut iter: std::str::SplitWhitespace<'_> = buf.split_whitespace();

}

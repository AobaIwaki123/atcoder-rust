use std::io::{self, Read};

fn main() {
  let mut buf = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();
  let mut iter = buf.split_whitespace();

  let n: usize = iter.next().unwrap().parse().unwrap();
  let t = iter.next().unwrap().chars().collect::<Vec<_>>();
  let a = iter.next().unwrap().chars().collect::<Vec<_>>();

  let ans = t.iter().zip(a.iter())
  .any(|(&c1, &c2)| c1 == 'o' && c2 == 'o');

println!("{}", if ans { "Yes" } else { "No"});
}

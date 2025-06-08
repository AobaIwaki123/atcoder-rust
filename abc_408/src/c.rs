use std::io::{self, Read};

fn main() {
  let mut buf = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();
  let mut iter = buf.split_whitespace();

  // 1 <= n <= 10^6
  let n: usize = iter.next().unwrap().parse().unwrap();
  // 1 <= m <= 2 * 10^5
  let m: usize = iter.next().unwrap().parse().unwrap();
  // 1 <= l_i <= r_i <= n
  let mut lr = Vec::with_capacity(m);
  for _ in 0..m {
    let l: usize = iter.next().unwrap().parse().unwrap();
    let r: usize = iter.next().unwrap().parse().unwrap();
    lr.push((l, r));
  }

  
}

use std::io::{self, Read};

// 制約
// 2≤N≤3×10 
// 1≤A_i​ ≤10 

fn main() {
  let mut buf: String = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();
  let mut iter: std::str::SplitWhitespace<'_> = buf.split_whitespace();

  let n: usize = iter.next().unwrap().parse().unwrap();
  let a: Vec<usize> = (0..n)
    .map(|_| iter.next().unwrap().parse::<usize>().unwrap())
    .collect();

  let mut ans = 0;

  for i in 0..n {
    for j in (i+1)..n {
      ans += a[i] * a[j];
    }
  }

  println!("{}", ans);
}

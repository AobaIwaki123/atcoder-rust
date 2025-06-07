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

  let mut simple_sum = 0;
  let mut double_sum = 0;

  for i in 0..n {
    simple_sum += a[i];
    double_sum += a[i].pow(2);
  }

  let ans = (simple_sum.pow(2) - double_sum) / 2;
  println!("{}", ans);
}

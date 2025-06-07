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

  let sum: usize = a.iter().sum();
  let sum_of_squares: usize = a.iter().map(|&x| x.pow(2)).sum();

  let ans = (sum.pow(2) - sum_of_squares) / 2;
  println!("{}", ans);
}

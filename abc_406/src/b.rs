use std::io::{self, Read};

fn main() {
  let mut buf: String = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();
  let mut iter = buf.split_whitespace();

  let n: i32 = iter.next().unwrap().parse().unwrap();
  let k: u32 = iter.next().unwrap().parse().unwrap();
  let a: Vec<i32> = (0..n)
    .map(|_| iter.next().unwrap().parse::<i32>().unwrap())
    .collect();
  
  let mut ans = 1;

  for &x in &a { // O(100)
    if (x > (10_i32.pow(k) / ans)){
      ans = 1;
      continue;
    }
    let mul = ans * x;
    if mul < 10_i32.pow(k-1) {
      ans = 1;
    } else {
      ans = mul;
    }
  }

  println!("{}", ans);
}

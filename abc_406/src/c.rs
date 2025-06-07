use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let p: Vec<i32> = (0..n)
        .map(|_| iter.next().unwrap().parse::<i32>().unwrap())
        .collect();

    let mut rle: Vec<(char, usize)> = vec![];

    for i in 0..n - 1 {
        let dir = if p[i] < p[i + 1] { '<' } else { '>' };
        if rle.is_empty() || rle.last().unwrap().0 != dir {
            rle.push((dir, 1));
        } else {
            rle.last_mut().unwrap().1 += 1;
        }
    }

    let mut ans: i64 = 0;

    for i in 1..rle.len() - 1 {
        if rle[i].0 == '>' {
            ans += (rle[i - 1].1 as i64) * (rle[i + 1].1 as i64);
        }
    }

    println!("{}", ans);
}

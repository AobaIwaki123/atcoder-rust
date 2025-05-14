use std::io::{self, Read};

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter: std::str::SplitWhitespace<'_> = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = (0..n)
        .map(|_| iter.next().unwrap().parse::<usize>().unwrap() - 1)
        .collect();

    let mut ans: i32 = 0;
    loop {
        let mut exist: Vec<bool> = vec![false; m];
        for &x in &a {
            exist[x] = true;
        }

        if exist.iter().any(|&x| !x) {
            break;
        }

        ans += 1;
        a.pop();
    }

    println!("{}", ans);
}

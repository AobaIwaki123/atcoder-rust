use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let p: Vec<i32> = (0..n)
        .map(|_| iter.next().unwrap().parse::<i32>().unwrap())
        .collect();

    let mut diff = vec![0; n - 1];
    for i in 0..n - 1 {
        if p[i] < p[i + 1] {
            diff[i] = 1;
        } else {
            diff[i] = -1;
        }
    }

    let mut peaks = vec![];
    let mut valleys = vec![];
    for i in 1..n - 1 {
        if p[i - 1] < p[i] && p[i] > p[i + 1] {
            peaks.push(i);
        }
        if p[i - 1] > p[i] && p[i] < p[i + 1] {
            valleys.push(i);
        }
    }

    let mut ans = 0;
    let mut l = 0;
    while l <= n - 4 {
        let mut r = l + 3;

        while r < n {
            let peak_cnt = peaks.iter().filter(|&&x| l < x && x < r).count();
            let valley_cnt = valleys.iter().filter(|&&x| l < x && x < r).count();

            if p[l] < p[l + 1] && peak_cnt == 1 && valley_cnt == 1 {
                ans += 1;
            }

            r += 1;
        }

        l += 1;
    }

    println!("{}", ans);
}

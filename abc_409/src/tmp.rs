use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Nの読み込み
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    
    // 配列Aの読み込み
    let a: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // 解答の計算
    let mut count = vec![0; 1000000001]; // 0から10^9までの数値の出現回数を数える
    for &num in &a {
        count[num] += 1;
    }

    let mut result = 0;
    let mut sum = 0;
    for i in 0..=1000000000 {
        sum += count[i];
        if sum >= i {
            result = i;
        }
    }

    println!("{}", result);
}
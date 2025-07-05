use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();

    // 3等分した点をそもそも取れない場合考慮する必要はない
    if l % 3 != 0 {
        println!("0");
        return;
    }

    let d_line = lines.next().unwrap().unwrap();
    let d: Vec<usize> = d_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut x = 0;
    let mut cnt = vec![0_usize; l];

    // mod lを取りながらdを1つずつ進めていき点が存在する場所を埋めていく
    // Note: dは距離のみを表すため同じ位置に複数の点が存在しえる
    for i in 0..n {
        if i != 0 {
            x += d[i - 1]; // l >= 3のため問題ない
        }
        x %= l;
        cnt[x] += 1;
    }

    let mut ans = 0_usize;

    // 解が存在する場合、等間隔の位置に点が現れるのでそれらをかければ良い
    for i in 0..(l / 3) {
        ans += cnt[i] * cnt[i + l / 3] * cnt[i + 2 * l / 3];
    }

    println!("{}", ans);
}

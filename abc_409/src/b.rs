use std::io;

fn main() {
    // 標準入力からNを読み取る
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // 配列Aを読み取る
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // xをNから0まで降順に試す
    for x in (0..=n).rev() {
        let count = a.iter().filter(|&&val| val >= x as i32).count();
        if count >= x {
            println!("{}", x);
            break;
        }
    }
}

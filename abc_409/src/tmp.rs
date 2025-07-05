use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for x in (0..=n).rev() {
        let count = a.iter().filter(|&&val| val >= x as i32).count();
        if count >= x {
            println!("{}", x);
            break;
        }
    }
}

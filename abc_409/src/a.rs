use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let t: String = iter.next().unwrap().to_string();
    let a: String = iter.next().unwrap().to_string();

    let mut ans = false;

    for i in 0..n {
        if t.chars().nth(i).unwrap() == 'o' && a.chars().nth(i).unwrap() == 'o' {
            ans = true;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}   

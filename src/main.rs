use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.trim().split(' ').flat_map(&str::parse::<f64>);
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    println!("{}", a / b);
}

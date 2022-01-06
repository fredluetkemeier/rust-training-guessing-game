use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let index: u64 = args[1].parse().expect("I need a positive integer!");

    let mut last = 0;
    let mut current = 1;

    println!("{}\n{}", last, current);

    for _ in 1..index {
        let next = next_in_sequence(last, current);

        println!("{}", next);

        last = current;
        current = next;
    }
}

fn next_in_sequence(last: u128, current: u128) -> u128 {
    last + current
}

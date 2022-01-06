use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let index: u64 = args[1].parse().expect("I need a positive integer!");

    println!("0\n1");

    next_in_sequence(&|x| x == index, 0, (0, 1));
}

fn next_in_sequence(
    is_at_limit: &dyn Fn(u64) -> bool,
    count: u64,
    (last, current): (u128, u128),
) -> u128 {
    let next = last + current;

    println!("{}", next);

    if is_at_limit(count) {
        return next;
    }

    return next_in_sequence(is_at_limit, count + 1, (current, next));
}

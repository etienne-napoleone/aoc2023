static INPUT: &str = include_str!("./day1.txt");

fn main() {
    let res: u32 = INPUT
        .lines()
        .map(|l| {
            let mut numbers = l.chars().filter(|c| c.is_numeric());
            let first = numbers.next().and_then(|c| c.to_digit(10)).unwrap_or(0);
            let second = numbers
                .next_back()
                .and_then(|c| c.to_digit(10))
                .unwrap_or(first);
            first * 10 + second
        })
        .sum();
    println!("{res}");
}

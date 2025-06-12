const FILE: &str = "/home/n0ko/programming/rusty/advent15/day1/instructions";

fn main() {
    let output = std::fs::read(FILE).unwrap();
    let mut start = 0;
    let mut count = 0;

    for &byte in output.iter() {
        let val = match byte {
            40 => 1,
            41 => -1,
            _ => 0,
        };

        start += val;
        count += 1;

        if start == -1 {
            println!("Goes negative at: {}", count);
            break;
        }
    }

    println!("{:?}", start);
}

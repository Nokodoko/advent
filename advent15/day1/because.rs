fn something() {
    let anotherFile: &str = "/home/n0ko/programming/rusty/advent15/day1/instructions";
    let output = std::fs::read(anotherFile).unwrap();

    let mut start = 0;
    for &byte in output.iter() {
        let val = match byte {
            40 => 1i32,
            41 => -1i32,
            _ => 0,
        };
        start += val
    }

    println!("{:?}", start);
}

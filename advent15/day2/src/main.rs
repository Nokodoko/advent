use std::fmt::Display;

#[derive(Debug)]
struct Package {
    length: i32,
    width: i32,
    height: i32,
}

impl Package {
    fn new(l: i32, w: i32, h: i32) -> Self {
        Package {
            length: l,
            width: w,
            height: h,
        }
    }
    fn bow(&self) -> i32 {
        let volume_cubic_inches = self.length as i32 * self.width as i32 * self.height as i32;
        let cubic_inches_per_cubic_foot = 1728; // 12 * 12 * 12
        volume_cubic_inches / cubic_inches_per_cubic_foot
    }
    // 2*l*w + 2*w*h + 2*h*l.
    fn area(&self) -> i32 {
        (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length)
    }

    fn slack(&self) -> i32 {
        let min_side_area = *[
            self.height * self.length,
            self.length * self.width,
            self.width * self.height,
        ]
        .iter()
        .min()
        .unwrap();
        min_side_area
    }

    fn smallest_perimeter(&self) -> i32 {
        let min_side_perimeter = *[
            2 * (self.height + self.length),
            2 * (self.length + self.width),
            2 * (self.width + self.height),
        ]
        .iter()
        .min()
        .unwrap();
        min_side_perimeter
    }
    fn ribbon(&self) -> f32 {
        let volume_cubic_inches = self.length as f32 * self.width as f32 * self.height as f32;
        let cubic_inches_per_cubic_foot = 1728.0; // 12 * 12 * 12
        let bow_value = volume_cubic_inches / cubic_inches_per_cubic_foot;
        self.smallest_perimeter() as f32 + bow_value * cubic_inches_per_cubic_foot
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}x{}", self.length, self.width, self.height)
    }
}

fn main() {
    let file: &str = "/home/n0ko/programming/rusty/advent15/day2/dimensions";
    let output = std::fs::read_to_string(file).expect("Failed to read file: {file}");

    let mut total_square_footage = 0;
    let mut total_ribbon = 0f32;
    for line in output.lines() {
        let dimensions: Vec<i32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        let package = Package::new(dimensions[0], dimensions[1], dimensions[2]);
        total_ribbon += package.ribbon();
        total_square_footage += package.area() + package.slack()
    }
    println!("total: {}", total_square_footage);
    println!("total ribbon: {}", total_ribbon);
}

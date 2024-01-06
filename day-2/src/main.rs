use std::{fs::File, io};

mod part_1;
mod part_2;

const FILE_PATH: &str = "input.txt";

// Record (red, green, blue)
struct Record(u32, u32, u32);

impl Record {
    fn from_str(s: &str) -> Self {
        let colors = s.split(", ");
        colors.fold(Self(0, 0, 0), |mut acc, val| {
            let (count, color) = val.split_once(' ').unwrap();
            let count = count.parse().unwrap();
            match color {
                "red" => acc.0 = count,
                "green" => acc.1 = count,
                "blue" => acc.2 = count,
                _ => (),
            }

            acc
        })
    }
}

fn main() {
    let input = io::read_to_string(File::open(FILE_PATH).unwrap()).unwrap();

    let res = part_2::fewest_power_set(input);

    println!("{res}");
}

#[cfg(test)]
mod tests;

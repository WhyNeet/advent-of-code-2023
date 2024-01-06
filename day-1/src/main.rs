mod part_2;

use std::{fs::File, io};

const FILE_PATH: &str = "input.txt";

fn main() {
    let input = io::read_to_string(File::open(FILE_PATH).unwrap()).unwrap();

    let res = part_2::line_nums_sum(input);

    println!("{res}");
}

#[cfg(test)]
mod tests;

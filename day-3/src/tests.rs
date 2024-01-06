use std::{fs::File, io};

#[test]
pub fn part_1() {
    let example = io::read_to_string(File::open("example.txt").unwrap()).unwrap();

    let res = super::part_1::part_numbers_sum(example);

    assert_eq!(res, 4361);
}

#[test]
pub fn part_2() {
    let example = io::read_to_string(File::open("example.txt").unwrap()).unwrap();

    let res = super::part_2::sum_gear_ratios(example);

    assert_eq!(res, 467835);
}

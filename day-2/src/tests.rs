use std::{fs::File, io};

#[test]
fn check_ids_sum() {
    let input = io::read_to_string(File::open("example.txt").unwrap()).unwrap();

    let res = crate::part_1::sum_possible_ids(input);

    assert_eq!(res, 8)
}

#[test]
fn check_fewest_power_set() {
    let input = io::read_to_string(File::open("example.txt").unwrap()).unwrap();

    let res = crate::part_2::fewest_power_set(input);

    assert_eq!(res, 2286);
}

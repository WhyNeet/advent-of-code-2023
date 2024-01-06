use std::{fs::File, io};

#[test]
fn check_lines_sum() {
    let input = io::read_to_string(File::open("example.txt").unwrap()).unwrap();

    let res = crate::part_2::line_nums_sum(input);

    assert_eq!(res, 281);
}

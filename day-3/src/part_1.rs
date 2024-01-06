pub fn part_numbers_sum(input: String) -> u32 {
    let input = input.lines().collect::<Vec<&str>>();

    let res = input.iter().enumerate().map(|(line_idx, &line)| {
        let mut sum = 0;

        line.chars()
            .enumerate()
            .fold(String::new(), |acc, (col_idx, val)| {
                if col_idx == line.len() - 1 && val.is_numeric() {
                    let acc = acc + &val.to_string();
                    if !acc.is_empty()
                        && check_if_adjacent(&input, line_idx, col_idx, acc.len() - 1)
                    {
                        let num = acc.parse::<u32>().unwrap();
                        sum += num;
                    }

                    return String::new();
                }

                if !val.is_numeric() {
                    if !acc.is_empty() && check_if_adjacent(&input, line_idx, col_idx, acc.len()) {
                        let num = acc.parse::<u32>().unwrap();
                        sum += num;
                    }

                    return String::new();
                }

                acc + &val.to_string()
            });

        sum
    });

    res.sum()
}

pub fn check_if_adjacent(lines: &[&str], line_idx: usize, col_idx: usize, num_len: usize) -> bool {
    if (lines[line_idx].as_bytes()[col_idx] != b'.'
        && !(lines[line_idx].as_bytes()[col_idx] as char).is_numeric())
        || (col_idx > num_len && lines[line_idx].as_bytes()[col_idx - num_len - 1] != b'.')
    {
        return true;
    }

    for y in [-1, 1] as [isize; 2] {
        let line_idx = line_idx as isize + y;
        if line_idx < 0 || line_idx as usize >= lines.len() {
            continue;
        }

        'col: for x in 0..=(num_len + 1) {
            let col_idx = col_idx as isize - x as isize;
            if col_idx < 0 {
                break 'col;
            }

            let char = lines[line_idx as usize].as_bytes()[col_idx as usize];
            if char != b'.' {
                return true;
            }
        }
    }

    false
}

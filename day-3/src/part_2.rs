use std::collections::HashMap;

pub fn sum_gear_ratios(input: String) -> u64 {
    let input = input.lines().collect::<Vec<&str>>();

    let mut gearmap = HashMap::<(usize, usize), (Option<u64>, Option<u64>)>::new();

    let mut insert_gear = |gear: (usize, usize), num: u64| {
        let gear = gearmap.entry(gear).or_insert((None, None));
        if gear.0.is_none() {
            gear.0 = Some(num);
        } else {
            gear.1 = Some(num);
        }
    };

    input.iter().enumerate().for_each(|(line_idx, &line)| {
        line.chars()
            .enumerate()
            .fold(String::new(), |acc, (col_idx, val)| {
                if col_idx == line.len() - 1 && val.is_numeric() {
                    let acc = acc + &val.to_string();
                    if !acc.is_empty() {
                        if let Some(gear) =
                            get_adjacent_gear(&input, line_idx, col_idx, acc.len() - 1)
                        {
                            insert_gear(gear, acc.parse().unwrap());
                        }
                    }

                    return String::new();
                }

                if !val.is_numeric() {
                    if !acc.is_empty() {
                        if let Some(gear) = get_adjacent_gear(&input, line_idx, col_idx, acc.len())
                        {
                            insert_gear(gear, acc.parse().unwrap());
                        }
                    }

                    return String::new();
                }

                acc + &val.to_string()
            });
    });

    gearmap
        .into_values()
        .filter(|(_, second)| second.is_some())
        .map(|(first, second)| first.unwrap() * second.unwrap())
        .sum::<u64>()
}

pub fn get_adjacent_gear(
    lines: &[&str],
    line_idx: usize,
    col_idx: usize,
    num_len: usize,
) -> Option<(usize, usize)> {
    if lines[line_idx].as_bytes()[col_idx] == b'*'
        && !(lines[line_idx].as_bytes()[col_idx] as char).is_numeric()
    {
        return Some((line_idx, col_idx));
    }

    if col_idx > num_len && lines[line_idx].as_bytes()[col_idx - num_len - 1] == b'*' {
        return Some((line_idx, col_idx - num_len - 1));
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
            if char == b'*' {
                return Some((line_idx as usize, col_idx as usize));
            }
        }
    }

    None
}

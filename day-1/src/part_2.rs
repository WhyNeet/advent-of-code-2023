pub fn line_nums_sum(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let res = find_line_num(line);
            res
        })
        .sum::<u32>()
}

pub fn find_line_num(line: &str) -> u32 {
    let left = line
        .chars()
        .try_fold(String::new(), |acc, val| {
            let val_str = val.to_string();

            if let Some(num) = string_to_num(&acc) {
                return Err(num);
            }

            if val.is_numeric() {
                return Err(val_str.parse().unwrap());
            }

            Ok(acc + &val_str)
        })
        .err()
        .unwrap();
    let right = line
        .chars()
        .try_rfold(String::new(), |acc, val| {
            let val_str = val.to_string();

            if let Some(num) = string_to_num(&acc) {
                return Err(num);
            }

            if val.is_numeric() {
                return Err(val_str.parse().unwrap());
            }

            Ok(val_str + &acc)
        })
        .err()
        .unwrap();

    left * 10 + right
}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn string_to_num(s: &str) -> Option<u32> {
    if s.is_empty() {
        return None;
    }

    NUMS.iter().enumerate().find_map(|(idx, &val)| {
        if s.contains(val) {
            Some(idx as u32 + 1)
        } else {
            None
        }
    })
}

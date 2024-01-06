use crate::Record;

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

pub fn sum_possible_ids(input: String) -> u32 {
    let res = input
        .lines()
        .map(|line| {
            let (game, record) = line.split_once(": ").unwrap();

            let records = record.split("; ");

            if records
                .map(Record::from_str)
                .all(|record| record.0 <= RED && record.1 <= GREEN && record.2 <= BLUE)
            {
                let game_id = game.split_once(' ').unwrap().1.parse().unwrap();
                game_id
            } else {
                0
            }
        })
        .sum::<u32>();

    res
}

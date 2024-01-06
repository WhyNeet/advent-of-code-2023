use crate::Record;

pub fn fewest_power_set(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let record = line.split_once(": ").unwrap().1;

            let records = record.split("; ");
            let records = records.map(Record::from_str);

            let max = records.fold(Record(0, 0, 0), |mut acc, val| {
                if val.0 > acc.0 {
                    acc.0 = val.0;
                }

                if val.1 > acc.1 {
                    acc.1 = val.1;
                }

                if val.2 > acc.2 {
                    acc.2 = val.2;
                }

                acc
            });

            max.0 * max.1 * max.2
        })
        .sum::<u32>()
}

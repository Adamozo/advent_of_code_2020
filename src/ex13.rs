use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

// -----------------------------------------------------------------------------

pub struct Day13VariantA;

impl DaySolver for Day13VariantA {
    type Output = u32;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_13", "data_files/ex13.txt", "n^2");

    fn solution(_s: &str) -> anyhow::Result<<Self as DaySolver>::Output> {
        let (arrival_time, buses) = get_data();
        Ok(get_bus_mult_minutes2(arrival_time, &buses))
    }
}

pub fn get_data() -> (u32, Vec<u32>) {
    let input = "7,13,x,x,59,x,31,19";
    let arrival_time = 939;

    let buses: Vec<u32> = input
        .split(',')
        .filter(|f| *f != "x")
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    (arrival_time, buses)
}

pub fn get_bus_mult_minutes(arrival_time: u32, buses: &Vec<u32>) -> u32 {
    let mut i = arrival_time.clone();

    let (bus_id, minute) = loop {
        let available_bus = buses.iter().find(|bus| i % *bus == 0);

        if let Some(bus) = available_bus {
            break (bus, i);
        }

        i += 1;
    };

    (minute - arrival_time) * bus_id
}

pub fn get_bus_mult_minutes2(arrival_time: u32, buses: &Vec<u32>) -> u32 {
    let (bus_id, waiting_time) = buses.iter().fold(
        (std::u32::MAX, std::u32::MAX),
        |(bus_id, offset), checked_bus| {
            let time_for_first_occur = checked_bus - (arrival_time % checked_bus);
            
            if arrival_time % checked_bus == 0{
                (*checked_bus, 0)
            } else if time_for_first_occur < offset {
                (*checked_bus, time_for_first_occur)
            } else {
                (bus_id, offset)
            }
        },
    );

    (waiting_time) * bus_id

}

pub fn get_bus_mult_minutes3(arrival_time: u32, buses: &Vec<u32>) -> u32 {
    let (bus_id, waiting_time) = buses
        .iter()
        .map(|checked_bus| (checked_bus, checked_bus - (arrival_time % checked_bus)))
        .min_by_key(|(_, time_for_first_occur)| *time_for_first_occur)
        .unwrap();

    (waiting_time) * bus_id
}

pub fn run() -> anyhow::Result<()> {
    let (arrival_time, buses) = get_data();

    println!("Version1");
    println!(
        "Bus ID mult by minutes needed to wait: {}",
        get_bus_mult_minutes(arrival_time, &buses)
    );

    println!("Version2");
    println!(
        "Bus ID mult by minutes needed to wait: {}",
        get_bus_mult_minutes2(arrival_time, &buses)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(939, vec![7, 13, 59, 31, 19] => 295)]
    #[test_case(944, vec![7, 13, 59, 31, 19] => 0)]
    #[test_case(937, vec![7, 13, 59, 31, 19] => 7)]
    #[test_case(934, vec![7, 13] => 26)]
    fn test_ex13_get_bus_mult_minutes(arrival_time: u32, buses: Vec<u32>) -> u32 {
        get_bus_mult_minutes(arrival_time, &buses)
    }

    #[test_case(939, vec![7, 13, 59, 31, 19] => 295)]
    #[test_case(944, vec![7, 13, 59, 31, 19] => 0)]
    #[test_case(937, vec![7, 13, 59, 31, 19] => 7)]
    #[test_case(934, vec![7, 13] => 26)]
    fn test_ex13_get_bus_mult_minutes2(arrival_time: u32, buses: Vec<u32>) -> u32 {
        get_bus_mult_minutes2(arrival_time, &buses)
    }
}

use std::io::{Error};
use std::fs;

struct Battery {
    value: u32,
    index: usize
}

fn main() {
    if let Ok(batery_banks) =  get_lines() {
        let mut total = 0;
        for battery in batery_banks {
            total = total + parse_line(&battery);
        }
        println!("The sum is {}", total);
    } else {
        println!("unable to parse input file");
    }
}

fn get_lines() -> Result<Vec<String>, Error> {
    let lines = fs::read_to_string("./inputs/input.txt").expect("could not get file"); 
    
    let res = lines
    .split('\n')
    .map(String::from);

    Ok(res.collect())
}

fn parse_line(bank: &str) -> u32 {
    let first_battery = get_largest_battery(bank.get(0..bank.len()-1).unwrap());
    let remaining_battery = bank.get(first_battery.index+1..).expect("No value found");
    let second_battery = get_largest_battery(remaining_battery);
    let result = format!("{}{}", first_battery.value, second_battery.value);
    result.parse::<u32>().expect("could not build number")
}

fn get_largest_battery(bank: &str) -> Battery {
    let mut battery_value: u32 = 0;
    let mut battery_index: usize = 0;

    let mut largest_battery = 0;

    for (index, battery) in bank.chars()
        .enumerate()
        .map(|(i, b)| (i, b.to_digit(10).expect("could not parse")))
    {
        if battery == 9 {
            battery_value = battery;
            battery_index = index;
            break;
        }

        if battery > largest_battery {
            largest_battery = battery;
            battery_value = battery;
            battery_index = index;
        }
    }

    Battery { value: battery_value, index: battery_index }

}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    fn parse_line_success() {
        let mut result = parse_line("987654321111111");
        result = result + parse_line("811111111111119");
        result = result + parse_line("234234234234278");
        result = result + parse_line("818181911112111");
        assert_eq!(result, 357);
    }
}

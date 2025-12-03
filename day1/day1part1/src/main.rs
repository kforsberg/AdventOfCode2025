use std::{fs, io::Error};

fn main() {
    if let Ok(inputs) = build_input() {
        let mut current_position: i32 = 50;
        let mut total_zero: u32 = 0;
        for instruction in inputs {
            current_position = turn_dial(current_position, instruction.0, instruction.1);
            if current_position == 0 {
                total_zero += 1;
            }
        }
        println!("The password is {}", total_zero);
    }
}

fn turn_dial(current_position: i32, direction: char, number_of_clicks: u32) -> i32 {
    let mut new_position = match direction {
        'R' => current_position + number_of_clicks as i32,
        'L' => current_position - number_of_clicks as i32,
        _ => panic!("Could not turn the dial"),
    };
    let mut times_past_100 = (new_position.abs() as f32 / 100.0).floor();
    if times_past_100 == 0.0 {
        times_past_100 = 1.0;
    }

    if new_position > 99 {
        new_position = new_position - (100 * times_past_100 as i32);
    } else if new_position < 0 {
        new_position = new_position + (100 * times_past_100 as i32);
    }
    return new_position;
}

fn build_input() -> Result<Vec<(char, u32)>, Error> {
    let mut instructions: Vec<(char, u32)> = vec![];
    let lines = fs::read_to_string("./inputs/input.txt").expect("could not find file path");
    for line in lines.split("\n") {
        let direction = line.as_bytes()[0] as char;
        let clicks = line.get(1..).expect("could not determine clicks");
        let click_u32: u32 = clicks.parse().expect("unable to parse clicks");
        instructions.push((direction, click_u32));
    }
    Ok(instructions)
}

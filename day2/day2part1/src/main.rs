use std::{fs, io::Error};

fn main() {
    let mut answer: u128 = 0;
    let file_contents = fs::read_to_string("./inputs/test.txt").expect("Could not read file");
    let instructions = build_instructions(file_contents);
    for instruction in instructions.unwrap() {
        answer += process_instruction(instruction);
    }
    println!("The answer is {}", answer);
}

fn build_instructions(instruction_contents: String) -> Result<Vec<(String, String)>, Error> {
    let mut instructions: Vec<(String, String)> = vec![];
    for instruction in instruction_contents.split(',') {
        instructions.push(parse_instruction(instruction).expect("Could not parse instructions"));
    }
    Ok(instructions)
}

fn parse_instruction(instruction: &str) -> Result<(String, String), Error> {
    let parsed: Vec<&str> = instruction.split('-').collect();
    let first = parsed
        .first()
        .expect("Could not parse first part of instructions");
    let second = parsed
        .get(1)
        .expect("Could not parse second part of instruction");
    Ok((first.to_string(), second.to_string()))
}

fn process_instruction(instruction: (String, String)) -> u128 {
    // get all possible digit length
    let first_length = instruction.0.chars().count();
    let second_length = instruction.1.chars().count();
    let sum: u128 = 0;

    for x in first_length..=second_length {
        println!("{}", x);
        if x % 2 > 0 {
            println!("Invalid length")
        }
    }

    println!("End of iteration\n");

    sum
}

fn generate_digit_string(number: u8, string_length: usize) -> Result<String, String> {
    if number > 9 || string_length == 0 {
        return Err("Invalid inputs".to_string());
    }
    let s = std::iter::repeat_n(char::from(b'0' + number), string_length).collect::<String>();
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_digit_string_works() {
        let result = generate_digit_string(8, 7);
        assert_eq!(result.unwrap(), "8888888");
    }

    #[test]
    fn builds_instructions_works() {
        let input = "123-998,89999-3399873";
        let result = build_instructions(input.to_string()).unwrap();

        assert_eq!(result.iter().len(), 2);
        assert_eq!(result.first().unwrap().0, "123");
        assert_eq!(result.first().unwrap().1, "998");
        assert_eq!(result.get(1).unwrap().0, "89999");
        assert_eq!(result.get(1).unwrap().1, "3399873");
    }
}

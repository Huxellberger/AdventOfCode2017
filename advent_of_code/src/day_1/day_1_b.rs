use input;

pub fn run_day_1_b()
{
    let input = input::input_reader::read_file("input/day_1_b.txt");

    println!("day 1 b result: {}",  get_satisfying_numbers(&input));
}

// Always has an even number of elements
pub fn get_satisfying_numbers(input_string: &str) -> u32
{
    let string_length = input_string.chars().count();
    let digits_ahead = string_length / 2;
    let mut current_count = 0;

    for i in 0..string_length
    {
        let combined_index = i + digits_ahead;
        let current_character = input_string.chars().nth(i).unwrap();

        // stop out of bounds
        if combined_index >= string_length
        {
            let overflow_count = (combined_index) - string_length;

            if characters_match(current_character, input_string.chars().nth(overflow_count).unwrap())
            {
                current_count += current_character.to_digit(32).unwrap();
            }
        }
        else
        {
            if characters_match(current_character, input_string.chars().nth(combined_index).unwrap())
            {
                current_count += current_character.to_digit(32).unwrap();
            }
        }
    }

    current_count
}

fn characters_match(first_char: char, second_char: char) -> bool
{
    first_char == second_char
}

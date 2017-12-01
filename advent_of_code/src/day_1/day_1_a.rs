use input;

pub fn run_day_1_a()
{
    let input = input::input_reader::read_file("input/day_1_a.txt");

    println!("day 1 a result: {}",  get_successive_numbers(&input));
}

pub fn get_successive_numbers(input_string: &str) -> u32
{
    let mut current_count = 0;
    let mut previous_character = '`';

    for character in input_string.chars()
    {
        if previous_character == character
        {
            current_count += character.to_digit(32).unwrap();
        }

        previous_character = character;
    }

    // Check for loop-round case
    if previous_character == input_string.chars().nth(0).unwrap()
    {
        current_count += previous_character.to_digit(32).unwrap();
    }

    current_count
}

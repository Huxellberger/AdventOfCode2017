use input;

pub fn run_day_2_b()
{
    let checksum_rows = input::input_reader::read_file_by_line("input/day_2_b.txt");

    let cumulative_checksum = get_checksum(&checksum_rows);

    println!("day 2 b result: {}", cumulative_checksum);
}

pub fn get_checksum(in_check_sheet: &Vec<String>) -> i32
{
    let mut total_checksum = 0;

    for line in in_check_sheet
    {
        let mut final_line_checksum = 0;
        let line_value = line.split("\t");

        let mut value_vector: Vec<i32> = Vec::new();

        for line_unit in line_value
        {
            value_vector.push(input::input_reader::convert_to_numerical_value(line_unit));
        }

        // Reduces checks we have to complete
        value_vector.sort();

        let value_count = value_vector.len();

        for current_low_value in 0..value_count
        {
            for possible_matches in current_low_value + 1..value_count
            {
                if value_vector[possible_matches] % value_vector[current_low_value] == 0
                {
                    final_line_checksum = value_vector[possible_matches] / value_vector[current_low_value];
                    break;
                }
            }
        }

        total_checksum += final_line_checksum;
    }

    total_checksum
}
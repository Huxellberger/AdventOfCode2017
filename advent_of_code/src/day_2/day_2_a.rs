use input;

pub fn run_day_2_a()
{
    let checksum_rows = input::input_reader::read_file_by_line("input/day_2_a.txt");

    let cumulative_checksum = get_checksum(&checksum_rows);

    println!("day 2 a result: {}", cumulative_checksum);
}

pub fn get_checksum(in_check_sheet: &Vec<String>) -> i32
{
    let mut total_checksum = 0;

    for line in in_check_sheet
    {
        let line_value = line.split("\t");

        let mut low_value = i32::max_value();
        let mut high_value = i32::min_value();

        for line_unit in line_value
        {
            let current_value = input::input_reader::convert_to_numerical_value(line_unit);

            if current_value < low_value
            {
                low_value = current_value;
            }

            if current_value > high_value
            {
                high_value = current_value;
            }
        }

        let value_diff = high_value - low_value;

        if value_diff > 0
        {
            total_checksum += value_diff;
        }
    }

    total_checksum
}
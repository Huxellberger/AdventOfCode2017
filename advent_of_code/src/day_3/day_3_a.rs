pub fn run_day_3_a()
{
    println!("day 3 a result: {}", get_manhattan_for_spiral_point(361527));
}

pub fn get_manhattan_for_spiral_point(in_point: i64) -> i64
{
    let mut row_count = 1;
    let mut iterations = 0;

    while square(row_count) < in_point
    {
        row_count += 2;
        iterations += 1;
    }

    let max_distance = iterations * 2;

    let mut considered_square = square(row_count);
    let mut offset_distance = 0;
    let mut current_offset = 0;
    let mut offset_sign = -1;

    while considered_square != in_point
    {
        considered_square -= 1;
        current_offset += 1;
        offset_distance += 1 * offset_sign;

        // Reached end of row
        if current_offset >= row_count - 1
        {
            current_offset = 0;
            offset_distance = 0;
            offset_sign = -1;
        }

        // Reached midpoint
        else if current_offset >= row_count / 2 + 1 && offset_sign == -1
        {
            offset_sign *= -1;
        }
    } 

    max_distance + offset_distance
}

pub fn square (in_num: i64) -> i64
{
    let mut current_val = in_num;
    let mut sq_result = 0;
    let mut count = 0;

    while current_val > 0
    {
        if (current_val & 1) == 1
        {
            sq_result += in_num << count;
        }

        current_val = current_val >> 1;
        count += 1;
    }  

    sq_result
} 
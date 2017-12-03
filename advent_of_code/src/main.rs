mod day_1{pub mod day_1_a; mod day_1_a_tests; pub mod day_1_b; mod day_1_b_tests;}

mod day_2{pub mod day_2_a; mod day_2_a_tests; pub mod day_2_b; mod day_2_b_tests;}

mod day_3{pub mod day_3_a; mod day_3_a_tests;}

mod input{pub mod input_reader; mod input_reader_tests;}

fn main() 
{
    day_1::day_1_a::run_day_1_a();
    day_1::day_1_b::run_day_1_b();

    day_2::day_2_a::run_day_2_a();
    day_2::day_2_b::run_day_2_b();

    day_3::day_3_a::run_day_3_a();
}

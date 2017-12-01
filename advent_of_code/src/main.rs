mod day_1{pub mod day_1_a;}

mod input{pub mod input_reader; mod input_reader_tests;}

fn main() 
{
    println!("Hello, world!");

    day_1::day_1_a::run_day_1_a();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

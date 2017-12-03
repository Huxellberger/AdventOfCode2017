#[cfg(test)]
mod day_3_a_tests
{
    use day_3;

    #[test]
    fn square_squaresnum() 
    {
        assert_eq!(day_3::day_3_a::square(5), 25);
    }

    #[test]
    fn getmanhattenforspiralpoint_1_0() 
    {
        assert_eq!(day_3::day_3_a::get_manhattan_for_spiral_point(1), 0);
    }

    #[test]
    fn getmanhattenforspiralpoint_12_3() 
    {
        assert_eq!(day_3::day_3_a::get_manhattan_for_spiral_point(12), 3);
    }

    #[test]
    fn getmanhattenforspiralpoint_23_2() 
    {
        assert_eq!(day_3::day_3_a::get_manhattan_for_spiral_point(23), 2);
    }

    #[test]
    fn getmanhattenforspiralpoint_1024_31() 
    {
        assert_eq!(day_3::day_3_a::get_manhattan_for_spiral_point(1024), 31);
    }
}
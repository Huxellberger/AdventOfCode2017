#[cfg(test)]
mod day_1_a_tests
{
    use day_1;

    #[test]
    fn getsuccessivenumbers_examplepairmatch() 
    {
        assert_eq!(day_1::day_1_a::get_successive_numbers("1122"), 3);
    }

    #[test]
    fn getsuccessivenumbers_chainofmatchescalculatesuccessfully() 
    {
        assert_eq!(day_1::day_1_a::get_successive_numbers("1111"), 4);
    }

    #[test]
    fn getsuccessivenumbers_nomatchesreturns0() 
    {
        assert_eq!(day_1::day_1_a::get_successive_numbers("12345"), 0);
    }

    #[test]
    fn getsuccessivenumbers_rotatescorrectly() 
    {
        assert_eq!(day_1::day_1_a::get_successive_numbers("91212129"), 9);
    }
}

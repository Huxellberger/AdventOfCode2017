#[cfg(test)]
mod day_1_b_tests
{
    use day_1;

    #[test]
    fn getsatisfyingnumbers_examplepairmatch() 
    {
        assert_eq!(day_1::day_1_b::get_satisfying_numbers("1212"), 6);
    }

    #[test]
    fn getsatisfyingnumbers_nomatchesreturns0() 
    {
        assert_eq!(day_1::day_1_b::get_satisfying_numbers("1221"), 0);
    }

    #[test]
    fn getsatisfyingnumbers_sequentialbehaves() 
    {
        assert_eq!(day_1::day_1_b::get_satisfying_numbers("123425"), 4);
    }

    #[test]
    fn getsatisfyingnumbers_rotatescorrectly() 
    {
        assert_eq!(day_1::day_1_b::get_satisfying_numbers("123123"), 12);
    }

    #[test]
    fn getsatisfyingnumbers_longinput() 
    {
        assert_eq!(day_1::day_1_b::get_satisfying_numbers("12131415"), 4);
    }
}

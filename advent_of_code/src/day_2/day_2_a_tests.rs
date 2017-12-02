#[cfg(test)]
mod day_2_a_tests
{
    use day_2;

    #[test]
    fn getchecksum_sheetgivesexpectedresult() 
    {
        let actual_checksum = day_2::day_2_a::get_checksum(&vec!["5\t1\t9\t5".to_owned(), "7\t5\t3".to_owned(), "2\t4\t6\t8".to_owned()]);

        assert_eq!(actual_checksum, 18);
    }
}
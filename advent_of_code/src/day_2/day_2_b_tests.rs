#[cfg(test)]
mod day_2_b_tests
{
    use day_2;

    #[test]
    fn getchecksum_sheetgivesexpectedresult() 
    {
        let actual_checksum = day_2::day_2_b::get_checksum(&vec!["5\t9\t2\t8".to_owned(), "9\t4\t7\t3".to_owned(), "3\t8\t6\t5".to_owned()]);

        assert_eq!(actual_checksum, 9);
    }
}
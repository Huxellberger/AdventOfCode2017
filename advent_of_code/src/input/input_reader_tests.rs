#[cfg(test)]
mod input_reader_tests
{
    use input;

    #[test]
    fn readinput_expectedtextreceived() 
    {
        let output = input::input_reader::read_file("input/test_input.txt");

        assert_eq!(output, "hello mum!");
    }

    #[test]
    #[should_panic]
    fn readinput_invalidcausespanic() 
    {
        input::input_reader::read_file("hellomum");
    }

    #[test]
    fn readinputbyline_dividesasexpected() 
    {
        assert_eq!(2, input::input_reader::read_file_by_line("input/test_line_input.txt").len());
    }

    #[test]
    fn converttonumericalvalue_convertsasexpected() 
    {
        assert_eq!(2365, input::input_reader::convert_to_numerical_value("2365"));
    }
}

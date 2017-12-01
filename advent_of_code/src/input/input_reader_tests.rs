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
}

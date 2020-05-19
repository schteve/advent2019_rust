/*
    --- Day 16: Flawed Frequency Transmission ---
    You're 3/4ths of the way through the gas giants. Not only do roundtrip signals to Earth take five hours, but the signal quality is quite bad as well. You can clean up the signal with the Flawed Frequency Transmission algorithm, or FFT.

    As input, FFT takes a list of numbers. In the signal you received (your puzzle input), each number is a single digit: data like 15243 represents the sequence 1, 5, 2, 4, 3.

    FFT operates in repeated phases. In each phase, a new list is constructed with the same length as the input list. This new list is also used as the input for the next phase.

    Each element in the new list is built by multiplying every value in the input list by a value in a repeating pattern and then adding up the results. So, if the input list were 9, 8, 7, 6, 5 and the pattern for a given element were 1, 2, 3, the result would be 9*1 + 8*2 + 7*3 + 6*1 + 5*2 (with each input element on the left and each value in the repeating pattern on the right of each multiplication). Then, only the ones digit is kept: 38 becomes 8, -17 becomes 7, and so on.

    While each element in the output array uses all of the same input array elements, the actual repeating pattern to use depends on which output element is being calculated. The base pattern is 0, 1, 0, -1. Then, repeat each value in the pattern a number of times equal to the position in the output list being considered. Repeat once for the first element, twice for the second element, three times for the third element, and so on. So, if the third element of the output list is being calculated, repeating the values would produce: 0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1.

    When applying the pattern, skip the very first value exactly once. (In other words, offset the whole pattern left by one.) So, for the second element of the output list, the actual pattern used would be: 0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1, ....

    After using this process to calculate each element of the output list, the phase is complete, and the output list of this phase is used as the new input list for the next phase, if any.

    Given the input signal 12345678, below are four phases of FFT. Within each phase, each output digit is calculated on a single line with the result at the far right; each multiplication operation shows the input digit on the left and the pattern value on the right:

    Input signal: 12345678

    1*1  + 2*0  + 3*-1 + 4*0  + 5*1  + 6*0  + 7*-1 + 8*0  = 4
    1*0  + 2*1  + 3*1  + 4*0  + 5*0  + 6*-1 + 7*-1 + 8*0  = 8
    1*0  + 2*0  + 3*1  + 4*1  + 5*1  + 6*0  + 7*0  + 8*0  = 2
    1*0  + 2*0  + 3*0  + 4*1  + 5*1  + 6*1  + 7*1  + 8*0  = 2
    1*0  + 2*0  + 3*0  + 4*0  + 5*1  + 6*1  + 7*1  + 8*1  = 6
    1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*1  + 7*1  + 8*1  = 1
    1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*0  + 7*1  + 8*1  = 5
    1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*0  + 7*0  + 8*1  = 8

    After 1 phase: 48226158

    4*1  + 8*0  + 2*-1 + 2*0  + 6*1  + 1*0  + 5*-1 + 8*0  = 3
    4*0  + 8*1  + 2*1  + 2*0  + 6*0  + 1*-1 + 5*-1 + 8*0  = 4
    4*0  + 8*0  + 2*1  + 2*1  + 6*1  + 1*0  + 5*0  + 8*0  = 0
    4*0  + 8*0  + 2*0  + 2*1  + 6*1  + 1*1  + 5*1  + 8*0  = 4
    4*0  + 8*0  + 2*0  + 2*0  + 6*1  + 1*1  + 5*1  + 8*1  = 0
    4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*1  + 5*1  + 8*1  = 4
    4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*0  + 5*1  + 8*1  = 3
    4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*0  + 5*0  + 8*1  = 8

    After 2 phases: 34040438

    3*1  + 4*0  + 0*-1 + 4*0  + 0*1  + 4*0  + 3*-1 + 8*0  = 0
    3*0  + 4*1  + 0*1  + 4*0  + 0*0  + 4*-1 + 3*-1 + 8*0  = 3
    3*0  + 4*0  + 0*1  + 4*1  + 0*1  + 4*0  + 3*0  + 8*0  = 4
    3*0  + 4*0  + 0*0  + 4*1  + 0*1  + 4*1  + 3*1  + 8*0  = 1
    3*0  + 4*0  + 0*0  + 4*0  + 0*1  + 4*1  + 3*1  + 8*1  = 5
    3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*1  + 3*1  + 8*1  = 5
    3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*0  + 3*1  + 8*1  = 1
    3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*0  + 3*0  + 8*1  = 8

    After 3 phases: 03415518

    0*1  + 3*0  + 4*-1 + 1*0  + 5*1  + 5*0  + 1*-1 + 8*0  = 0
    0*0  + 3*1  + 4*1  + 1*0  + 5*0  + 5*-1 + 1*-1 + 8*0  = 1
    0*0  + 3*0  + 4*1  + 1*1  + 5*1  + 5*0  + 1*0  + 8*0  = 0
    0*0  + 3*0  + 4*0  + 1*1  + 5*1  + 5*1  + 1*1  + 8*0  = 2
    0*0  + 3*0  + 4*0  + 1*0  + 5*1  + 5*1  + 1*1  + 8*1  = 9
    0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*1  + 1*1  + 8*1  = 4
    0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*0  + 1*1  + 8*1  = 9
    0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*0  + 1*0  + 8*1  = 8

    After 4 phases: 01029498
    Here are the first eight digits of the final output list after 100 phases for some larger inputs:

    80871224585914546619083218645595 becomes 24176176.
    19617804207202209144916044189917 becomes 73745418.
    69317163492948606335995924319873 becomes 52432133.
    After 100 phases of FFT, what are the first eight digits in the final output list?
*/

use std::iter;

fn gen_base_pattern(element: i32) -> Vec<i32> {
    let mut pattern: Vec<i32> = [0, 1, 0, -1].iter()
                                            .flat_map(|&i| iter::repeat(i).take((element + 1) as usize)) // If user specifies element 0 then want a pattern with 1 of each value
                                            .collect();
    pattern.rotate_left(1);
    pattern
}

fn ones_digit(input: i32) -> i32 {
    let output = input % 10;
    output.abs()
}

fn mult_pattern(input: &Vec<i32>, pattern: &Vec<i32>) -> i32 {
    let sum: i32 = pattern.iter()
                        .cycle()
                        .zip(input.iter())
                        .map(|(p, i)| i * p)
                        .sum();
    ones_digit(sum)
}

fn phase(input: &Vec<i32>) -> Vec<i32> {
    let output: Vec<i32> = (0..input.len())
                                .map(|i| gen_base_pattern(i as i32))
                                .map(|pattern| mult_pattern(input, &pattern))
                                .collect();
    output
}

fn fft(input: Vec<i32>, phases: i32) -> Vec<i32> {
    let mut working_vec = input;
    (0..phases).for_each(|_| working_vec = phase(&working_vec));
    working_vec
}

fn parse_string(s: &str) -> Vec<i32> {
    let list: Vec<i32> = s.trim()
                            .chars()
                            .map(|c| c.to_digit(10).unwrap() as i32)
                            .collect();
    list
}

#[aoc(day16, part1)]
pub fn solve(input: &str) -> String {
    let list = parse_string(&input);
    let fft_result = fft(list, 100);
    let fft_result_str: String = fft_result[..8].iter()
                                                .map(|i| i.to_string())
                                                .collect();
    println!("First 8 digits of FFT: {}", fft_result_str);
    fft_result_str
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_base_pattern() {
        let pattern = gen_base_pattern(0);
        assert_eq!(&pattern, &[1, 0, -1, 0]);

        let pattern = gen_base_pattern(1);
        assert_eq!(&pattern, &[0, 1, 1, 0, 0, -1, -1, 0]);

        let pattern = gen_base_pattern(2);
        assert_eq!(&pattern, &[0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0]);
    }

    #[test]
    fn test_ones_digit() {
        assert_eq!(ones_digit(38), 8);
        assert_eq!(ones_digit(-17), 7);
        assert_eq!(ones_digit(4), 4);
        assert_eq!(ones_digit(-4), 4);
    }

    #[test]
    fn test_mult_pattern() {
        let input = parse_string("98765");
        let pattern = vec![1, 2, 3];
        assert_eq!(mult_pattern(&input, &pattern), 2);

        let input = parse_string("12345678");
        let pattern = gen_base_pattern(0);
        assert_eq!(mult_pattern(&input, &pattern), 4);

        let pattern = gen_base_pattern(1);
        assert_eq!(mult_pattern(&input, &pattern), 8);

        let pattern = gen_base_pattern(2);
        assert_eq!(mult_pattern(&input, &pattern), 2);

        let pattern = gen_base_pattern(3);
        assert_eq!(mult_pattern(&input, &pattern), 2);

        let pattern = gen_base_pattern(4);
        assert_eq!(mult_pattern(&input, &pattern), 6);

        let pattern = gen_base_pattern(5);
        assert_eq!(mult_pattern(&input, &pattern), 1);

        let pattern = gen_base_pattern(6);
        assert_eq!(mult_pattern(&input, &pattern), 5);

        let pattern = gen_base_pattern(7);
        assert_eq!(mult_pattern(&input, &pattern), 8);
    }

    #[test]
    fn test_phase() {
        let input = parse_string("12345678");
        let phase_result = phase(&input);
        assert_eq!(&phase_result, &parse_string("48226158"));

        let input = parse_string("48226158");
        let phase_result = phase(&input);
        assert_eq!(&phase_result, &parse_string("34040438"));

        let input = parse_string("34040438");
        let phase_result = phase(&input);
        assert_eq!(&phase_result, &parse_string("03415518"));

        let input = parse_string("03415518");
        let phase_result = phase(&input);
        assert_eq!(&phase_result, &parse_string("01029498"));
    }

    #[test]
    fn test_fft() {
        let input = parse_string("12345678");
        let fft_result = fft(input, 4);
        assert_eq!(&fft_result, &parse_string("01029498"));

        let input = parse_string("80871224585914546619083218645595");
        let fft_result = fft(input, 100);
        assert_eq!(&fft_result[0..8], &parse_string("24176176")[..]);

        let input = parse_string("19617804207202209144916044189917");
        let fft_result = fft(input, 100);
        assert_eq!(&fft_result[0..8], &parse_string("73745418")[..]);

        let input = parse_string("69317163492948606335995924319873");
        let fft_result = fft(input, 100);
        assert_eq!(&fft_result[0..8], &parse_string("52432133")[..]);
    }

    #[test]
    fn test_parse_string() {
        let input = "12345678";
        let list = parse_string(input);
        assert_eq!(&list, &[1, 2, 3, 4, 5, 6, 7, 8]);
    }
}

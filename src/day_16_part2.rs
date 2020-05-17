/*
    --- Part Two ---
    Now that your FFT is working, you can decode the real signal.

    The real signal is your puzzle input repeated 10000 times. Treat this new signal as a single input list. Patterns are still calculated as before, and 100 phases of FFT are still applied.

    The first seven digits of your initial input signal also represent the message offset. The message offset is the location of the eight-digit message in the final output list. Specifically, the message offset indicates the number of digits to skip before reading the eight-digit message. For example, if the first seven digits of your initial input signal were 1234567, the eight-digit message would be the eight digits after skipping 1,234,567 digits of the final output list. Or, if the message offset were 7 and your final output list were 98765432109876543210, the eight-digit message would be 21098765. (Of course, your real message offset will be a seven-digit number, not a one-digit number like 7.)

    Here is the eight-digit message in the final output list after 100 phases. The message offset given in each input has been highlighted. (Note that the inputs given below are repeated 10000 times to find the actual starting input lists.)

    03036732577212944063491565474664 becomes 84462026.
    02935109699940807407585447034323 becomes 78725270.
    03081770884921959731165446850517 becomes 53553731.
    After repeating your input signal 10000 times and running 100 phases of FFT, what is the eight-digit message embedded in the final output list?
*/

fn ones_digit(input: i32) -> i32 {
    let output = input % 10;
    output.abs()
}

fn phase(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();
    let mut sum = 0;
    for i in input.iter().rev() {
        sum += i;
        sum = ones_digit(sum);
        output.push(sum);
    }

    output.reverse();
    output
}

fn fft(input: &[i32], phases: i32) -> Vec<i32> {
    let mut working_vec = input.to_vec();
    for _ in 0..phases {
        let phase_output = phase(&working_vec);
        working_vec = phase_output;
    }

    working_vec
}

fn parse_string(s: &str) -> Vec<i32> {
    let list: Vec<i32> = s.trim()
                            .chars()
                            .map(|c| c.to_digit(10).unwrap() as i32)
                            .collect();
    list
}

fn parse_string_x1000(s: &str) -> Vec<i32> {
    let clean_s = s.trim();
    let list: Vec<i32> = clean_s.chars()
                            .map(|c| c.to_digit(10).unwrap() as i32)
                            .cycle()
                            .take(clean_s.len() * 10000)
                            .collect();
    list
}

fn get_offset(list: &[i32]) -> i32 {
    let mut offset = 0;
    for (i, &item) in list[0..7].iter().rev().enumerate() {
        offset += 10i32.pow(i as u32) * item;
    }
    println!("Offset: {}", offset);
    offset
}

#[aoc(day16, part2)]
pub fn solve(input: &str) -> String {
    let list = parse_string_x1000(&input);
    let offset = get_offset(&list);
    println!("Size: {}", list[(offset as usize)..].len());

    let fft_result = fft(&list[(offset as usize)..], 100);
    let fft_result_str: String = fft_result[0..8].into_iter()
                                .map(|i| i.to_string())
                                .collect::<String>();
    println!("FFT x10000: {}", fft_result_str);
    fft_result_str
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fft() {
        let list = parse_string_x1000("03036732577212944063491565474664");
        let offset = get_offset(&list);
        let fft_result = fft(&list[(offset as usize)..], 100);
        assert_eq!(&fft_result[0..8], &parse_string("84462026")[..]);

        let list = parse_string_x1000("02935109699940807407585447034323");
        let offset = get_offset(&list);
        let fft_result = fft(&list[(offset as usize)..], 100);
        assert_eq!(&fft_result[0..8], &parse_string("78725270")[..]);

        let list = parse_string_x1000("03081770884921959731165446850517");
        let offset = get_offset(&list);
        let fft_result = fft(&list[(offset as usize)..], 100);
        assert_eq!(&fft_result[0..8], &parse_string("53553731")[..]);
    }
}

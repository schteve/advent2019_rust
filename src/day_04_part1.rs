/*
    --- Day 4: Secure Container ---
    You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.

    However, they do remember a few key facts about the password:

    It is a six-digit number.
    The value is within the range given in your puzzle input.
    Two adjacent digits are the same (like 22 in 122345).
    Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    Other than the range rule, the following are true:

    111111 meets these criteria (double 11, never decreases).
    223450 does not meet these criteria (decreasing pair of digits 50).
    123789 does not meet these criteria (no double).
    How many different passwords within the range given in your puzzle input meet these criteria?
*/

fn is_password_valid(password: u32) -> bool {
    let mut last_value = 0u32;
    let mut digits_repeat = false;
    let mut digits_never_decrease = true;

    for j in password.to_string().chars() {
        let this_value = j.to_digit(10).unwrap(); // Base 10

        if this_value == last_value {
            digits_repeat = true;
        }

        if this_value < last_value {
            digits_never_decrease = false;
            break;
        }

        last_value = this_value;
    }

    // print!("{} - ", password);
    if digits_repeat == false {
        // println!("no, digits don't repeat");
        return false;
    } else if digits_never_decrease == false {
        // println!("no, digits decrease");
        return false;
    } else {
        // println!("yes!");
        return true;
    }
}

fn count_passwords(start: u32, end: u32) -> u32 {
    let count = (start..end).map(is_password_valid)
                            .filter(|&is_valid| is_valid == true)
                            .count();
    count as u32
}

#[aoc(day4, part1)]
pub fn solve(input: &str) -> u32 {
    let range: Vec<u32> = input.trim().split('-')
                                .map(|s| s.parse::<u32>().unwrap())
                                .collect();
    let count = count_passwords(range[0], range[1]);
    println!("Passwords: {}", count);
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_password_valid() {
        assert_eq!(is_password_valid(112233), true);
        assert_eq!(is_password_valid(223450), false);
        assert_eq!(is_password_valid(123789), false);
    }

    #[test]
    fn test_count_passwords() {
        assert_eq!(count_passwords(123450, 123460), 1); // 455
        assert_eq!(count_passwords(123400, 123500), 11); // 444, 445, 446, 447, 448, 449, 455, 466, 477, 488, 499
    }
}

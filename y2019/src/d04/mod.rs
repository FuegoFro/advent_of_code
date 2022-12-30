pub fn main() {
    let low = 240920;
    let high = 789857;

    for test in [
        111111, 223450, 123789, 113789, 123788, 112233, 123444, 111122,
    ]
    .iter()
    {
        println!("{} -> {}", test, is_valid_password(test));
    }

    let valid = (low..=high).filter(is_valid_password).count();
    println!("{}", valid);
}

fn is_valid_password(num: &i32) -> bool {
    let digits = get_reverse_digits(*num);
    let mut prev = &digits[0];
    let mut has_double = false;
    let mut current_sequence_len = 1;
    for digit in digits[1..].iter() {
        if prev == digit {
            current_sequence_len += 1;
        } else {
            if current_sequence_len == 2 {
                has_double = true;
            }
            current_sequence_len = 1;
        }
        if prev < digit {
            return false;
        }
        prev = digit;
    }
    has_double || current_sequence_len == 2
}

fn get_reverse_digits(mut num: i32) -> Vec<u8> {
    let mut digits = Vec::new();

    while num > 0 {
        digits.push((num % 10) as u8);
        num /= 10;
    }

    digits
}

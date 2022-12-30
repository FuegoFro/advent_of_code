lalrpop_mod!(pub plus_precedence, "/d18/plus_precedence.rs");

use util::p_u64;

enum Operation {
    Add,
    Multiply,
}

fn handle_value(total: &mut Option<u64>, operation: &Option<Operation>, value: u64) {
    match total {
        None => {
            assert!(operation.is_none());
            *total = Some(value);
        }
        Some(total_value) => match operation {
            Some(Operation::Add) => *total = Some(*total_value + value),
            Some(Operation::Multiply) => *total = Some(*total_value * value),
            None => panic!("Expected to see operation before number!"),
        },
    }
}

fn eval_expr(chars: &mut impl Iterator<Item = char>) -> u64 {
    let mut operation: Option<Operation> = None;
    let mut total: Option<u64> = None;
    while let Some(char) = chars.next() {
        match char {
            '(' => {
                let inner_val = eval_expr(chars);
                handle_value(&mut total, &operation, inner_val);
            }
            ')' => return total.expect("Expected vale in expr"),
            ' ' => {}
            '+' => operation = Some(Operation::Add),
            '*' => operation = Some(Operation::Multiply),
            '0'..='9' => {
                let char_val = p_u64(&char.to_string());
                handle_value(&mut total, &operation, char_val);
            }
            _ => panic!("Unknown char: {}", char),
        }
    }
    total.expect("Expected vale in expr")
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let total: u64 = input.split('\n').map(|l| eval_expr(&mut l.chars())).sum();
    println!("{}", total);

    let plus_precedence_parser = plus_precedence::ExprParser::new();
    let total_plus_precedence: u64 = input
        .split('\n')
        .map(|l| plus_precedence_parser.parse(l).unwrap())
        .sum();
    println!("{}", total_plus_precedence);
}

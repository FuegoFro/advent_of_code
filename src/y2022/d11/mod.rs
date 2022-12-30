#![allow(clippy::needless_question_mark)]
use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"Monkey \d+:
  Starting items: (?P<items>(\d+(, )?)+)
  Operation: new = (?P<left_operand>[^ ]+) (?P<operator>[+*]) (?P<right_operand>[^ ]+)
  Test: divisible by (?P<test_divisible>\d+)
    If true: throw to monkey (?P<true_monkey>\d+)
    If false: throw to monkey (?P<false_monkey>\d+)"#)]
struct MonkeyRaw {
    items: Vec<String>,
    left_operand: String,
    operator: String,
    right_operand: String,
    test_divisible: u64,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Value(u64),
}

impl Operand {
    fn from_str(s: &str) -> Self {
        if s == "old" {
            Self::Old
        } else if let Ok(v) = s.parse() {
            Self::Value(v)
        } else {
            panic!("Unknown operand {:?}", s);
        }
    }

    fn value(&self, old: u64) -> u64 {
        match self {
            Operand::Old => old,
            Operand::Value(v) => *v,
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Multiply,
}

impl Operator {
    fn from_str(s: &str) -> Self {
        match s {
            "*" => Self::Multiply,
            "+" => Self::Plus,
            _ => panic!("Unknown operator {:?}", s),
        }
    }

    fn execute(&self, left: u64, right: u64) -> u64 {
        match self {
            Operator::Plus => left + right,
            Operator::Multiply => left * right,
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    left: Operand,
    operator: Operator,
    right: Operand,
}

impl Operation {
    fn new_value(&self, old: u64) -> u64 {
        self.operator
            .execute(self.left.value(old), self.right.value(old))
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    num_inspections: usize,
    items: Vec<u64>,
    operation: Operation,
    test_divisible: u64,
    true_monkey: usize,
    false_monkey: usize,
}

enum ResetOp {
    DivideBy(u64),
    ModBy(u64),
}

impl ResetOp {
    fn reset(&self, input: u64) -> u64 {
        match self {
            ResetOp::DivideBy(val) => input / val,
            ResetOp::ModBy(val) => input % val,
        }
    }
}

fn do_round(monkeys: &mut Vec<Monkey>, reset_op: ResetOp) {
    for i in 0..monkeys.len() {
        let mut items_to_process = Vec::new();
        std::mem::swap(&mut items_to_process, &mut monkeys[i].items);
        monkeys[i].num_inspections += items_to_process.len();
        for item in items_to_process.into_iter() {
            let new_worry = monkeys[i].operation.new_value(item);
            let new_worry = reset_op.reset(new_worry);
            let throw_to_idx = if new_worry % monkeys[i].test_divisible == 0 {
                monkeys[i].true_monkey
            } else {
                monkeys[i].false_monkey
            };
            monkeys[throw_to_idx].items.push(new_worry);
        }
    }
}

fn calculate_monkey_business(monkeys: &[Monkey]) -> usize {
    monkeys
        .iter()
        .map(|m| m.num_inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let orig_monkeys = input
        .split("\n\n")
        .map(|m| m.parse::<MonkeyRaw>().unwrap())
        .map(|m| Monkey {
            num_inspections: 0,
            items: m
                .items
                .into_iter()
                .map(|s| s.trim().parse().unwrap())
                .collect_vec(),
            operation: Operation {
                left: Operand::from_str(&m.left_operand),
                operator: Operator::from_str(&m.operator),
                right: Operand::from_str(&m.right_operand),
            },
            test_divisible: m.test_divisible,
            true_monkey: m.true_monkey,
            false_monkey: m.false_monkey,
        })
        .collect_vec();

    let mut monkeys_pt1 = orig_monkeys.clone();
    for _ in 0..20 {
        do_round(&mut monkeys_pt1, ResetOp::DivideBy(3));
    }
    println!("Part 1: {}", calculate_monkey_business(&monkeys_pt1));

    let mut monkeys_pt2 = orig_monkeys;
    let modulus: u64 = monkeys_pt2.iter().map(|m| m.test_divisible).product();
    for _ in 0..10000 {
        do_round(&mut monkeys_pt2, ResetOp::ModBy(modulus));
    }
    println!("Part 2: {}", calculate_monkey_business(&monkeys_pt2));
}

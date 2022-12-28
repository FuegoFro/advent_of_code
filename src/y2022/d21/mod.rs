use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Alignment::Left;

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl Operation {
    fn get_output(&self, left: i64, right: i64) -> i64 {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
            Operation::Eq => left,
        }
    }
    fn get_left(&self, output: i64, right: i64) -> i64 {
        match self {
            Operation::Add => output - right,
            Operation::Sub => output + right,
            Operation::Mul => output / right,
            Operation::Div => output * right,
            Operation::Eq => right,
        }
    }
    fn get_right(&self, output: i64, left: i64) -> i64 {
        match self {
            Operation::Add => output - left,
            Operation::Sub => left - output,
            Operation::Mul => output / left,
            Operation::Div => left / output,
            Operation::Eq => left,
        }
    }
}

enum Action {
    Unknown,
    Number(i64),
    Operation {
        left: String,
        op: Operation,
        right: String,
    },
}

impl Action {
    fn from_str(s: &str) -> Self {
        if let Some((left, op, right)) = s.split(' ').collect_tuple() {
            let op = match op {
                "*" => Operation::Mul,
                "+" => Operation::Add,
                "/" => Operation::Div,
                "-" => Operation::Sub,
                _ => panic!("Unknown op {:?}", op),
            };
            Self::Operation {
                left: left.to_string(),
                op,
                right: right.to_string(),
            }
        } else {
            Self::Number(s.parse().unwrap())
        }
    }

    fn needed_monkeys(&self) -> Vec<&str> {
        match self {
            Action::Unknown | Action::Number(_) => vec![],
            Action::Operation { left, right, .. } => vec![left, right],
        }
    }
}

struct Monkey {
    name: String,
    action: Action,
}

impl Monkey {
    fn from_str(s: &str) -> Self {
        let (name, action) = s.split_once(": ").unwrap();
        Self {
            name: name.to_string(),
            action: Action::from_str(action),
        }
    }
}

// Two modes
// - "normal"
//      Takes inputs, sets output
// - "reverse"
//      Takes output (and sometimes input), sets input

fn ensure(
    monkeys: &HashMap<String, Action>,
    values: &mut HashMap<String, Option<i64>>,
    name: &str,
) {
    for monkey in monkeys[name].needed_monkeys().into_iter() {
        ensure(monkeys, values, monkey);
    }
    let value = match &monkeys[name] {
        Action::Unknown => None,
        Action::Number(val) => Some(*val),
        Action::Operation { left, op, right } => match (values[left], values[right]) {
            (Some(left), Some(right)) => Some(op.get_output(left, right)),
            _ => None,
        },
    };
    values.insert(name.to_string(), value);
}

fn run_reverse(
    monkeys: &HashMap<String, Action>,
    values: &mut HashMap<String, Option<i64>>,
    name: &str,
) {
    let expected_output = values[name].unwrap();
    match &monkeys[name] {
        Action::Unknown => {
            values.insert(name.to_string(), Some(expected_output));
        }
        Action::Number(_) => (),
        Action::Operation { left, op, right } => match (values[left], values[right]) {
            (Some(left_val), None) => {
                values.insert(
                    right.to_string(),
                    Some(op.get_right(expected_output, left_val)),
                );
                run_reverse(monkeys, values, right);
            }
            (None, Some(right_val)) => {
                values.insert(
                    left.to_string(),
                    Some(op.get_left(expected_output, right_val)),
                );
                run_reverse(monkeys, values, left);
            }
            val @ _ => panic!("Unexpected (left, right) = {:?}", val),
        },
    };
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let mut monkeys = input
        .split('\n')
        .map(Monkey::from_str)
        .map(|m| (m.name, m.action))
        .collect::<HashMap<_, _>>();

    let mut values = HashMap::new();
    ensure(&monkeys, &mut values, "root");

    println!("Part 1: {}", values["root"].unwrap());

    if let Some(Action::Operation { ref mut op, .. }) = monkeys.get_mut("root") {
        *op = Operation::Eq;
    } else {
        panic!("Failed to update root op");
    }
    monkeys.insert("humn".into(), Action::Unknown);
    let mut values = HashMap::new();
    ensure(&monkeys, &mut values, "root");

    let root_value = monkeys["root"]
        .needed_monkeys()
        .into_iter()
        .flat_map(|n| values[n])
        .next()
        .unwrap();
    values.insert("root".into(), Some(root_value));
    run_reverse(&monkeys, &mut values, "root");
    println!("Part 2: {}", values["humn"].unwrap());
}

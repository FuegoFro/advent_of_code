use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use util::p_u32;

enum Rule {
    Literal(char),
    Sequence(Vec<u32>),
    Alternative(Vec<Vec<u32>>),
}

impl Rule {
    fn from_packed(packed: &str) -> Self {
        lazy_static! {
            static ref RE_LITERAL: Regex = Regex::new(r#"^"(?P<char>[a-zA-Z])"$"#).unwrap();
            static ref RE_SEQUENCE: Regex = Regex::new(r"^\d+( \d+)*$").unwrap();
            static ref RE_ALTERNATIVE: Regex = Regex::new(r"^\d+( \d+)* \| \d+( \d+)*$").unwrap();
        }
        // IntelliJ doesn't understand this without this alias.
        let re_literal: &Regex = &RE_LITERAL;
        let re_sequence: &Regex = &RE_SEQUENCE;
        let re_alternative: &Regex = &RE_ALTERNATIVE;
        if let Some(caps) = re_literal.captures(packed) {
            let char = caps.name("char").unwrap().as_str().chars().next().unwrap();
            Rule::Literal(char)
        } else if re_sequence.is_match(packed) {
            Rule::Sequence(packed.split(" ").map(p_u32).collect())
        } else if re_alternative.is_match(packed) {
            Rule::Alternative(
                packed
                    .split(" | ")
                    .map(|s| s.split(" ").map(p_u32).collect())
                    .collect(),
            )
        } else {
            panic!("Unknown packed string: {}", packed);
        }
    }
}

fn regex_for_rule(
    memo: &mut HashMap<u32, String>,
    rules: &HashMap<u32, Rule>,
    rule_num: u32,
) -> String {
    if let Some(prev_result) = memo.get(&rule_num) {
        return prev_result.to_owned();
    }

    let regex = match &rules[&rule_num] {
        Rule::Literal(char) => char.to_string(),
        Rule::Sequence(s) => s.iter().map(|r| regex_for_rule(memo, rules, *r)).join(""),
        Rule::Alternative(a) => [
            "(",
            &a.iter()
                .map(|s| s.iter().map(|r| regex_for_rule(memo, rules, *r)).join(""))
                .join("|"),
            ")",
        ]
        .join(""),
    };

    memo.insert(rule_num, regex.clone());

    regex
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let mut split = input.split("\n\n");
    let rules_raw = split.next().unwrap();
    let messages = split.next().unwrap();
    assert!(split.next().is_none());

    let rules = rules_raw
        .split("\n")
        .map(|l| {
            let mut splitn = l.splitn(2, ": ");
            let rule_num = splitn.next().map(p_u32).unwrap();
            let rule = splitn.next().map(Rule::from_packed).unwrap();
            assert!(splitn.next().is_none());
            (rule_num, rule)
        })
        .collect::<HashMap<_, _>>();

    let mut memo = HashMap::new();
    let regex = ["^", &regex_for_rule(&mut memo, &rules, 0), "$"].join("");
    let rule_zero_regex = Regex::new(&regex).unwrap();
    let matching = messages
        .split("\n")
        .filter(|m| rule_zero_regex.is_match(m))
        .collect::<Vec<_>>();
    println!("{}", matching.len());

    let regex_42 = Regex::new(&format!("^{}", regex_for_rule(&mut memo, &rules, 42))).unwrap();
    let regex_31 = Regex::new(&format!("^{}", regex_for_rule(&mut memo, &rules, 31))).unwrap();
    let matching = messages
        .split("\n")
        .filter(|m| {
            let mut m = *m;
            let mut num_42_matches = 0;
            let mut num_31_matches = 0;
            while let Some(mat) = regex_42.find(m) {
                num_42_matches += 1;
                m = &m[mat.end()..];
            }
            while let Some(mat) = regex_31.find(m) {
                num_31_matches += 1;
                m = &m[mat.end()..];
            }
            m.is_empty()
                && num_42_matches >= 2
                && num_31_matches >= 1
                && num_42_matches > num_31_matches
        })
        .collect::<Vec<_>>();
    println!("{}", matching.len());
}

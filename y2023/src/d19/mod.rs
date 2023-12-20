use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;
use std::ops::Range;

mod stringly_typed {
    use serde::Deserialize;
    use std::ops::Deref;
    use std::str::FromStr;

    pub(crate) struct StringlyTyped<T>(T);
    impl<T: std::fmt::Debug> std::fmt::Debug for StringlyTyped<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }

    impl<T> Deref for StringlyTyped<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<'de, T: Deserialize<'de> + FromStr> Deserialize<'de> for StringlyTyped<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s: String = Deserialize::deserialize(deserializer)?;
            Ok(StringlyTyped(s.parse().map_err(serde::de::Error::custom)?))
        }
    }
}

use stringly_typed::StringlyTyped;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?P<attribute>\w+)(?P<op>.)(?P<arg>\d+)")]
struct Condition {
    attribute: String,
    op: String,
    arg: u32,
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"((?P<condition>[^,:]+):)?(?P<action>\w+)")]
struct Rule {
    condition: Option<StringlyTyped<Condition>>,
    action: String,
}

impl Rule {
    fn accepts(&self, part: &Part) -> bool {
        self.condition
            .as_ref()
            .map(|c| {
                let lhs = match c.attribute.as_str() {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    attr => panic!("Unknown attribute {}", attr),
                };
                match c.op.as_str() {
                    ">" => lhs > c.arg,
                    "<" => lhs < c.arg,
                    op => panic!("Unknown op {}", op),
                }
            })
            .unwrap_or(true)
    }
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?P<name>\w+)\{(?P<rules>.+)\}")]
struct Workflow {
    name: String,
    rules: Vec<StringlyTyped<Rule>>,
}

impl Workflow {
    fn result(&self, part: &Part) -> &str {
        &(self.rules.iter().find(|r| r.accepts(part)).unwrap().action)
    }
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}")]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug)]
struct PartRange {
    ranges: HashMap<String, Range<u32>>,
}

impl PartRange {
    /// Returns (pass, fail)
    fn split(self, rule: &Rule) -> (Option<Self>, Option<Self>) {
        match rule.condition.as_ref() {
            Some(condition) => {
                let greater_than = condition.op == ">";
                let range = &self.ranges[&condition.attribute];
                // Split based on greater than, swap later.
                // Eg w/ 1..=10, "<5" will be 1..=4 and 5..=10, but ">4" will be 5..=10 and 1..=4
                let greater_than_arg = if greater_than {
                    condition.arg + 1
                } else {
                    condition.arg
                };
                // We're using "<" with "greater_than_arg" because the arg is intended for the rhs, but is easier
                // to read here on the lhs.
                let (lower, higher) = if greater_than_arg < range.start {
                    (None, Some(range.clone()))
                } else if greater_than_arg < range.end {
                    (
                        Some(range.start..greater_than_arg),
                        Some(greater_than_arg..range.end),
                    )
                } else {
                    (Some(range.clone()), None)
                };

                let (pass, fail) = if greater_than {
                    (higher, lower)
                } else {
                    (lower, higher)
                };

                (
                    pass.map(|pass| {
                        let mut ranges = self.ranges.clone();
                        ranges.insert(condition.attribute.clone(), pass);
                        Self { ranges }
                    }),
                    fail.map(|fail| {
                        let mut ranges = self.ranges;
                        ranges.insert(condition.attribute.clone(), fail);
                        Self { ranges }
                    }),
                )
            }
            None => (Some(self), None),
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let (workflows_raw, parts_raw) = input.split_once("\n\n").unwrap();
    let workflows = workflows_raw
        .lines()
        .map(|l| l.parse::<Workflow>().unwrap())
        .map(|w| (w.name.clone(), w))
        .collect::<HashMap<_, _>>();
    let parts = parts_raw
        .lines()
        .map(|l| l.parse::<Part>().unwrap())
        .collect_vec();

    let p1 = parts
        .iter()
        .map(|part| {
            let mut workflow = "in";
            while workflow != "R" && workflow != "A" {
                workflow = workflows.get(workflow).unwrap().result(part);
            }
            if workflow == "R" {
                0
            } else {
                part.x + part.m + part.a + part.s
            }
        })
        .sum::<u32>();

    println!("Part 1: {}", p1);

    let p2 = num_options(
        &workflows,
        "in",
        PartRange {
            ranges: [
                ("x".into(), 1..4001),
                ("m".into(), 1..4001),
                ("a".into(), 1..4001),
                ("s".into(), 1..4001),
            ]
            .into_iter()
            .collect(),
        },
    );
    println!("Part 2: {}", p2);
}

fn num_options(
    workflows: &HashMap<String, Workflow>,
    workflow_name: &str,
    mut part_range: PartRange,
) -> u64 {
    // eprintln!("Called w/ {} {:?}", workflow_name, part_range);
    // if workflow_name != "in" && workflow_name != "px" {
    //     return 0;
    // }
    if workflow_name == "R" {
        return 0;
    }
    if workflow_name == "A" {
        return part_range
            .ranges
            .values()
            .map(|r| (r.end - r.start) as u64)
            .product();
    }
    let workflow = workflows.get(workflow_name).unwrap();
    let mut options = 0;
    for rule in workflow.rules.iter() {
        // eprintln!("Iteration w/ {:?}", part_range);
        // eprintln!(
        //     "Evaluating {}{}",
        //     rule.condition
        //         .as_ref()
        //         .map(|c| format!("{}{}{}:", c.attribute, c.op, c.arg))
        //         .unwrap_or(String::new()),
        //     rule.action,
        // );
        let (pass, fail) = part_range.split(rule);
        if let Some(pass) = pass {
            options += num_options(workflows, &rule.action, pass);
        }
        if let Some(fail) = fail {
            part_range = fail;
        } else {
            // No way to continue this rule chain
            break;
        }
    }

    options
}

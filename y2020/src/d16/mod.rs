use std::ops::RangeInclusive;

use regex::Regex;

use util::p_u32;

#[derive(Debug)]
struct Field {
    name: String,
    ranges: Vec<RangeInclusive<u32>>,
}

impl Field {
    fn from_packed(packed: &str) -> Self {
        lazy_static! {
            static ref RE_FIELD: Regex =
                Regex::new(r"^(?P<name>.+): (?P<low>\d+-\d+) or (?P<high>\d+-\d+)$").unwrap();
        }
        // IntelliJ doesn't understand this without this alias.
        let re_field: &Regex = &RE_FIELD;
        let captures = re_field.captures(packed).expect(packed);
        Field {
            name: captures.name("name").unwrap().as_str().to_owned(),
            ranges: vec![
                Field::parse_range(captures.name("low").unwrap().as_str()),
                Field::parse_range(captures.name("high").unwrap().as_str()),
            ],
        }
    }

    fn parse_range(packed: &str) -> RangeInclusive<u32> {
        let mut parts = packed.split('-');
        let low = parts.next().map(p_u32).unwrap();
        let high = parts.next().map(p_u32).unwrap();
        assert!(parts.next().is_none());
        low..=high
    }

    fn is_valid(&self, value: u32) -> bool {
        self.ranges.iter().any(|r| r.contains(&value))
    }
}

struct Ticket {
    values: Vec<u32>,
}

impl Ticket {
    fn from_packed(packed: &str) -> Self {
        Ticket {
            values: packed.split(',').map(p_u32).collect(),
        }
    }

    fn invalid_values_sums(&self, fields: &[Field]) -> u32 {
        self.values
            .iter()
            .filter(|v| !fields.iter().any(|f| f.is_valid(**v)))
            .sum()
    }

    fn is_valid(&self, fields: &[Field]) -> bool {
        self.values
            .iter()
            .all(|v| fields.iter().any(|f| f.is_valid(*v)))
    }
}

fn pt1(fields: &[Field], nearby_tickets: &[Ticket]) {
    let invalid_sums: u32 = nearby_tickets
        .iter()
        .map(|t| t.invalid_values_sums(fields))
        .sum();

    println!("{}", invalid_sums);
}

fn pt2(fields: &Vec<Field>, your_ticket: &Ticket, nearby_tickets: &[Ticket]) {
    let valid_tickets = nearby_tickets
        .iter()
        .filter(|t| t.is_valid(fields))
        .collect::<Vec<_>>();

    let mut field_and_indices = fields
        .iter()
        .map(|field| {
            let mut potential_indices: Vec<usize> = Vec::new();
            for i in 0..fields.len() {
                if valid_tickets.iter().all(|t| field.is_valid(t.values[i])) {
                    potential_indices.push(i);
                }
            }
            (field, potential_indices)
        })
        .collect::<Vec<_>>();

    let mut ordered_fields: Vec<Option<&Field>> = vec![None; fields.len()];
    while !field_and_indices.is_empty() {
        field_and_indices = field_and_indices
            .into_iter()
            .filter_map(|(f, idxs)| {
                assert!(!idxs.is_empty());
                if idxs.len() == 1 {
                    let idx = idxs[0];
                    assert!(ordered_fields[idx].is_none());
                    ordered_fields[idx] = Some(f);
                    // Done with this entry, remove it
                    None
                } else {
                    // Filter down the used indices
                    Some((
                        f,
                        idxs.into_iter()
                            .filter(|i| ordered_fields[*i].is_none())
                            .collect::<Vec<_>>(),
                    ))
                }
            })
            .collect();
    }

    let departure_product: u64 = ordered_fields
        .iter()
        .map(|f| f.unwrap())
        .enumerate()
        .filter(|(_, f)| f.name.starts_with("departure"))
        .map(|(i, _)| your_ticket.values[i] as u64)
        .product();

    println!("{}", departure_product);
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let mut input_parts = input.split("\n\n");
    let fields = input_parts
        .next()
        .unwrap()
        .split('\n')
        .map(Field::from_packed)
        .collect::<Vec<_>>();
    let your_ticket = input_parts
        .next()
        .unwrap()
        .split('\n')
        .skip(1)
        .map(Ticket::from_packed)
        .next()
        .unwrap();
    let nearby_tickets = input_parts
        .next()
        .unwrap()
        .split('\n')
        .skip(1)
        .map(Ticket::from_packed)
        .collect::<Vec<_>>();

    pt1(&fields, &nearby_tickets);
    pt2(&fields, &your_ticket, &nearby_tickets);
}

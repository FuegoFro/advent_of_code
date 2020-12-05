use regex::Regex;

fn is_valid_year(year: &Option<String>, min: i32, max: i32) -> bool {
    year.as_ref()
        .filter(|y| y.len() == 4)
        .and_then(|y| y.parse::<i32>().ok())
        .filter(|y| min <= *y && *y <= max)
        .is_some()
}

fn is_valid_height(height: &Option<String>) -> bool {
    height
        .as_ref()
        .map(|h| h.split_at(h.len() - 2))
        // Parse the number part
        .and_then(|(number, unit)| number.parse::<i32>().ok().map(|n| (n, unit)))
        // Convert the unit to min/max
        .and_then(|(number, unit)| match unit {
            "cm" => Some((number, 150, 193)),
            "in" => Some((number, 59, 76)),
            _ => None,
        })
        .filter(|(number, min, max)| min <= number && number <= max)
        .is_some()
}

fn is_valid_hair_color(color: &Option<String>) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    color.as_ref().map_or(false, |color| RE.is_match(&color))
}

fn is_valid_eye_color(color: &Option<String>) -> bool {
    color.as_ref().map_or(false, |color| match color.as_ref() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    })
}

fn is_valid_id(id: &Option<String>) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    id.as_ref().map_or(false, |id| RE.is_match(&id))
}

#[derive(Default, Debug)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn from_packed(packed: &str) -> Self {
        let mut passport = Passport::default();

        for raw_field in packed.split_ascii_whitespace() {
            let mut split = raw_field.splitn(2, ":");
            let key = split.next().expect(raw_field);
            let value = Some(split.next().expect(raw_field).to_owned());
            match key {
                "byr" => passport.birth_year = value,
                "iyr" => passport.issue_year = value,
                "eyr" => passport.expiration_year = value,
                "hgt" => passport.height = value,
                "hcl" => passport.hair_color = value,
                "ecl" => passport.eye_color = value,
                "pid" => passport.passport_id = value,
                "cid" => passport.country_id = value,
                _ => panic!("Unknown key {}", key),
            }
        }

        passport
    }

    fn is_valid_passport_pt1(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid_passport_pt2(&self) -> bool {
        is_valid_year(&self.birth_year, 1920, 2002)
            && is_valid_year(&self.issue_year, 2010, 2020)
            && is_valid_year(&self.expiration_year, 2020, 2030)
            && is_valid_height(&self.height)
            && is_valid_hair_color(&self.hair_color)
            && is_valid_eye_color(&self.eye_color)
            && is_valid_id(&self.passport_id)
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let num_valid = input
        .split("\n\n")
        .map(|i| Passport::from_packed(i))
        .filter(|p| p.is_valid_passport_pt2())
        .count();

    println!("{}", num_valid)
}

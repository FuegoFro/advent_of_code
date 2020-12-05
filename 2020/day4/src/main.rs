#[macro_use]
extern crate lazy_static;

use regex::Regex;

// enum PassportField {
//     BirthYear(Option<String>),
//     IssueYear(Option<String>),
//     ExpirationYear(Option<String>),
//     Height(Option<String>),
//     HairColor(Option<String>),
//     EyeColor(Option<String>),
//     PassportID(Option<String>),
//     CountryID(Option<String>),
// }

fn is_valid_year(year: &Option<String>, min: i32, max: i32) -> bool {
    if let Some(year) = year {
        if let Ok(parsed) = year.parse::<i32>() {
            return year.len() == 4 && min <= parsed && parsed <= max;
        }
    }

    false
}

fn is_valid_height(height: &Option<String>) -> bool {
    let height = if let Some(height) = height {
        height
    } else {
        return false;
    };

    let (number, unit) = height.split_at(height.len() - 2);
    let (min, max) = match unit {
        "cm" => (150, 193),
        "in" => (59, 76),
        _ => return false,
    };

    if let Ok(parsed) = number.parse() {
        min <= parsed && parsed <= max
    } else {
        false
    }
}

fn is_valid_hair_color(color: &Option<String>) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    color.as_ref().map_or(false, |color| RE.is_match(&color))
}

fn is_valid_eye_color(color: &Option<String>) -> bool {
    color.as_ref().map_or(false, |color| {
        match color.as_ref() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    })
}

fn is_valid_id(id: &Option<String>) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    id.as_ref().map_or(false, |id| RE.is_match(&id))
}

// impl PassportField {
//     fn from_packed(packed: &str) -> Self {
//         let mut split = packed.splitn(2, ":");
//         let key = split.next().expect(packed);
//         let value = Some(split.next().expect(packed).to_owned());
//
//         match key {
//             "byr" => Self::BirthYear(value),
//             "iyr" => Self::IssueYear(value),
//             "eyr" => Self::ExpirationYear(value),
//             "hgt" => Self::Height(value),
//             "hcl" => Self::HairColor(value),
//             "ecl" => Self::EyeColor(value),
//             "pid" => Self::PassportID(value),
//             "cid" => Self::CountryID(value),
//             _ => panic!("Unknown key {}", key),
//         }
//     }
//
//     fn is_present_or_ignored(&self) -> bool {
//         match self {
//             PassportField::CountryID(..) => true,
//             _ => self.0.is_some()
//         }
//     }
//
//     fn is_valid(&self) -> bool {
//         match self {
//             Self::BirthYear(Some(year)) => is_valid_year(year, 1920, 2002),
//             Self::IssueYear(Some(year)) => is_valid_year(year, 2010, 2020),
//             Self::ExpirationYear(Some(year)) => is_valid_year(year, 2020, 2030),
//             Self::Height(Some(height)) => is_valid_height(height),
//             Self::HairColor(Some(color)) => is_valid_hair_color(color),
//             Self::EyeColor(Some(color)) => is_valid_eye_color(color),
//             Self::PassportID(Some(id)) => is_valid_id(id),
//             Self::CountryID( .. ) => true,
//             _ => false,
//         }
//     }
// }

// #[derive(Default, Debug)]
// struct Passport {
//     birth_year: PassportField::BirthYear,
//     issue_year: PassportField::IssueYear,
//     expiration_year: PassportField::ExpirationYear,
//     height: PassportField::Height,
//     hair_color: PassportField::HairColor,
//     eye_color: PassportField::EyeColor,
//     passport_id: PassportField::PassportID,
//     country_id: PassportField::CountryID,
// }

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

    // fn all_fields(&self) -> impl Iterator<Item=&PassportField> {
    //     // TODO - Better way to make an iterator?
    //     vec![
    //         self.birth_year,
    //         self.issue_year,
    //         self.expiration_year,
    //         self.height,
    //         self.hair_color,
    //         self.eye_color,
    //         self.passport_id,
    //         self.country_id,
    //     ].iter()
    // }

    fn is_valid_passport_pt1(&self) -> bool {
        self.birth_year.is_some() &&
            self.issue_year.is_some() &&
            self.expiration_year.is_some() &&
            self.height.is_some() &&
            self.hair_color.is_some() &&
            self.eye_color.is_some() &&
            self.passport_id.is_some()
    }

    fn is_valid_passport_pt2(&self) -> bool {
        is_valid_year(&self.birth_year, 1920, 2002) &&
            is_valid_year(&self.issue_year, 2010, 2020) &&
            is_valid_year(&self.expiration_year, 2020, 2030) &&
            is_valid_height(&self.height) &&
            is_valid_hair_color(&self.hair_color) &&
            is_valid_eye_color(&self.eye_color) &&
            is_valid_id(&self.passport_id)
    }
}

fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let num_valid = input
        .split("\n\n")
        .map(|i| Passport::from_packed(i))
        .filter(|p| p.is_valid_passport_pt2())
        .count();

    println!("{}", num_valid)
}

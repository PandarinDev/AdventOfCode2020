use std::collections::HashMap;
use std::error::Error;
use regex::Regex;

#[derive(Debug)]
struct InputParsingError;

impl Error for InputParsingError {}

impl std::fmt::Display for InputParsingError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse input data.")
    }

}

trait Validator {

    fn key(&self) -> &'static str;
    fn validate(&self, entry: &str) -> bool;

}

struct BirthdayValidator;

impl Validator for BirthdayValidator {

    fn key(&self) -> &'static str {
        "byr"
    }

    fn validate(&self, entry: &str) -> bool {
        let birthday = match entry.parse::<i32>() {
            Ok(b) => b,
            _ => return false
        };
        birthday >= 1920 && birthday <= 2002
    }

}

struct IssueYearValidator;

impl Validator for IssueYearValidator {

    fn key(&self) -> &'static str {
        "iyr"
    }

    fn validate(&self, entry: &str) -> bool {
        let issue_year = match entry.parse::<i32>() {
            Ok(i) => i,
            _ => return false
        };
        issue_year >= 2010 && issue_year <= 2020
    }

}

struct ExpirationYearValidator;

impl Validator for ExpirationYearValidator {

    fn key(&self) -> &'static str {
        "eyr"
    }

    fn validate(&self, entry: &str) -> bool {
        let expiration_year = match entry.parse::<i32>() {
            Ok(e) => e,
            _ => return false
        };
        expiration_year >= 2020 && expiration_year <= 2030
    }

}

struct HeightValidator;

impl Validator for HeightValidator {

    fn key(&self) -> &'static str {
        "hgt"
    }

    fn validate(&self, entry: &str) -> bool {
        let unit = &entry[entry.len() - 2..];
        let height = match entry[0..entry.len() - 2].parse::<i32>() {
            Ok(h) => h,
            _ => return false
        };
        match unit {
            "cm" => height >= 150 && height <= 193,
            "in" => height >= 59 && height <= 76,
            _ => false
        }
    }

}

struct HairColorValidator {
    pattern: Regex
}

impl HairColorValidator {

    fn new() -> Self {
        HairColorValidator { pattern: Regex::new(r"^#[0-9a-f]{6}$").unwrap() }
    }

}

impl Validator for HairColorValidator {

    fn key(&self) -> &'static str {
        "hcl"
    }

    fn validate(&self, entry: &str) -> bool {
        self.pattern.is_match(entry)
    }

}

struct EyeColorValidator;

impl Validator for EyeColorValidator {

    fn key(&self) -> &'static str {
        "ecl"
    }

    fn validate(&self, entry: &str) -> bool {
        match entry {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false
        }
    }

}

struct PassportIdValidator {
    pattern: Regex
}

impl PassportIdValidator {

    fn new() -> Self {
        PassportIdValidator { pattern: Regex::new(r"^\d{9}$").unwrap() }
    }

}

impl Validator for PassportIdValidator {

    fn key(&self) -> &'static str {
        "pid"
    }

    fn validate(&self, entry: &str) -> bool {
        self.pattern.is_match(entry)
    }

}

struct Passport {
    entries: HashMap<String, String>
}

struct ValidatorChain {
    validators: Vec<Box<dyn Validator>>
}

impl ValidatorChain {

    fn new() -> Self {
        let validators: Vec<Box<dyn Validator>> = vec![
            Box::new(BirthdayValidator),
            Box::new(IssueYearValidator),
            Box::new(ExpirationYearValidator),
            Box::new(HeightValidator),
            Box::new(HairColorValidator::new()),
            Box::new(EyeColorValidator),
            Box::new(PassportIdValidator::new())
        ];
        ValidatorChain { validators }
    }

    fn is_valid(&self, passport: &Passport) -> bool {
        for validator in &self.validators {
            let key = validator.key();
            let value = passport.entries.get(key);
            if value.is_none() || !validator.validate(value.unwrap()) {
                return false;
            }
        }
        true
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let passports = parse_passports()?;
    let validator_chain = ValidatorChain::new();
    let valid_passport_count = passports.iter().filter(|p| validator_chain.is_valid(p)).count();
    println!("{}", valid_passport_count);
    Ok(())
}

fn parse_passports() -> Result<Vec<Passport>, Box<dyn Error>> {
    let file_path = std::env::current_exe()?.join("../../../resources/day4.txt");
    let input = std::fs::read_to_string(file_path)?;
    let lines: Vec<_> = input.lines().collect();
    let mut passports: Vec<Passport> = vec![];
    let mut passport_entries = HashMap::new();
    for line in lines {
        if line.is_empty() {
            passports.push(Passport { entries: passport_entries });
            passport_entries = HashMap::new();
            continue;
        }
        let entries: Vec<_> = line.split(" ")
            .map(|e| e.split(":").collect::<Vec<_>>())
            .map(|e| (e[0].to_string(), e[1].to_string()))
            .collect();
        for entry in entries {
            passport_entries.insert(entry.0, entry.1);
        }
    }
    // Note: str.lines() skips the last empty line so we need to push the last passport
    passports.push(Passport { entries: passport_entries });
    Ok(passports)
}


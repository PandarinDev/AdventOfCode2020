use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
struct InputParsingError;

impl Error for InputParsingError {}

impl std::fmt::Display for InputParsingError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse input data.")
    }

}

const REQUIRED_KEYS: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid"
];

struct Passport {
    entries: HashMap<String, String>
}

impl Passport {

    fn is_valid(&self) -> bool {
        REQUIRED_KEYS.iter().all(|k| self.entries.contains_key(*k))
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let passports = parse_passports()?;
    let valid_passport_count = passports.iter().filter(|p| p.is_valid()).count();
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


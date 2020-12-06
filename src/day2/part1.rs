use std::error::Error;
use regex::Regex;

struct PasswordPolicy {
    min_occurences: i32,
    max_occurences: i32,
    character: char
}

struct PasswordEntry {
    policy: PasswordPolicy,
    password: String
}

impl PasswordEntry {

    fn is_valid(&self) -> bool {
        let occurences = self.password.chars()
            .filter(|c| *c == self.policy.character)
            .count() as i32;
        occurences >= self.policy.min_occurences && occurences <= self.policy.max_occurences
    }

}

#[derive(Debug)]
struct LineParsingError;

impl Error for LineParsingError {}

impl std::fmt::Display for LineParsingError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to parse input line.")
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let entries = parse_input()?;
    let valid_entry_count = entries.iter()
        .filter(|e| e.is_valid())
        .count();
    println!("{}", valid_entry_count);
    Ok(())
}

fn parse_input() -> Result<Vec<PasswordEntry>, Box<dyn Error>> {
    // Example input line: "12-14 s: ssskrssssssfsxpsqsp"
    let line_pattern = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let file_path = std::env::current_exe()?.join("../../../resources/day2.txt");
    let input = std::fs::read_to_string(file_path)?;
    let result: Result<Vec<_>, _> = input.lines()
        .map(|line| parse_line(&line_pattern, &line))
        .collect();
    result.map_err(|err| err.into())
}

fn parse_line(pattern: &Regex, line: &str) -> Result<PasswordEntry, Box<dyn Error>> {
    let captures = match pattern.captures(line) {
        Some(captures) => captures,
        _ => return Err(LineParsingError.into())
    };
    let get_str_match = |i: usize| captures.get(i).unwrap().as_str();
    let get_int_match = |i: usize| get_str_match(i).parse::<i32>().unwrap();
    let min_occurences = get_int_match(1);
    let max_occurences = get_int_match(2);
    let character = get_str_match(3).chars().next().unwrap();
    let password = get_str_match(4).to_string();
    let policy = PasswordPolicy { min_occurences, max_occurences, character };
    Ok(PasswordEntry { policy, password })
}

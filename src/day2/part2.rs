use std::error::Error;
use regex::Regex;

struct PasswordPolicy {
    first_position: usize,
    second_position: usize,
    character: char
}

struct PasswordEntry {
    policy: PasswordPolicy,
    password: String
}

impl PasswordEntry {

    fn is_valid(&self) -> bool {
        let chars: Vec<_> = self.password.chars().collect();
        // Note: Positions in policies are indexed from 1
        let first_char = chars[self.policy.first_position - 1];
        let second_char = chars[self.policy.second_position - 1];
        let first_matches = first_char == self.policy.character;
        let second_matches = second_char == self.policy.character;
        // There must be exactly one match for the password to be valid
        (first_matches && !second_matches) || (!first_matches && second_matches)
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
    let get_usize_match = |i: usize| get_str_match(i).parse::<usize>().unwrap();
    let first_position = get_usize_match(1);
    let second_position = get_usize_match(2);
    let character = get_str_match(3).chars().next().unwrap();
    let password = get_str_match(4).to_string();
    let policy = PasswordPolicy { first_position, second_position, character };
    Ok(PasswordEntry { policy, password })
}

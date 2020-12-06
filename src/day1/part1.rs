use std::error::Error;

const DESIRED_SUM: i32 = 2020;

type Pair = (i32, i32);

#[derive(Debug)]
struct PairNotFoundError;

impl Error for PairNotFoundError {}

impl std::fmt::Display for PairNotFoundError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pair was not found.")
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let entries = parse_input()?;
    let pair = find_pair_summing_to(&entries, DESIRED_SUM)?;
    println!("{}", pair.0 * pair.1);
    Ok(())
}

fn parse_input() -> Result<Vec<i32>, Box<dyn Error>> {
    let file_path = std::env::current_exe()?.join("../../../resources/day1.txt");
    let input = std::fs::read_to_string(file_path)?;
    let result: Result<Vec<_>, _> = input.lines()
        .map(| line | line.parse::<i32>())
        .collect();
    result.map_err(|err| err.into())
}

fn find_pair_summing_to(entries: &Vec<i32>, desired_sum: i32) -> Result<Pair, PairNotFoundError> {
    for first_entry in entries {
        for second_entry in entries {
            if first_entry + second_entry == desired_sum {
                return Ok((*first_entry, *second_entry));
            }
        }
    }
    Err(PairNotFoundError)
}

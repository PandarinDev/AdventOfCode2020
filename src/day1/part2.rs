use std::error::Error;

const DESIRED_SUM: i32 = 2020;

type Triplet = (i32, i32, i32);

#[derive(Debug)]
struct TripletNotFoundError;

impl Error for TripletNotFoundError {}

impl std::fmt::Display for TripletNotFoundError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Triplet was not found.")
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let entries = parse_input()?;
    let triplet = find_triplet_summing_to(&entries, DESIRED_SUM)?;
    println!("{}", triplet.0 * triplet.1 * triplet.2);
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

fn find_triplet_summing_to(entries: &Vec<i32>, desired_sum: i32) -> Result<Triplet, TripletNotFoundError> {
    for first_entry in entries {
        for second_entry in entries {
            for third_entry in entries {
                if first_entry + second_entry + third_entry == desired_sum {
                    return Ok((*first_entry, *second_entry, *third_entry));
                }
            }
        }
    }
    Err(TripletNotFoundError)
}

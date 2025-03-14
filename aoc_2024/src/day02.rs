use std::error::Error;
use csv;

fn load_input() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_path("res/input02.csv")?;

    let mut rows: Vec<Vec<i32>> = Vec::new();

    for result in reader.records() {
        let record = result?;
        let row: Vec<i32>= record
            .iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        rows.push(row);
    }

    return Ok(rows);
}

pub fn run() {
    // part01
    let _input_vec = load_input().unwrap_or_else(|error| {
        panic!("Problem loading the file: {error:?}");
    });
} 

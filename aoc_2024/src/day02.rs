use std::error::Error;
use csv;

fn load_input() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .flexible(true)
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

fn is_safe(readings: &Vec<i32>) -> bool {
    if readings[0] > readings[1] {
        for window in & mut readings.windows(2) {
            let diff = window[0] - window[1];
            if diff < 1 || diff > 3 {
                return false;
            }
        }
    }
    else if readings[0] < readings[1] {
        for window in & mut readings.windows(2) {
            let diff = window[1] - window[0];
            if diff < 1 || diff > 3 {
                return false;
            }
        }
    }
    else {
        return false;
    }
    return true;
}

fn is_safe_buffer(readings: &Vec<i32>) -> bool {
    if is_safe(readings) {
        return true
    }
    for i in 0..readings.len() {
        let mut cloned_readings = readings.clone();
        cloned_readings.remove(i);
        if is_safe(&cloned_readings) {
            return true;
        }
    }
    return false;
}

pub fn run() {
    // part01
    let input_vec = load_input().unwrap_or_else(|error| {
        panic!("Problem loading the file: {error:?}");
    });

    let mut safety_score = 0;
    for readings in &input_vec[..] {
        if is_safe(&readings) {
            safety_score += 1;
        }
    }
    println!("day02.01: {}", safety_score);

    // part02
    let mut safety_score: i32 = 0;
    for readings in &input_vec[..] {
        if is_safe_buffer(&readings) {
            safety_score += 1;
        }
    }
    println!("day02.02: {}", safety_score);
} 

#[cfg(test)]
mod tests02 {
    use super::*;

    #[test]
    fn safety_checking_readings_is_correct() {
        let vec1 = vec![7, 6, 4, 2, 1];
        let vec2 = vec![1, 2, 7, 8, 9];
        let vec3 = vec![9, 7, 6, 2, 1];
        let vec4 = vec![1, 3, 2, 4, 5];
        let vec5 = vec![8, 6, 4, 4, 1];
        let vec6 = vec![1, 3, 6, 7, 9];

        assert!(is_safe(&vec1));
        assert!(!is_safe(&vec2));
        assert!(!is_safe(&vec3));
        assert!(!is_safe(&vec4));
        assert!(!is_safe(&vec5));
        assert!(is_safe(&vec6));
    }

    #[test]
    fn buffer_safety_checking_readings_is_correct() {
        let vec1 = vec![7, 6, 4, 2, 1];
        let vec2 = vec![1, 2, 7, 8, 9];
        let vec3 = vec![9, 7, 6, 2, 1];
        let vec4 = vec![1, 3, 2, 4, 5];
        let vec5 = vec![8, 6, 4, 4, 1];
        let vec6 = vec![1, 3, 6, 7, 9];

        assert!(is_safe_buffer(&vec1));
        assert!(!is_safe_buffer(&vec2));
        assert!(!is_safe_buffer(&vec3));
        assert!(is_safe_buffer(&vec4));
        assert!(is_safe_buffer(&vec5));
        assert!(is_safe_buffer(&vec6));
    }
}


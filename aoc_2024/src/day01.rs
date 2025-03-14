use std::error::Error;
use std::fs;
use csv;

fn load_input() -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_path("res/input01.csv")?;

    let mut left_values = Vec::new();
    let mut right_values = Vec::new();

    for result in reader.records() {
        let record = result?;
        left_values.push(record[0].parse::<i32>()?);
        right_values.push(record[1].parse::<i32>()?);
    }

    return Ok((left_values, right_values));
}

fn change_to_supported_delim() {
    let file_path = "res/input01.csv";
    let content = fs::read_to_string(file_path)
        .expect("Failed to read the file");

    let new_content = content.replace("   ", " ");

    fs::write(file_path, new_content)
        .expect("Failed to write to the file");
}

fn compare_left_and_right(vectors: &mut(Vec<i32>, Vec<i32>)) -> Vec<i32> {
    let mut diff_values = Vec::new();
    vectors.0.sort();
    vectors.1.sort();
    for (pos, e) in vectors.0.iter().enumerate() {
        let diff = vectors.1[pos] - e; 
        diff_values.push(diff.abs());
    }
    return diff_values;
}

fn find_similarity(vectors: &mut(Vec<i32>, Vec<i32>)) -> i32 {
    // iterate over left vector
    let mut similarity_score: i32 = 0;

    for val in vectors.0.iter() {
        let right_repeat = vectors.1.iter().filter(|&n| *n == *val).count() as i32;
        let sim_score = right_repeat * val;
        similarity_score += sim_score;
    }
    
    return similarity_score;
}

pub fn run() {
    change_to_supported_delim();
    
    // part01
    let mut vec_pair = load_input().unwrap_or_else(|error| {
        panic!("Problem loading the file: {error:?}");
    });
    let diff_values = compare_left_and_right(&mut vec_pair);
    let sum: i32 = diff_values.iter().sum();
    println!("Day01.01 {}", sum);

    // part02
    let similarity_score = find_similarity(&mut vec_pair);
    println!("Day01.02 {}", similarity_score);
}

//write some tests!
#[cfg(test)]
mod tests01 {
    use super::*;

    #[test]
    fn comparing_vectors_is_correct() {
        let vec1 = vec![3, 4, 5, 5, 1];
        let vec2 = vec![6, 3, 1, 2, 4];
        let mut vec_pair = (vec1, vec2);
        let diff_values = compare_left_and_right(&mut vec_pair);

        assert_eq!(diff_values, vec![0, 1, 1, 1, 1]);
    }

    #[test]
    fn similarity_score_is_correct() {
        let vec1 = vec![3, 4, 2, 1, 3, 3];
        let vec2 = vec![4, 3, 5, 3, 9, 3];
        let mut vec_pair = (vec1, vec2);
        let sim_score = find_similarity(&mut vec_pair);

        assert_eq!(sim_score, 31);
    }
}

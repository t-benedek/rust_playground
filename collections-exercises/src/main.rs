use std::collections::HashMap;

fn main() {
    // GIVEN list of integers
    let mut given_list = vec!{1,88,11,3,13,17,88,88,7,5,13};
    // let mut given_list = vec!{1,2,3};

    println!("\nInput Vector : {:?}", given_list);

    let median = integer_list_median(&mut given_list);
    println!("Median : {}", median);

    let mode = integer_list_mode(&mut given_list);
    println!("Mode : {}\n", mode);
}

// find the median. This is the number with the middle position of the sorted vector 
fn integer_list_median(given_list: &mut Vec<i32>) -> i32 {
    // Sort vector
    given_list.sort();

    let len = given_list.len();
    match len  {
        0 => return 0,
        1 => return given_list[0],
        _ => {
            // if the length is even then take the lower position, if it is odd then take the middle as expected
            let median_pos = (len-1) / 2;
            given_list.get(median_pos).copied().unwrap_or(0)        
        }
    }
}

// find the mode. This is value that most often is found in the vector
fn integer_list_mode(given_list: &mut Vec<i32>) -> i32 {
    let mut map : HashMap<&i32, i32> = HashMap::new();
    let mut most_often_int = 0;
    let mut highest_count = 0;

    for vec_int in given_list {
        let count = map.entry(vec_int).or_insert(0);
        *count += 1;
        if *count > highest_count {
            highest_count = *count;
            most_often_int = *vec_int;
        }
            
    }
    most_often_int
}


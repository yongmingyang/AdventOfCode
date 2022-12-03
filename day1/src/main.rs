use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
    let mut highest_var = 0;
    let mut second_highest_var = 0;
    let mut third_highest_var = 0;
    let mut current_var = 0;
    for s in contents.lines() {
        if s.is_empty() {
            if current_var > highest_var {
                third_highest_var = second_highest_var;
                second_highest_var = highest_var;
                highest_var = current_var;
            } else if current_var > second_highest_var {
                third_highest_var = second_highest_var;
                second_highest_var = current_var;
            } else if current_var > third_highest_var {
                third_highest_var = current_var;
            }
            current_var = 0;
        } else {
            let int_value: i32 = s.parse().unwrap();
            current_var += int_value;

        }
    }
    let total_var = highest_var + second_highest_var + third_highest_var;
    println!("{}", total_var);
}

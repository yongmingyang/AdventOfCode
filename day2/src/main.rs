use std::{fs, collections::HashMap};


// PART 1
// fn main() {

//     let mut pointsMapper: HashMap<char, HashMap<char, i32>> = HashMap::new();
//     let mut A_mapper: HashMap<char, i32> = HashMap::new();
//     let mut B_mapper: HashMap<char, i32> = HashMap::new();
//     let mut C_mapper: HashMap<char, i32> = HashMap::new();

//     A_mapper.insert('X', 4); // 1 + 3 DRAW
//     A_mapper.insert('Y', 8); // 2 + 6 WIN
//     A_mapper.insert('Z', 3); // 3 + 0 LOSE

//     B_mapper.insert('X', 1); // 1 + 0 LOSE
//     B_mapper.insert('Y', 5); // 2 + 3 DRAW
//     B_mapper.insert('Z', 9); // 3 + 6 WIN

//     C_mapper.insert('X', 7); // 1 + 6 WIN
//     C_mapper.insert('Y', 2); // 2 + 0 LOSE
//     C_mapper.insert('Z', 6); // 3 + 3 DRAW

//     pointsMapper.insert('A', A_mapper);
//     pointsMapper.insert('B', B_mapper);
//     pointsMapper.insert('C', C_mapper);

//     let contents = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
//     let mut total_points = 0;
//     for s in contents.lines() {
//         let player = s.chars().nth(0).unwrap();
//         let opponent = s.chars().nth(2).unwrap();
//         let points = pointsMapper.get(&player).unwrap().get(&opponent).unwrap();
//         total_points += points;
//     }
//     println!("Total points: {}", total_points);
// }

fn main() {

    let mut pointsMapper: HashMap<char, HashMap<char, i32>> = HashMap::new();
    let mut A_mapper: HashMap<char, i32> = HashMap::new();
    let mut B_mapper: HashMap<char, i32> = HashMap::new();
    let mut C_mapper: HashMap<char, i32> = HashMap::new();

    // X = LOSE
    // Y = DRAW
    // Z = WIN

    // A = ROCK
    // B = PAPER
    // C = SCISSORS

    A_mapper.insert('X', 3); // 3 + 0
    A_mapper.insert('Y', 4); // 1 + 3
    A_mapper.insert('Z', 8); // 6 + 2 

    B_mapper.insert('X', 1); // 1 + 0 LOSE
    B_mapper.insert('Y', 5); // 2 + 3 DRAW
    B_mapper.insert('Z', 9); // 3 + 6 WIN

    C_mapper.insert('X', 2); // 2 + 0 LOSE
    C_mapper.insert('Y', 6); // 3 + 0 DRAW
    C_mapper.insert('Z', 7); // 1 + 6 WIN

    pointsMapper.insert('A', A_mapper);
    pointsMapper.insert('B', B_mapper);
    pointsMapper.insert('C', C_mapper);

    let contents = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
    let mut total_points = 0;
    for s in contents.lines() {
        let player = s.chars().nth(0).unwrap();
        let opponent = s.chars().nth(2).unwrap();
        let points = pointsMapper.get(&player).unwrap().get(&opponent).unwrap();
        total_points += points;
    }
    println!("Total points: {}", total_points);
}

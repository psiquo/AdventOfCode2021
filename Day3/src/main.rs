use std::fs;
use foreach::ForEach;
fn main() {
    part_1("./resources/input_1");
    part_2("./resources/input_1");
}

fn part_1(filename : &str){
    let contents = fs::read_to_string(filename).expect("Cannot read file");

    let char_matrix : Vec<Vec<char>> = 
        contents.trim().split("\n").map(|x| x.chars().collect()).collect();
    
    let (mut gamma_rate,mut epsilon_rate) : (String,String) = 
        (String::default(),String::default());
    
    for bnum in 0..char_matrix.get(0).unwrap().len() {
        
        if get_most_common(&char_matrix,bnum) == '1' {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }

    let gamma_rate = u32::from_str_radix(&gamma_rate,2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate,2).unwrap();

    println!("Gamma rate: {}\nEpsilon rate:{}\nPower Consumption: {}",
                        gamma_rate,epsilon_rate,gamma_rate*epsilon_rate);
}

fn part_2(filename : &str) {
    let contents = fs::read_to_string(filename).expect("Cannot read file");

    let mut char_matrix_tops : Vec<Vec<char>> = 
        contents.trim().split("\n").map(|x| x.chars().collect()).collect();
    let mut char_matrix_leasts = char_matrix_tops.clone();
    
    let (mut oxygen_rat, mut co2_rat) = (String::default(),String::default());

    for bnum in 0..char_matrix_tops.get(0).unwrap().len() {
        let most_common = if get_most_common(&char_matrix_tops, bnum) == '0' {'0'} else {'1'}; 
        let least_common = if get_most_common(&char_matrix_leasts, bnum) == '0' {'1'} else {'0'};
        
        char_matrix_tops = char_matrix_tops.into_iter()
            .filter(|x| x.get(bnum).unwrap().clone() == most_common)
            .collect::<Vec<Vec<char>>>();

        char_matrix_leasts = char_matrix_leasts.into_iter()
            .filter(|x| x.get(bnum).unwrap().clone() == least_common)
            .collect::<Vec<Vec<char>>>();

        if char_matrix_tops.len() == 1 && oxygen_rat.len() == 0 {
            char_matrix_tops.get(0).unwrap().iter().foreach(|x,_| oxygen_rat.push(*x) )
        }

        if char_matrix_leasts.len() == 1 && co2_rat.len() == 0 {
            char_matrix_leasts.get(0).unwrap().iter().foreach(|x,_| co2_rat.push(*x) )
        }
    }
    let oxygen_rat = u32::from_str_radix(&oxygen_rat,2).unwrap();
    let co2_rat = u32::from_str_radix(&co2_rat,2).unwrap();

    println!("{}:{}:{}",oxygen_rat,co2_rat,oxygen_rat*co2_rat);

}

fn get_most_common(matrix : &Vec<Vec<char>>, index : usize) -> char {
    let one_count = 
            matrix.iter()
                .map(|v| v.get(index).unwrap())
                .fold(0,|sum , x| sum + x.to_digit(2).unwrap());

    let len = matrix.len();

    if len - one_count as usize > one_count as usize {
        '0'
    } else if len - one_count as usize == one_count as usize {
        '='
    } else {
        '1'
    }
}


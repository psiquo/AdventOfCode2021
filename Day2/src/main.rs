use std::fs;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() == 1 || args[1].eq("1") {
        part_1("./resources/input_1");
    } else{
        part_2("./resources/input_1");
    }
}

fn part_1(filename : &str) {
    let contents = fs::read_to_string(filename)
                    .expect("Cannot read file");

    let mut horiz : i32 = 0;
    let mut depth : i32 = 0;

    for command in contents.trim().split("\n") {
        let command = &command.split(" ").collect::<Vec<&str>>()[0..2];
        let val : i32 = command[1].parse().expect("Not a number");

        match command[0] {
            "up"   => depth -= val,
            "down" => depth += val,
            "forward" => horiz += val,
            _ => println!("Invalid command: {}",command[0])
        }
    }

    println!("{}",depth * horiz);
}


fn part_2(filename : &str) {
    let contents = fs::read_to_string(filename)
                    .expect("Cannot read file");

    let mut horiz : i32 = 0;
    let mut depth : i32 = 0;
    let mut aim : i32 = 0;

    for command in contents.trim().split("\n") {
        let command = &command.split(" ").collect::<Vec<&str>>()[0..2];
        let val : i32 = command[1].parse().expect("Not a number");

        match command[0] {
            "up"   => aim -= val,
            "down" => aim += val,
            "forward" => {
                horiz += val;
                depth += aim * val;
            },
            _ => println!("Invalid command: {}",command[0])
        }
    }

    println!("{}",depth * horiz);
}
